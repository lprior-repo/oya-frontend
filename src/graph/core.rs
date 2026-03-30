use super::execution_types::ExecutionConfig;
use super::{can_transition, ExecutionState, Node, NodeId, Viewport, Workflow};
use crate::graph::{calc, workflow_node::WorkflowNode};
use std::str::FromStr;

impl Workflow {
    /// Set the status of a node, validating the state transition.
    ///
    /// This function is the primary public API for state transitions.
    /// It validates that the transition is allowed by the state machine
    /// and updates both the `execution_state` and `config["status"]` fields.
    ///
    /// # Errors
    /// Returns `InvalidTransition` if the state transition is not allowed.
    pub fn set_node_status(
        node: &mut Node,
        proposed_status: ExecutionState,
    ) -> Result<(), super::InvalidTransition> {
        // Validate state transition using the state machine
        if !can_transition(node.execution_state, proposed_status) {
            return Err(super::InvalidTransition {
                from: node.execution_state,
                to: proposed_status,
            });
        }

        node.execution_state = proposed_status;
        let status_text = proposed_status.to_string();

        let config_obj = match node.config.as_object().cloned() {
            Some(obj) => obj
                .into_iter()
                .chain(std::iter::once((
                    "status".to_string(),
                    serde_json::Value::String(status_text),
                )))
                .collect(),
            None => std::iter::once(("status".to_string(), serde_json::Value::String(status_text)))
                .collect(),
        };
        node.config = serde_json::Value::Object(config_obj);
        Ok(())
    }

    /// Set a node's pending status, transitioning `Idle` -> `Queued` or `Queued` -> `Queued`.
    ///
    /// This is a specialized function for setting pending status on nodes.
    /// It validates that the transition is allowed and updates `config["status"]` to "pending".
    ///
    /// # Errors
    /// Returns `InvalidTransition` if the node is not in `Idle` or `Queued` state.
    pub fn set_node_pending_status(node: &mut Node) -> Result<(), super::InvalidTransition> {
        // Validate state transition: Idle -> Queued or Queued -> Queued (self-transition allowed)
        let is_valid_transition = can_transition(node.execution_state, ExecutionState::Queued)
            || (node.execution_state == ExecutionState::Queued);
        if !is_valid_transition {
            return Err(super::InvalidTransition {
                from: node.execution_state,
                to: ExecutionState::Queued,
            });
        }

        node.execution_state = ExecutionState::Queued;
        let status_text = "pending";
        let config_obj = node.config.as_object().cloned().map_or_else(
            || {
                std::iter::once((
                    "status".to_string(),
                    serde_json::Value::String(status_text.to_string()),
                ))
                .collect::<serde_json::Map<_, _>>()
            },
            |obj| {
                obj.into_iter()
                    .chain(std::iter::once((
                        "status".to_string(),
                        serde_json::Value::String(status_text.to_string()),
                    )))
                    .collect()
            },
        );
        node.config = serde_json::Value::Object(config_obj);
        Ok(())
    }

    #[must_use]
    pub fn new() -> Self {
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
            execution_records: Vec::new(),
            restate_ingress_url: "http://localhost:8080".to_string(),
            current_memory_bytes: 0,
            execution_config: ExecutionConfig::default(),
            execution_failed: false,
        }
    }

    pub fn add_node(&mut self, node_type: &str, x: f32, y: f32) -> NodeId {
        let existing_positions: Vec<(f32, f32)> = self.nodes.iter().map(|n| (n.x, n.y)).collect();
        let (final_x, final_y) = calc::find_safe_position(&existing_positions, x, y, 30.0);

        let id = NodeId::new();
        let name = format!("{node_type} {}", self.nodes.len() + 1);

        let workflow_node = WorkflowNode::from_str(node_type)
            .unwrap_or_else(|_| WorkflowNode::Run(crate::graph::RunConfig::default()));

        let mut node = Node::from_workflow_node(name, workflow_node, final_x, final_y);
        node.id = id;
        self.nodes.push(node);
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
            let (new_x, new_y) = calc::update_node_position(node.x, node.y, dx, dy);
            node.x = new_x;
            node.y = new_y;
        }
    }

    pub fn deselect_all(&mut self) {
        self.nodes.iter_mut().for_each(|node| {
            node.set_selected(false);
        });
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.retain(|n| n.id != id);
        self.connections
            .retain(|c| c.source != id && c.target != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{PortName, RunConfig, WorkflowNode};

    #[test]
    fn given_occupied_position_when_adding_node_then_safe_position_offsets_new_node() {
        let mut workflow = Workflow::new();
        let _ = workflow.add_node("run", 100.0, 100.0);
        let id = workflow.add_node("run", 100.0, 100.0);

        let added = workflow.nodes.iter().find(|node| node.id == id);
        assert!(added.is_some_and(|node| (node.x, node.y) == (130.0, 130.0)));
    }

    #[test]
    fn given_added_node_when_returning_id_then_it_matches_inserted_node_id() {
        let mut workflow = Workflow::new();
        let id = workflow.add_node("run", 0.0, 0.0);

        assert!(workflow.nodes.iter().any(|node| node.id == id));
    }

    #[test]
    fn given_viewport_offset_and_zoom_when_adding_node_at_center_then_node_is_centered() {
        let mut workflow = Workflow::new();
        workflow.viewport = Viewport {
            x: -200.0,
            y: -100.0,
            zoom: 2.0,
        };

        workflow.add_node_at_viewport_center("run");

        let node = workflow.nodes.first();
        assert!(node.is_some_and(|n| (n.x, n.y) == (300.0, 200.0)));
    }

    #[test]
    fn given_removed_node_when_removing_then_incident_connections_are_removed() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("http-handler", 0.0, 0.0);
        let b = workflow.add_node("run", 100.0, 0.0);
        let c = workflow.add_node("run", 200.0, 0.0);
        let main = PortName::from("main");

        let _ = workflow.add_connection_checked(a, b, &main, &main);
        let _ = workflow.add_connection_checked(b, c, &main, &main);

        workflow.remove_node(b);

        assert_eq!(workflow.nodes.len(), 2);
        assert!(workflow
            .connections
            .iter()
            .all(|conn| conn.source != b && conn.target != b));
    }

    #[test]
    fn given_node_when_setting_status_then_status_is_updated_in_execution_state_and_config() {
        let mut node = Node::from_workflow_node(
            "n".to_string(),
            WorkflowNode::Run(RunConfig::default()),
            0.0,
            0.0,
        );

        // Node starts in Idle state, need to transition to Queued first
        let _ = Workflow::set_node_status(&mut node, ExecutionState::Queued);

        // Now we can transition from Queued to Running
        let _ = Workflow::set_node_status(&mut node, ExecutionState::Running);

        // execution_state should be updated
        assert_eq!(node.execution_state, ExecutionState::Running);
        // config["status"] should be updated
        assert_eq!(
            node.config
                .get("status")
                .and_then(serde_json::Value::as_str),
            Some("running")
        );
    }
}
