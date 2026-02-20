use super::{Node, NodeId, Viewport, Workflow};
use crate::graph::metadata::node_metadata;

impl Workflow {
    pub(super) fn set_node_status(node: &mut Node, status: &str) {
        if let Some(obj) = node.config.as_object_mut() {
            obj.insert(
                "status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
        } else {
            node.config = serde_json::json!({ "status": status });
        }
    }

    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            viewport: Viewport {
                x: 0.0,
                y: 0.0,
                zoom: 1.0,
            },
            execution_queue: Vec::new(),
            current_step: 0,
            history: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_type: &str, x: f32, y: f32) -> NodeId {
        let mut final_x = x;
        let mut final_y = y;

        while self
            .nodes
            .iter()
            .any(|n| (n.x - final_x).abs() < 10.0 && (n.y - final_y).abs() < 10.0)
        {
            final_x += 30.0;
            final_y += 30.0;
        }

        let id = NodeId::new();
        let name = format!("{node_type} {}", self.nodes.len() + 1);
        let (category, icon, description) = node_metadata(node_type);

        self.nodes.push(Node {
            id,
            name,
            description,
            node_type: node_type.to_string(),
            category,
            icon,
            x: final_x,
            y: final_y,
            config: serde_json::json!({}),
            last_output: None,
            selected: false,
            executing: false,
            skipped: false,
            error: None,
        });
        id
    }

    pub fn add_node_at_viewport_center(&mut self, node_type: &str) {
        let vx = self.viewport.x;
        let vy = self.viewport.y;
        let vz = self.viewport.zoom;
        let nx = (400.0 - vx) / vz;
        let ny = (300.0 - vy) / vz;
        self.add_node(node_type, nx, ny);
    }

    pub fn update_node_position(&mut self, id: NodeId, dx: f32, dy: f32) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
            node.x += dx;
            node.y += dy;
            node.x = (node.x / 10.0).round() * 10.0;
            node.y = (node.y / 10.0).round() * 10.0;
        }
    }

    pub fn deselect_all(&mut self) {
        self.nodes.iter_mut().for_each(|node| {
            node.selected = false;
        });
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.retain(|n| n.id != id);
        self.connections
            .retain(|c| c.source != id && c.target != id);
    }
}
