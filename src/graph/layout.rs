use crate::graph::{NodeId, Workflow};
use petgraph::algo::toposort;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;

pub struct DagLayout {
    pub layer_spacing: f32,
    pub node_spacing: f32,
}

impl Default for DagLayout {
    fn default() -> Self {
        Self {
            layer_spacing: 140.0,
            node_spacing: 60.0,
        }
    }
}

const NODE_WIDTH: f32 = 220.0;
const NODE_HEIGHT: f32 = 68.0;
const LEFT_PADDING: f32 = 120.0;
const TOP_PADDING: f32 = 80.0;

impl DagLayout {
    #[allow(
        clippy::cast_precision_loss,
        clippy::too_many_lines,
        clippy::items_after_statements
    )]
    pub fn apply(&self, workflow: &mut Workflow) {
        if workflow.nodes.is_empty() {
            return;
        }

        let mut graph = Graph::<NodeId, ()>::new();
        let mut index_map = HashMap::new();
        let mut reverse_map = HashMap::new();
        let mut node_order = HashMap::new();

        // Add nodes to petgraph
        for (order, node) in workflow.nodes.iter().enumerate() {
            let idx = graph.add_node(node.id);
            index_map.insert(node.id, idx);
            reverse_map.insert(idx, node.id);
            node_order.insert(idx, order);
        }

        // Add edges
        for conn in &workflow.connections {
            if let (Some(&src), Some(&tgt)) =
                (index_map.get(&conn.source), index_map.get(&conn.target))
            {
                graph.add_edge(src, tgt, ());
            }
        }

        // 1. Cycle Removal / Toposort
        let Ok(sorted_indices) = toposort(&graph, None) else {
            // If cyclic, try to layout what we can or return
            return;
        };

        // 2. Layer Assignment (Longest Path Layering)
        let mut layers: HashMap<NodeIndex, usize> = HashMap::new();
        for &node_idx in &sorted_indices {
            let layer = graph
                .neighbors_directed(node_idx, petgraph::Direction::Incoming)
                .filter_map(|parent| layers.get(&parent).map(|&l| l + 1))
                .max()
                .unwrap_or(0);
            layers.insert(node_idx, layer);
        }

        // Group by layer in deterministic topological order
        let mut nodes_by_layer: Vec<Vec<NodeIndex>> = Vec::new();
        for &node_idx in &sorted_indices {
            let layer = layers.get(&node_idx).map_or(0, |value| *value);
            while nodes_by_layer.len() <= layer {
                nodes_by_layer.push(Vec::new());
            }
            nodes_by_layer[layer].push(node_idx);
        }

        // 3. Crossing minimization (barycenter sweep)
        for _ in 0..4 {
            for layer_idx in 1..nodes_by_layer.len() {
                let mut barycenters: Vec<(NodeIndex, f32)> = nodes_by_layer[layer_idx]
                    .iter()
                    .map(|&node| {
                        let (sum, count) = graph
                            .neighbors_directed(node, petgraph::Direction::Incoming)
                            .filter_map(|parent| {
                                nodes_by_layer[layer_idx - 1]
                                    .iter()
                                    .position(|&n| n == parent)
                                    .map(|pos| pos as f32)
                            })
                            .fold((0.0, 0.0), |(s, c), pos| (s + pos, c + 1.0));

                        let barycenter = if count > 0.0 { sum / count } else { 0.0 };
                        (node, barycenter)
                    })
                    .collect();

                barycenters.sort_by(|a, b| {
                    let order_a = node_order.get(&a.0).map_or(usize::MAX, |value| *value);
                    let order_b = node_order.get(&b.0).map_or(usize::MAX, |value| *value);
                    a.1.total_cmp(&b.1).then_with(|| order_a.cmp(&order_b))
                });
                nodes_by_layer[layer_idx] = barycenters.into_iter().map(|(n, _)| n).collect();
            }
        }

        // 4. Coordinate assignment (Dagre-like layered layout)
        let mut x_by_index: HashMap<NodeIndex, f32> = HashMap::new();
        let mut max_layer_width = 0.0_f32;

        for (layer, nodes) in nodes_by_layer.iter().enumerate() {
            let mut placed_x: Vec<f32> = Vec::new();

            for node_idx in nodes {
                let parent_positions = graph
                    .neighbors_directed(*node_idx, petgraph::Direction::Incoming)
                    .filter_map(|parent| x_by_index.get(&parent).copied())
                    .collect::<Vec<_>>();

                let preferred_x = if parent_positions.is_empty() {
                    0.0
                } else {
                    parent_positions.iter().sum::<f32>() / (parent_positions.len() as f32)
                };

                let x = placed_x.last().map_or(preferred_x, |prev| {
                    preferred_x.max(*prev + NODE_WIDTH + self.node_spacing)
                });
                placed_x.push(x);
                x_by_index.insert(*node_idx, x);
            }

            if let (Some(first), Some(last)) = (placed_x.first(), placed_x.last()) {
                let layer_width = (last - first + NODE_WIDTH).max(0.0);
                max_layer_width = max_layer_width.max(layer_width);
            }

            let y = (layer as f32) * (NODE_HEIGHT + self.layer_spacing);
            for node_idx in nodes {
                if let Some(node_id) = reverse_map.get(node_idx) {
                    if let Some(node) = workflow.nodes.iter_mut().find(|n| n.id == *node_id) {
                        node.y = y;
                    }
                }
            }
        }

        for nodes in &nodes_by_layer {
            let layer_positions = nodes
                .iter()
                .filter_map(|idx| x_by_index.get(idx).copied())
                .collect::<Vec<_>>();
            if layer_positions.is_empty() {
                continue;
            }

            let first = layer_positions.first().map_or(0.0, |v| *v);
            let last = layer_positions.last().map_or(0.0, |v| *v);
            let layer_width = (last - first + NODE_WIDTH).max(0.0);
            let layer_offset = (max_layer_width - layer_width) / 2.0;

            for node_idx in nodes {
                if let Some(node_id) = reverse_map.get(node_idx) {
                    if let Some(node) = workflow.nodes.iter_mut().find(|n| n.id == *node_id) {
                        let x = x_by_index.get(node_idx).map_or(0.0, |value| *value);
                        node.x = x + layer_offset;
                    }
                }
            }
        }

        let min_x = workflow
            .nodes
            .iter()
            .map(|node| node.x)
            .reduce(f32::min)
            .map_or(0.0, |value| value);
        let min_y = workflow
            .nodes
            .iter()
            .map(|node| node.y)
            .reduce(f32::min)
            .map_or(0.0, |value| value);

        for node in &mut workflow.nodes {
            node.x = node.x - min_x + LEFT_PADDING;
            node.y = node.y - min_y + TOP_PADDING;
        }
    }
}
