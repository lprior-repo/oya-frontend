use oya_frontend::graph::workflow_node::WorkflowNode;
use oya_frontend::graph::{Connection, Node, NodeId};
use std::collections::HashMap;

use crate::ui::editor_interactions::{NODE_HEIGHT, NODE_WIDTH};
use crate::ui::parallel_group_overlay::{AggregateStatus, BoundingBox, ParallelGroup};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub const BEND_CLAMP: f32 = 200.0;

#[derive(Clone, Copy, PartialEq)]
pub struct EdgeAnchor {
    pub from: Position,
    pub to: Position,
}

#[derive(Clone)]
pub struct DragState {
    pub edge_id: String,
    pub start_page_y: f32,
    pub start_bend: f32,
}

pub fn get_source_point(node: &Node) -> Position {
    Position {
        x: node.x + NODE_WIDTH,
        y: node.y + NODE_HEIGHT / 2.0,
    }
}

pub fn get_target_point(node: &Node) -> Position {
    Position {
        x: node.x,
        y: node.y + NODE_HEIGHT / 2.0,
    }
}

pub fn create_smooth_step_path(from: Position, to: Position, bend_y: f32) -> (String, Position) {
    let mid_y = f32::midpoint(from.y, to.y) + bend_y.clamp(-BEND_CLAMP, BEND_CLAMP);
    let radius: f32 = 8.0;

    let dx = to.x - from.x;
    let dy = to.y - from.y;

    if dx.abs() < 2.0 || !dx.is_finite() || !dy.is_finite() {
        return (
            format!("M {} {} L {} {}", from.x, from.y, to.x, to.y),
            Position {
                x: f32::midpoint(from.x, to.x),
                y: mid_y,
            },
        );
    }

    let sign_x = if dx > 0.0 { 1.0 } else { -1.0 };
    let r = radius.min(dx.abs() / 2.0).min(dy.abs() / 4.0);

    (
        format!(
            "M {fx} {fy} L {fx} {my_r} Q {fx} {my} {fx_r} {my} L {tx_r} {my} Q {tx} {my} {tx} {my_r2} L {tx} {ty}",
            fx = from.x,
            fy = from.y,
            my = mid_y,
            my_r = mid_y - r,
            my_r2 = mid_y + r,
            fx_r = from.x + sign_x * r,
            tx_r = to.x - sign_x * r,
            tx = to.x,
            ty = to.y
        ),
        Position {
            x: f32::midpoint(from.x, to.x),
            y: mid_y,
        },
    )
}

#[allow(dead_code)]
pub fn build_node_lookup(nodes: &[Node]) -> HashMap<NodeId, Node> {
    nodes.iter().map(|node| (node.id, node.clone())).collect()
}

#[allow(dead_code)]
pub fn resolve_edge_anchors(
    edges: &[Connection],
    node_by_id: &HashMap<NodeId, Node>,
) -> HashMap<String, EdgeAnchor> {
    edges
        .iter()
        .filter_map(|edge| {
            let source = node_by_id.get(&edge.source)?;
            let target = node_by_id.get(&edge.target)?;
            let from = get_source_point(source);
            let to = get_target_point(target);
            Some((edge.id.to_string(), EdgeAnchor { from, to }))
        })
        .collect()
}

pub fn resolve_edge_anchors_with_parallel(
    edges: &[Connection],
    node_by_id: &HashMap<NodeId, Node>,
    parallel_groups: &[ParallelGroup],
) -> HashMap<String, EdgeAnchor> {
    edges
        .iter()
        .filter_map(|edge| {
            let source = node_by_id.get(&edge.source)?;
            let target = node_by_id.get(&edge.target)?;
            let from = get_source_point(source);
            let to = get_target_point(target);

            let group = parallel_groups.iter().find(|g| {
                g.parallel_node_id == edge.source && g.branch_node_ids.contains(&edge.target)
            });

            let adjusted_to = group.map_or(to, |g| {
                let branch_nodes: Vec<Node> = g
                    .branch_node_ids
                    .iter()
                    .filter_map(|id| node_by_id.get(id).cloned())
                    .collect();
                let offset = calculate_parallel_offset(&edge.target, &branch_nodes, NODE_HEIGHT);
                Position {
                    x: to.x,
                    y: to.y + offset,
                }
            });

            Some((
                edge.id.to_string(),
                EdgeAnchor {
                    from,
                    to: adjusted_to,
                },
            ))
        })
        .collect()
}

#[allow(clippy::cast_precision_loss)]
pub fn calculate_parallel_offset(target_id: &NodeId, targets: &[Node], node_height: f32) -> f32 {
    let mut sorted: Vec<_> = targets.iter().enumerate().collect();
    sorted.sort_by_key(|a| a.1.id.0);

    let idx = sorted
        .iter()
        .position(|(_, n)| n.id == *target_id)
        .unwrap_or(0);

    let spacing = node_height / 2.5;
    (idx as f32 - (sorted.len() as f32 - 1.0) / 2.0) * spacing
}

pub fn find_parallel_branches(
    node_by_id: &HashMap<NodeId, Node>,
    connections: &[Connection],
) -> Vec<ParallelGroup> {
    // Only consider explicit WorkflowNode::Parallel nodes as sources for parallel groups
    let parallel_node_ids: Vec<NodeId> = node_by_id
        .values()
        .filter(|node| matches!(node.node, WorkflowNode::Parallel(_)))
        .map(|node| node.id)
        .collect();

    let mut source_targets: HashMap<NodeId, std::collections::HashSet<NodeId>> = HashMap::new();

    for connection in connections {
        // Only include connections from explicit Parallel nodes
        if parallel_node_ids.contains(&connection.source) {
            source_targets
                .entry(connection.source)
                .or_default()
                .insert(connection.target);
        }
    }

    source_targets
        .into_iter()
        .filter_map(|(source_id, target_ids)| {
            if target_ids.len() < 2 {
                return None;
            }

            let mut target_nodes: Vec<Node> = target_ids
                .iter()
                .copied()
                .filter_map(|id| node_by_id.get(&id).cloned())
                .collect();

            target_nodes.sort_by_key(|a| a.id.0);

            let min_y = target_nodes
                .iter()
                .map(|n| n.y)
                .fold(f32::INFINITY, f32::min);
            let max_y = target_nodes
                .iter()
                .map(|n| n.y + NODE_HEIGHT)
                .fold(f32::NEG_INFINITY, f32::max);
            let min_x = target_nodes
                .iter()
                .map(|n| n.x)
                .fold(f32::INFINITY, f32::min);
            let max_x = target_nodes
                .iter()
                .map(|n| n.x + NODE_WIDTH)
                .fold(f32::NEG_INFINITY, f32::max);

            let bounds = BoundingBox {
                x: min_x - 8.0,
                y: min_y - 8.0,
                width: (max_x - min_x) + 16.0,
                height: (max_y - min_y) + 16.0,
            };

            Some(ParallelGroup {
                parallel_node_id: source_id,
                branch_node_ids: target_nodes.iter().map(|n| n.id).collect(),
                bounding_box: bounds,
                branch_count: target_nodes.len(),
                aggregate_status: AggregateStatus::Pending,
            })
        })
        .collect()
}

pub fn sanitize_bend_input_edge(input: f32, start_bend: f32) -> f32 {
    if !input.is_finite() {
        return start_bend;
    }
    input.clamp(-BEND_CLAMP, BEND_CLAMP)
}

#[allow(dead_code)]
pub fn normalize_bend_delta(page_delta: f32, zoom: f32) -> f32 {
    if !zoom.is_finite() || zoom <= 0.0 {
        return 0.0;
    }
    page_delta / zoom
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[cfg(test)]
mod tests {
    use super::{
        build_node_lookup, calculate_parallel_offset, create_smooth_step_path,
        find_parallel_branches, get_source_point, get_target_point, normalize_bend_delta,
        resolve_edge_anchors, resolve_edge_anchors_with_parallel, sanitize_bend_input_edge,
        AggregateStatus, BoundingBox, ParallelGroup, Position, Rect,
    };
    use oya_frontend::graph::{Connection, Node, NodeId, PortName, WorkflowNode};
    use uuid::Uuid;

    // Constants for test data builders
    const NODE_HEIGHT: f32 = 68.0;

    // ==================== Test Data Builders ====================

    fn build_node(id: NodeId, x: f32, y: f32) -> Node {
        let mut node = Node::from_workflow_node(
            format!("Node {}", id),
            WorkflowNode::Run(oya_frontend::graph::RunConfig::default()),
            x,
            y,
        );
        node.id = id;
        node
    }

    /// Build a Parallel node for testing
    fn build_parallel_node(id: NodeId, x: f32, y: f32) -> Node {
        let mut node = Node::from_workflow_node(
            format!("Parallel {}", id),
            WorkflowNode::Parallel(oya_frontend::graph::workflow_node::ParallelConfig::default()),
            x,
            y,
        );
        node.id = id;
        node
    }

    fn build_connection(id: Uuid, source: NodeId, target: NodeId) -> Connection {
        Connection {
            id,
            source,
            target,
            source_port: PortName::from("out"),
            target_port: PortName::from("in"),
        }
    }

    fn build_node_with_id(id: NodeId, x: f32, y: f32) -> Node {
        let mut node = build_node(id, x, y);
        node.id = id;
        node
    }

    // ==================== find_parallel_branches Tests ====================

    #[test]
    fn given_source_with_two_targets_when_find_parallel_then_returns_one_group() {
        let source_id = NodeId::new();
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();

        let source = build_parallel_node(source_id, 100.0, 100.0);
        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);

        let nodes = vec![source.clone(), target_a.clone(), target_b.clone()];

        let conn_a = build_connection(Uuid::new_v4(), source_id, target_a_id);
        let conn_b = build_connection(Uuid::new_v4(), source_id, target_b_id);
        let connections = vec![conn_a, conn_b];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert_eq!(groups.len(), 1);
        let group = &groups[0];

        assert_eq!(group.parallel_node_id, source_id);
        assert_eq!(group.branch_node_ids.len(), 2);
        // Target nodes are sorted by ID lexicographically
        let mut sorted_ids = [target_a_id, target_b_id];
        sorted_ids.sort_by(|left, right| left.0.cmp(&right.0));
        assert_eq!(group.branch_node_ids[0], sorted_ids[0]);
        assert_eq!(group.branch_node_ids[1], sorted_ids[1]);
        assert_eq!(group.bounding_box.x, 292.0);
        assert_eq!(group.bounding_box.y, 92.0);
        assert_eq!(group.bounding_box.width, 236.0);
        assert_eq!(group.bounding_box.height, 184.0);
    }

    #[test]
    fn given_source_with_three_targets_when_find_parallel_then_returns_one_group() {
        let source_id = NodeId::new();
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();
        let target_c_id = NodeId::new();

        let source = build_parallel_node(source_id, 100.0, 100.0);
        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);
        let target_c = build_node(target_c_id, 300.0, 300.0);

        let nodes = vec![source, target_a, target_b, target_c];

        let conn_a = build_connection(Uuid::new_v4(), source_id, target_a_id);
        let conn_b = build_connection(Uuid::new_v4(), source_id, target_b_id);
        let conn_c = build_connection(Uuid::new_v4(), source_id, target_c_id);
        let connections = vec![conn_a, conn_b, conn_c];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].branch_node_ids.len(), 3);
    }

    #[test]
    fn given_source_with_many_targets_when_find_parallel_then_returns_one_group() {
        let source_id = NodeId::new();
        let mut target_ids = vec![];
        let mut nodes = vec![];
        let mut connections = vec![];

        for i in 0..5 {
            let target_id = NodeId::new();
            target_ids.push(target_id);
            nodes.push(build_node(target_id, 300.0, 100.0 + (i as f32) * 100.0));
            connections.push(build_connection(Uuid::new_v4(), source_id, target_id));
        }

        let source = build_parallel_node(source_id, 100.0, 100.0);
        nodes.insert(0, source);

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].branch_node_ids.len(), 5);
    }

    #[test]
    fn given_single_connection_when_find_parallel_then_returns_empty_vec() {
        let source_id = NodeId::new();
        let target_id = NodeId::new();

        let source = build_node(source_id, 100.0, 100.0);
        let target = build_node(target_id, 300.0, 100.0);

        let nodes = vec![source, target];

        let connection = build_connection(Uuid::new_v4(), source_id, target_id);
        let connections = vec![connection];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert!(groups.is_empty());
    }

    #[test]
    fn given_empty_connections_when_find_parallel_then_returns_empty_vec() {
        let nodes = vec![];
        let connections = vec![];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert!(groups.is_empty());
    }

    #[test]
    fn given_empty_nodes_when_find_parallel_then_returns_empty_vec() {
        let nodes = vec![];
        let source_id = NodeId::new();
        let target_id = NodeId::new();

        let connection = build_connection(Uuid::new_v4(), source_id, target_id);
        let connections = vec![connection];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert!(groups.is_empty());
    }

    #[test]
    fn given_many_non_parallel_sources_when_find_parallel_then_returns_empty_vec() {
        let mut nodes = vec![];
        let mut connections = vec![];

        for i in 0..10 {
            let source_id = NodeId::new();
            let target_id = NodeId::new();

            let source = build_node(source_id, 100.0, (i as f32) * 200.0);
            let target = build_node(target_id, 300.0, (i as f32) * 200.0);

            nodes.push(source);
            nodes.push(target);

            connections.push(build_connection(Uuid::new_v4(), source_id, target_id));
        }

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert!(groups.is_empty());
    }

    #[test]
    fn given_duplicate_connections_when_find_parallel_then_treats_as_single_connection() {
        let source_id = NodeId::new();
        let target_id = NodeId::new();

        let source = build_node(source_id, 100.0, 100.0);
        let target = build_node(target_id, 300.0, 100.0);

        let nodes = vec![source, target];

        // Two connections from same source to same target
        let conn_a = build_connection(Uuid::new_v4(), source_id, target_id);
        let conn_b = build_connection(Uuid::new_v4(), source_id, target_id);
        let connections = vec![conn_a, conn_b];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert!(groups.is_empty());
    }

    #[test]
    fn given_mixed_parallel_and_non_parallel_when_find_parallel_then_only_parallel_returned() {
        let source_a_id = NodeId::new();
        let source_b_id = NodeId::new();
        let target_a1_id = NodeId::new();
        let target_a2_id = NodeId::new();
        let target_b1_id = NodeId::new();

        let source_a = build_parallel_node(source_a_id, 100.0, 100.0);
        let source_b = build_node(source_b_id, 100.0, 300.0);
        let target_a1 = build_node(target_a1_id, 300.0, 100.0);
        let target_a2 = build_node(target_a2_id, 300.0, 200.0);
        let target_b1 = build_node(target_b1_id, 300.0, 300.0);

        let nodes = vec![source_a, source_b, target_a1, target_a2, target_b1];

        let conn_a1 = build_connection(Uuid::new_v4(), source_a_id, target_a1_id);
        let conn_a2 = build_connection(Uuid::new_v4(), source_a_id, target_a2_id);
        let conn_b1 = build_connection(Uuid::new_v4(), source_b_id, target_b1_id);
        let connections = vec![conn_a1, conn_a2, conn_b1];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].branch_node_ids.len(), 2);
    }

    // ==================== calculate_parallel_offset Tests ====================

    #[test]
    fn given_two_targets_when_calculate_offset_then_returns_symmetric_values() {
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();

        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);

        let targets = vec![target_a.clone(), target_b.clone()];

        let offset_a = calculate_parallel_offset(&target_a_id, &targets, NODE_HEIGHT);
        let offset_b = calculate_parallel_offset(&target_b_id, &targets, NODE_HEIGHT);

        let spacing = NODE_HEIGHT / 2.5;

        let mut sorted_ids = [target_a_id, target_b_id];
        sorted_ids.sort_by(|left, right| left.0.cmp(&right.0));

        let expected_a = if target_a_id == sorted_ids[0] {
            -spacing / 2.0
        } else {
            spacing / 2.0
        };
        let expected_b = -expected_a;

        assert_eq!(offset_a, expected_a);
        assert_eq!(offset_b, expected_b);
    }

    #[test]
    fn given_three_targets_when_calculate_offset_then_returns_centered_values() {
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();
        let target_c_id = NodeId::new();

        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);
        let target_c = build_node(target_c_id, 300.0, 300.0);

        let targets = vec![target_a, target_b, target_c];

        let offset_a = calculate_parallel_offset(&target_a_id, &targets, NODE_HEIGHT);
        let offset_b = calculate_parallel_offset(&target_b_id, &targets, NODE_HEIGHT);
        let offset_c = calculate_parallel_offset(&target_c_id, &targets, NODE_HEIGHT);

        let spacing = NODE_HEIGHT / 2.5;

        let mut sorted_ids = [target_a_id, target_b_id, target_c_id];
        sorted_ids.sort_by(|left, right| left.0.cmp(&right.0));

        let expected_for = |id: NodeId| {
            if id == sorted_ids[0] {
                -spacing
            } else if id == sorted_ids[1] {
                0.0
            } else {
                spacing
            }
        };

        assert_eq!(offset_a, expected_for(target_a_id));
        assert_eq!(offset_b, expected_for(target_b_id));
        assert_eq!(offset_c, expected_for(target_c_id));
    }

    #[test]
    fn given_four_targets_when_calculate_offset_then_returns_symmetric_values() {
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();
        let target_c_id = NodeId::new();
        let target_d_id = NodeId::new();

        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);
        let target_c = build_node(target_c_id, 300.0, 300.0);
        let target_d = build_node(target_d_id, 300.0, 400.0);

        let targets = vec![target_a, target_b, target_c, target_d];

        let offset_a = calculate_parallel_offset(&target_a_id, &targets, NODE_HEIGHT);
        let offset_b = calculate_parallel_offset(&target_b_id, &targets, NODE_HEIGHT);
        let offset_c = calculate_parallel_offset(&target_c_id, &targets, NODE_HEIGHT);
        let offset_d = calculate_parallel_offset(&target_d_id, &targets, NODE_HEIGHT);

        let spacing = NODE_HEIGHT / 2.5;

        let mut sorted_ids = [target_a_id, target_b_id, target_c_id, target_d_id];
        sorted_ids.sort_by(|left, right| left.0.cmp(&right.0));

        let expected_for = |id: NodeId| {
            if id == sorted_ids[0] {
                -spacing * 1.5
            } else if id == sorted_ids[1] {
                -spacing / 2.0
            } else if id == sorted_ids[2] {
                spacing / 2.0
            } else {
                spacing * 1.5
            }
        };

        assert_eq!(offset_a, expected_for(target_a_id));
        assert_eq!(offset_b, expected_for(target_b_id));
        assert_eq!(offset_c, expected_for(target_c_id));
        assert_eq!(offset_d, expected_for(target_d_id));
    }

    #[test]
    fn given_single_target_when_calculate_offset_then_returns_zero() {
        let target_id = NodeId::new();
        let target = build_node(target_id, 300.0, 100.0);

        let targets = vec![target];

        let offset = calculate_parallel_offset(&target_id, &targets, NODE_HEIGHT);

        assert_eq!(offset, 0.0);
    }

    #[test]
    fn given_target_id_not_in_targets_when_calculate_offset_then_returns_zero() {
        let target_id = NodeId::new();
        let other_id = NodeId::new();
        let target = build_node(other_id, 300.0, 100.0);

        let targets = vec![target];

        let offset = calculate_parallel_offset(&target_id, &targets, NODE_HEIGHT);

        assert_eq!(offset, 0.0);
    }

    #[test]
    fn given_targets_at_varying_y_positions_when_calculate_offset_then_respects_sorted_order() {
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();
        let target_c_id = NodeId::new();

        // Create nodes with y-positions that don't match ID order
        let target_a = build_node(target_a_id, 300.0, 300.0); // y=300, but ID sorts first
        let target_b = build_node(target_b_id, 300.0, 100.0); // y=100, but ID sorts middle
        let target_c = build_node(target_c_id, 300.0, 200.0); // y=200, but ID sorts last

        let targets = vec![target_a, target_b, target_c];

        let offset_a = calculate_parallel_offset(&target_a_id, &targets, NODE_HEIGHT);
        let offset_b = calculate_parallel_offset(&target_b_id, &targets, NODE_HEIGHT);
        let offset_c = calculate_parallel_offset(&target_c_id, &targets, NODE_HEIGHT);

        // Offsets are determined by sorted ID order, not y-position
        let spacing = NODE_HEIGHT / 2.5;
        let mut sorted_ids = [target_a_id, target_b_id, target_c_id];
        sorted_ids.sort_by(|left, right| left.0.cmp(&right.0));

        let expected_for = |id: NodeId| {
            if id == sorted_ids[0] {
                -spacing
            } else if id == sorted_ids[1] {
                0.0
            } else {
                spacing
            }
        };

        assert_eq!(offset_a, expected_for(target_a_id));
        assert_eq!(offset_b, expected_for(target_b_id));
        assert_eq!(offset_c, expected_for(target_c_id));
    }

    // ==================== resolve_edge_anchors_with_parallel Tests ====================

    #[test]
    fn given_parallel_groups_when_resolve_anchors_then_offsets_applied_to_targets() {
        let source_id = NodeId::new();
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();

        let source = build_node(source_id, 100.0, 100.0);
        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);

        let nodes = vec![source, target_a.clone(), target_b.clone()];

        let conn_a = build_connection(Uuid::new_v4(), source_id, target_a_id);
        let conn_b = build_connection(Uuid::new_v4(), source_id, target_b_id);
        let connections = vec![conn_a, conn_b];

        // Create parallel group
        let group = ParallelGroup {
            parallel_node_id: source_id,
            branch_node_ids: vec![target_a_id, target_b_id],
            bounding_box: BoundingBox {
                x: 292.0,
                y: 92.0,
                width: 16.0,
                height: 116.0,
            },
            branch_count: 2,
            aggregate_status: AggregateStatus::Pending,
        };
        let groups = vec![group];

        let anchors = resolve_edge_anchors_with_parallel(&connections, &build_node_lookup(&nodes), &groups);

        let anchor_a = anchors.get(&connections[0].id.to_string()).copied();
        let anchor_b = anchors.get(&connections[1].id.to_string()).copied();

        assert!(anchor_a.is_some());
        assert!(anchor_b.is_some());

        let anchor_a = anchor_a.unwrap();
        let anchor_b = anchor_b.unwrap();

        let spacing = NODE_HEIGHT / 2.5;
        let mut sorted_ids = [target_a_id, target_b_id];
        sorted_ids.sort_by(|left, right| left.0.cmp(&right.0));

        let expected_offset_a = if target_a_id == sorted_ids[0] {
            -spacing / 2.0
        } else {
            spacing / 2.0
        };
        let expected_offset_b = -expected_offset_a;

        assert_eq!(anchor_a.from.x, 320.0); // source.x + NODE_WIDTH
        assert_eq!(anchor_a.from.y, 134.0); // source.y + NODE_HEIGHT / 2
        assert_eq!(anchor_a.to.y, 134.0 + expected_offset_a);

        assert_eq!(anchor_b.from.x, 320.0);
        assert_eq!(anchor_b.from.y, 134.0);
        assert_eq!(anchor_b.to.y, 234.0 + expected_offset_b);
    }

    #[test]
    fn given_non_parallel_edges_when_resolve_anchors_then_no_offsets_applied() {
        let source_id = NodeId::new();
        let target_id = NodeId::new();

        let source = build_node(source_id, 100.0, 100.0);
        let target = build_node(target_id, 300.0, 100.0);

        let nodes = vec![source, target];

        let connection = build_connection(Uuid::new_v4(), source_id, target_id);
        let connections = vec![connection.clone()];

        let groups: Vec<ParallelGroup> = vec![];

        let anchors = resolve_edge_anchors_with_parallel(&connections, &build_node_lookup(&nodes), &groups);

        let anchor = anchors.get(&connection.id.to_string()).copied();

        assert!(anchor.is_some());
        let anchor = anchor.unwrap();

        // No offset applied since no parallel group
        assert_eq!(anchor.to.y, 134.0); // target.y + NODE_HEIGHT / 2
    }

    #[test]
    fn given_mixed_parallel_and_non_parallel_edges_when_resolve_anchors() {
        let source_id = NodeId::new();
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();
        let target_c_id = NodeId::new();

        let source = build_node(source_id, 100.0, 100.0);
        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);
        let target_c = build_node(target_c_id, 300.0, 300.0);

        let nodes = vec![source, target_a.clone(), target_b.clone(), target_c.clone()];

        let conn_a = build_connection(Uuid::new_v4(), source_id, target_a_id);
        let conn_b = build_connection(Uuid::new_v4(), source_id, target_b_id);
        let conn_c = build_connection(Uuid::new_v4(), source_id, target_c_id);
        let connections = vec![conn_a.clone(), conn_b.clone(), conn_c.clone()];

        // Only target_a and target_b are in parallel group
        let group = ParallelGroup {
            parallel_node_id: source_id,
            branch_node_ids: vec![target_a_id, target_b_id],
            bounding_box: BoundingBox {
                x: 292.0,
                y: 92.0,
                width: 16.0,
                height: 116.0,
            },
            branch_count: 2,
            aggregate_status: AggregateStatus::Pending,
        };
        let groups = vec![group];

        let anchors = resolve_edge_anchors_with_parallel(&connections, &build_node_lookup(&nodes), &groups);

        let anchor_a = anchors.get(&conn_a.id.to_string()).copied();
        let anchor_b = anchors.get(&conn_b.id.to_string()).copied();
        let anchor_c = anchors.get(&conn_c.id.to_string()).copied();

        let spacing = NODE_HEIGHT / 2.5;
        let mut sorted_ids = [target_a_id, target_b_id];
        sorted_ids.sort_by(|left, right| left.0.cmp(&right.0));

        let expected_offset_a = if target_a_id == sorted_ids[0] {
            -spacing / 2.0
        } else {
            spacing / 2.0
        };
        let expected_offset_b = -expected_offset_a;

        // Parallel edges have offsets
        assert_eq!(anchor_a.unwrap().to.y, 134.0 + expected_offset_a);
        assert_eq!(anchor_b.unwrap().to.y, 234.0 + expected_offset_b);

        // Non-parallel edge has no offset
        assert_eq!(anchor_c.unwrap().to.y, 334.0);
    }

    // ==================== Rect Tests ====================

    #[test]
    fn given_rect_when_created_then_has_correct_values() {
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
        };

        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
    }

    // ==================== Integration Tests ====================

    #[test]
    fn given_workflow_with_parallel_branches_when_full_pipeline_then_correct_output() {
        // Complete workflow: nodes + connections -> parallel groups -> edge anchors

        let source_id = NodeId::new();
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();

        let source = build_parallel_node(source_id, 100.0, 100.0);
        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);

        let nodes = vec![source, target_a, target_b];

        let conn_a = build_connection(Uuid::new_v4(), source_id, target_a_id);
        let conn_b = build_connection(Uuid::new_v4(), source_id, target_b_id);
        let connections = vec![conn_a.clone(), conn_b.clone()];

        // Step 1: Find parallel groups
        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);
        assert_eq!(groups.len(), 1);

        // Step 2: Resolve edge anchors with parallel groups
        let anchors = resolve_edge_anchors_with_parallel(&connections, &build_node_lookup(&nodes), &groups);

        // Step 3: Verify anchors exist and have correct structure
        assert_eq!(anchors.len(), 2);

        let anchor_a = anchors.get(&conn_a.id.to_string()).copied().unwrap();
        let anchor_b = anchors.get(&conn_b.id.to_string()).copied().unwrap();

        // Both anchors start from same source point
        assert_eq!(anchor_a.from.x, anchor_b.from.x);
        assert_eq!(anchor_a.from.y, anchor_b.from.y);

        // Anchor to is at target position
        assert_eq!(anchor_a.to.x, 300.0);
        assert_eq!(anchor_b.to.x, 300.0);
    }

    // ==================== Explicit Parallel Source Gating Tests ====================

    #[test]
    fn given_non_parallel_node_with_two_targets_when_find_parallel_then_returns_empty() {
        // Even with >=2 outgoing edges, non-Parallel nodes should NOT create parallel groups
        let source_id = NodeId::new();
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();

        let source = build_node(source_id, 100.0, 100.0); // Not a Parallel node
        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);

        let nodes = vec![source.clone(), target_a.clone(), target_b.clone()];

        let conn_a = build_connection(Uuid::new_v4(), source_id, target_a_id);
        let conn_b = build_connection(Uuid::new_v4(), source_id, target_b_id);
        let connections = vec![conn_a, conn_b];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        // Should be empty because source is not WorkflowNode::Parallel
        assert!(groups.is_empty());
    }

    #[test]
    fn given_parallel_node_with_two_targets_when_find_parallel_then_returns_one_group() {
        let source_id = NodeId::new();
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();

        let source = build_parallel_node(source_id, 100.0, 100.0); // Explicit Parallel node
        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);

        let nodes = vec![source.clone(), target_a.clone(), target_b.clone()];

        let conn_a = build_connection(Uuid::new_v4(), source_id, target_a_id);
        let conn_b = build_connection(Uuid::new_v4(), source_id, target_b_id);
        let connections = vec![conn_a, conn_b];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert_eq!(groups.len(), 1);
        let group = &groups[0];

        assert_eq!(group.parallel_node_id, source_id);
        assert_eq!(group.branch_node_ids.len(), 2);
    }

    #[test]
    fn given_multiple_parallel_nodes_when_find_parallel_then_returns_groups_for_each() {
        let source_a_id = NodeId::new();
        let source_b_id = NodeId::new();
        let target_a1_id = NodeId::new();
        let target_a2_id = NodeId::new();
        let target_b1_id = NodeId::new();
        let target_b2_id = NodeId::new();

        let source_a = build_parallel_node(source_a_id, 100.0, 100.0);
        let source_b = build_parallel_node(source_b_id, 100.0, 300.0);
        let target_a1 = build_node(target_a1_id, 300.0, 100.0);
        let target_a2 = build_node(target_a2_id, 300.0, 200.0);
        let target_b1 = build_node(target_b1_id, 300.0, 300.0);
        let target_b2 = build_node(target_b2_id, 300.0, 400.0);

        let nodes = vec![
            source_a, source_b, target_a1, target_a2, target_b1, target_b2,
        ];

        let conn_a1 = build_connection(Uuid::new_v4(), source_a_id, target_a1_id);
        let conn_a2 = build_connection(Uuid::new_v4(), source_a_id, target_a2_id);
        let conn_b1 = build_connection(Uuid::new_v4(), source_b_id, target_b1_id);
        let conn_b2 = build_connection(Uuid::new_v4(), source_b_id, target_b2_id);
        let connections = vec![conn_a1, conn_a2, conn_b1, conn_b2];

        let groups = find_parallel_branches(&build_node_lookup(&nodes), &connections);

        assert_eq!(groups.len(), 2);
    }

    // ==================== get_source_point Tests ====================

    #[test]
    fn given_node_at_origin_when_get_source_point_then_returns_right_center() {
        let node = build_node(NodeId::new(), 0.0, 0.0);
        let point = get_source_point(&node);

        assert_eq!(point.x, 220.0); // 0.0 + NODE_WIDTH
        assert_eq!(point.y, 34.0); // 0.0 + NODE_HEIGHT / 2
    }

    #[test]
    fn given_node_at_nonzero_position_when_get_source_point_then_returns_right_center() {
        let node = build_node(NodeId::new(), 100.0, 200.0);
        let point = get_source_point(&node);

        assert_eq!(point.x, 320.0); // 100.0 + NODE_WIDTH
        assert_eq!(point.y, 234.0); // 200.0 + NODE_HEIGHT / 2
    }

    #[test]
    fn given_node_with_negative_coordinates_when_get_source_point_then_returns_right_center() {
        let node = build_node(NodeId::new(), -50.0, -100.0);
        let point = get_source_point(&node);

        assert_eq!(point.x, 170.0); // -50.0 + NODE_WIDTH
        assert_eq!(point.y, -66.0); // -100.0 + NODE_HEIGHT / 2
    }

    #[test]
    fn given_node_with_large_coordinates_when_get_source_point_then_returns_right_center() {
        let node = build_node(NodeId::new(), 10000.0, 50000.0);
        let point = get_source_point(&node);

        assert_eq!(point.x, 10220.0);
        assert_eq!(point.y, 50034.0);
    }

    // ==================== get_target_point Tests ====================

    #[test]
    fn given_node_at_origin_when_get_target_point_then_returns_left_center() {
        let node = build_node(NodeId::new(), 0.0, 0.0);
        let point = get_target_point(&node);

        assert_eq!(point.x, 0.0); // left edge
        assert_eq!(point.y, 34.0); // NODE_HEIGHT / 2
    }

    #[test]
    fn given_node_at_nonzero_position_when_get_target_point_then_returns_left_center() {
        let node = build_node(NodeId::new(), 100.0, 200.0);
        let point = get_target_point(&node);

        assert_eq!(point.x, 100.0); // left edge = node.x
        assert_eq!(point.y, 234.0); // 200.0 + NODE_HEIGHT / 2
    }

    #[test]
    fn given_node_with_negative_coordinates_when_get_target_point_then_returns_left_center() {
        let node = build_node(NodeId::new(), -50.0, -100.0);
        let point = get_target_point(&node);

        assert_eq!(point.x, -50.0);
        assert_eq!(point.y, -66.0);
    }

    // ==================== build_node_lookup Tests ====================

    #[test]
    fn given_empty_slice_when_build_node_lookup_then_returns_empty_map() {
        let lookup = build_node_lookup(&[]);
        assert!(lookup.is_empty());
    }

    #[test]
    fn given_single_node_when_build_node_lookup_then_returns_map_with_one_entry() {
        let id = NodeId::new();
        let node = build_node(id, 10.0, 20.0);

        let lookup = build_node_lookup(&[node]);

        assert_eq!(lookup.len(), 1);
        assert!(lookup.contains_key(&id));
        let retrieved = lookup.get(&id).unwrap();
        assert_eq!(retrieved.x, 10.0);
        assert_eq!(retrieved.y, 20.0);
    }

    #[test]
    fn given_multiple_nodes_when_build_node_lookup_then_returns_all_entries() {
        let id_a = NodeId::new();
        let id_b = NodeId::new();
        let id_c = NodeId::new();
        let node_a = build_node(id_a, 0.0, 0.0);
        let node_b = build_node(id_b, 100.0, 200.0);
        let node_c = build_node(id_c, 300.0, 400.0);

        let lookup = build_node_lookup(&[node_a, node_b, node_c]);

        assert_eq!(lookup.len(), 3);
        assert!(lookup.contains_key(&id_a));
        assert!(lookup.contains_key(&id_b));
        assert!(lookup.contains_key(&id_c));
    }

    #[test]
    fn given_duplicate_ids_when_build_node_lookup_then_last_one_wins() {
        let id = NodeId::new();
        let node_v1 = build_node_with_id(id, 10.0, 20.0);
        let node_v2 = build_node_with_id(id, 30.0, 40.0);

        let lookup = build_node_lookup(&[node_v1, node_v2]);

        assert_eq!(lookup.len(), 1);
        let retrieved = lookup.get(&id).unwrap();
        // Last one wins (standard HashMap behavior from .collect())
        assert_eq!(retrieved.x, 30.0);
        assert_eq!(retrieved.y, 40.0);
    }

    // ==================== sanitize_bend_input_edge Tests ====================

    #[test]
    fn given_finite_input_within_clamp_when_sanitize_then_returns_input() {
        let result = sanitize_bend_input_edge(50.0, 0.0);
        assert_eq!(result, 50.0);
    }

    #[test]
    fn given_finite_input_at_positive_clamp_when_sanitize_then_returns_clamp() {
        let result = sanitize_bend_input_edge(200.0, 0.0);
        assert_eq!(result, 200.0);
    }

    #[test]
    fn given_finite_input_at_negative_clamp_when_sanitize_then_returns_clamp() {
        let result = sanitize_bend_input_edge(-200.0, 0.0);
        assert_eq!(result, -200.0);
    }

    #[test]
    fn given_finite_input_exceeding_positive_clamp_when_sanitize_then_clamps() {
        let result = sanitize_bend_input_edge(500.0, 0.0);
        assert_eq!(result, 200.0);
    }

    #[test]
    fn given_finite_input_exceeding_negative_clamp_when_sanitize_then_clamps() {
        let result = sanitize_bend_input_edge(-500.0, 0.0);
        assert_eq!(result, -200.0);
    }

    #[test]
    fn given_nan_input_when_sanitize_then_returns_start_bend() {
        let result = sanitize_bend_input_edge(f32::NAN, 42.0);
        assert_eq!(result, 42.0);
    }

    #[test]
    fn given_positive_infinity_input_when_sanitize_then_returns_start_bend() {
        let result = sanitize_bend_input_edge(f32::INFINITY, 42.0);
        assert_eq!(result, 42.0);
    }

    #[test]
    fn given_negative_infinity_input_when_sanitize_then_returns_start_bend() {
        let result = sanitize_bend_input_edge(f32::NEG_INFINITY, 42.0);
        assert_eq!(result, 42.0);
    }

    #[test]
    fn given_zero_input_when_sanitize_then_returns_zero() {
        let result = sanitize_bend_input_edge(0.0, 100.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn given_negative_input_within_clamp_when_sanitize_then_returns_input() {
        let result = sanitize_bend_input_edge(-50.0, 0.0);
        assert_eq!(result, -50.0);
    }

    // ==================== resolve_edge_anchors (non-parallel) Tests ====================

    #[test]
    fn given_empty_edges_when_resolve_anchors_then_returns_empty_map() {
        let lookup = build_node_lookup(&[]);
        let anchors = resolve_edge_anchors(&[], &lookup);
        assert!(anchors.is_empty());
    }

    #[test]
    fn given_edge_with_missing_source_when_resolve_anchors_then_skips_edge() {
        let source_id = NodeId::new();
        let target_id = NodeId::new();
        let target = build_node(target_id, 300.0, 100.0);

        let lookup = build_node_lookup(&[target]);

        let conn = build_connection(Uuid::new_v4(), source_id, target_id);
        let anchors = resolve_edge_anchors(&[conn], &lookup);

        assert!(anchors.is_empty());
    }

    #[test]
    fn given_edge_with_missing_target_when_resolve_anchors_then_skips_edge() {
        let source_id = NodeId::new();
        let target_id = NodeId::new();
        let source = build_node(source_id, 100.0, 100.0);

        let lookup = build_node_lookup(&[source]);

        let conn = build_connection(Uuid::new_v4(), source_id, target_id);
        let anchors = resolve_edge_anchors(&[conn], &lookup);

        assert!(anchors.is_empty());
    }

    #[test]
    fn given_valid_edge_when_resolve_anchors_then_returns_correct_anchor() {
        let source_id = NodeId::new();
        let target_id = NodeId::new();
        let source = build_node(source_id, 100.0, 100.0);
        let target = build_node(target_id, 300.0, 200.0);

        let lookup = build_node_lookup(&[source, target]);

        let conn = build_connection(Uuid::new_v4(), source_id, target_id);
        let anchors = resolve_edge_anchors(&[conn.clone()], &lookup);

        assert_eq!(anchors.len(), 1);

        let anchor = anchors.get(&conn.id.to_string()).copied().unwrap();
        assert_eq!(anchor.from.x, 320.0); // source.x + NODE_WIDTH
        assert_eq!(anchor.from.y, 134.0); // source.y + NODE_HEIGHT / 2
        assert_eq!(anchor.to.x, 300.0); // target.x
        assert_eq!(anchor.to.y, 234.0); // target.y + NODE_HEIGHT / 2
    }

    #[test]
    fn given_multiple_valid_edges_when_resolve_anchors_then_returns_all() {
        let source_id = NodeId::new();
        let target_a_id = NodeId::new();
        let target_b_id = NodeId::new();
        let source = build_node(source_id, 100.0, 100.0);
        let target_a = build_node(target_a_id, 300.0, 100.0);
        let target_b = build_node(target_b_id, 300.0, 200.0);

        let lookup = build_node_lookup(&[source, target_a, target_b]);

        let conn_a = build_connection(Uuid::new_v4(), source_id, target_a_id);
        let conn_b = build_connection(Uuid::new_v4(), source_id, target_b_id);
        let anchors = resolve_edge_anchors(&[conn_a.clone(), conn_b.clone()], &lookup);

        assert_eq!(anchors.len(), 2);
        assert!(anchors.contains_key(&conn_a.id.to_string()));
        assert!(anchors.contains_key(&conn_b.id.to_string()));
    }

    // ==================== create_smooth_step_path Tests ====================

    #[test]
    fn given_horizontal_edge_no_bend_when_create_path_then_returns_valid_svg_path() {
        let from = Position { x: 100.0, y: 100.0 };
        let to = Position { x: 300.0, y: 100.0 };

        let (path, midpoint) = create_smooth_step_path(from, to, 0.0);

        assert!(path.starts_with('M'));
        assert!(path.contains('L'));
        assert_eq!(midpoint.x, 200.0); // midpoint of from.x and to.x
    }

    #[test]
    fn given_very_close_x_when_create_path_then_returns_straight_line() {
        let from = Position { x: 100.0, y: 100.0 };
        let to = Position { x: 101.0, y: 200.0 };

        let (path, _midpoint) = create_smooth_step_path(from, to, 0.0);

        // When dx.abs() < 2.0, should return simple M...L... path
        assert!(path.starts_with("M 100 100 L 101 200"));
    }

    #[test]
    fn given_nan_dx_when_create_path_then_returns_straight_line() {
        let from = Position { x: f32::NAN, y: 100.0 };
        let to = Position { x: 300.0, y: 200.0 };

        let (path, _midpoint) = create_smooth_step_path(from, to, 0.0);

        assert!(path.starts_with('M'));
        assert!(path.contains('L'));
    }

    #[test]
    fn given_positive_bend_when_create_path_then_midpoint_shifted() {
        let from = Position { x: 100.0, y: 100.0 };
        let to = Position { x: 300.0, y: 200.0 };

        let (_path, midpoint) = create_smooth_step_path(from, to, 50.0);

        let expected_mid_y = f32::midpoint(100.0, 200.0) + 50.0;
        assert_eq!(midpoint.y, expected_mid_y);
    }

    #[test]
    fn given_bend_exceeding_clamp_when_create_path_then_midpoint_clamped() {
        let from = Position { x: 100.0, y: 100.0 };
        let to = Position { x: 300.0, y: 200.0 };

        let (_path, midpoint) = create_smooth_step_path(from, to, 500.0);

        let expected_mid_y = f32::midpoint(100.0, 200.0) + 200.0; // clamped to BEND_CLAMP
        assert_eq!(midpoint.y, expected_mid_y);
    }

    // ==================== Zoom-Normalized Bend Tests ====================

    #[test]
    fn given_valid_zoom_when_normalize_bend_then_returns_scaled_delta() {
        let page_delta = 100.0;
        let zoom = 2.0; // 200% zoom

        let result = normalize_bend_delta(page_delta, zoom);

        assert_eq!(result, 50.0);
    }

    #[test]
    fn given_zoom_of_one_when_normalize_bend_then_returns_same_delta() {
        let page_delta = 75.0;
        let zoom = 1.0;

        let result = normalize_bend_delta(page_delta, zoom);

        assert_eq!(result, 75.0);
    }

    #[test]
    fn given_invalid_zoom_zero_when_normalize_bend_then_returns_zero() {
        let page_delta = 100.0;
        let zoom = 0.0;

        let result = normalize_bend_delta(page_delta, zoom);

        assert_eq!(result, 0.0);
    }

    #[test]
    fn given_invalid_zoom_negative_when_normalize_bend_then_returns_zero() {
        let page_delta = 100.0;
        let zoom = -1.0;

        let result = normalize_bend_delta(page_delta, zoom);

        assert_eq!(result, 0.0);
    }

    #[test]
    fn given_invalid_zoom_nan_when_normalize_bend_then_returns_zero() {
        let page_delta = 100.0;
        let zoom = f32::NAN;

        let result = normalize_bend_delta(page_delta, zoom);

        assert_eq!(result, 0.0);
    }

    #[test]
    fn given_invalid_zoom_infinity_when_normalize_bend_then_returns_zero() {
        let page_delta = 100.0;
        let zoom = f32::INFINITY;

        let result = normalize_bend_delta(page_delta, zoom);

        assert_eq!(result, 0.0);
    }

    // ==================== Shared Target Disambiguation Test ====================

    #[test]
    fn given_shared_target_across_sources_when_resolve_anchors_then_uses_source_target_match() {
        // Scenario: Two different Parallel sources both point to the SAME target
        // The anchor resolution should correctly associate each edge with its source
        let source_a_id = NodeId::new();
        let source_b_id = NodeId::new();
        let shared_target_id = NodeId::new();

        // Both sources must be Parallel nodes for parallel group detection
        let source_a = build_parallel_node(source_a_id, 100.0, 100.0);
        let source_b = build_parallel_node(source_b_id, 100.0, 300.0);
        let shared_target = build_node(shared_target_id, 300.0, 200.0);

        let nodes = vec![source_a.clone(), source_b.clone(), shared_target.clone()];

        let conn_a = build_connection(Uuid::new_v4(), source_a_id, shared_target_id);
        let conn_b = build_connection(Uuid::new_v4(), source_b_id, shared_target_id);
        let connections = vec![conn_a.clone(), conn_b.clone()];

        // Create parallel groups for each source (each has single target)
        let group_a = ParallelGroup {
            parallel_node_id: source_a_id,
            branch_node_ids: vec![shared_target_id],
            bounding_box: BoundingBox {
                x: 292.0,
                y: 192.0,
                width: 236.0,
                height: 84.0,
            },
            branch_count: 1,
            aggregate_status: AggregateStatus::Pending,
        };
        let group_b = ParallelGroup {
            parallel_node_id: source_b_id,
            branch_node_ids: vec![shared_target_id],
            bounding_box: BoundingBox {
                x: 292.0,
                y: 392.0,
                width: 236.0,
                height: 84.0,
            },
            branch_count: 1,
            aggregate_status: AggregateStatus::Pending,
        };
        let groups = vec![group_a, group_b];

        let anchors = resolve_edge_anchors_with_parallel(&connections, &build_node_lookup(&nodes), &groups);

        // Both edges should resolve to the same target position (no offset since single target)
        let anchor_a = anchors.get(&conn_a.id.to_string()).copied();
        let anchor_b = anchors.get(&conn_b.id.to_string()).copied();

        assert!(anchor_a.is_some());
        assert!(anchor_b.is_some());

        let anchor_a = anchor_a.unwrap();
        let anchor_b = anchor_b.unwrap();

        // Both should have the same target y since there's only one target in each group
        assert_eq!(anchor_a.to.y, anchor_b.to.y);
    }
}
