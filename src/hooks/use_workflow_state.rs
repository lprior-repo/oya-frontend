#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use crate::errors::{WorkflowError, WorkflowResult};
use dioxus::prelude::*;
use oya_frontend::graph::{Connection, Node, NodeId, PortName, Viewport, Workflow};
use std::collections::HashMap;

/// Workflow state hook - manages workflow data, undo/redo, and derived views.
///
/// This follows the functional reactive pattern where:
/// - State is stored in Copy signals
/// - Actions are methods that mutate state immutably (clone, modify, set)
/// - Derived data is computed via Memo for performance
#[derive(Clone, Copy, PartialEq)]
pub struct WorkflowState {
    workflow: Signal<Workflow>,
    workflow_name: Signal<String>,
    undo_stack: Signal<Vec<Workflow>>,
    redo_stack: Signal<Vec<Workflow>>,
    nodes: Memo<Vec<Node>>,
    nodes_by_id: Memo<HashMap<NodeId, Node>>,
    connections: Memo<Vec<Connection>>,
    viewport: Memo<Viewport>,
}

async fn run_workflow_detached(mut workflow: Workflow) -> Workflow {
    workflow.run().await;
    workflow
}

impl WorkflowState {
    /// Access to workflow data signal
    pub fn workflow(&self) -> Signal<Workflow> {
        self.workflow
    }

    /// Access to workflow name signal
    pub fn workflow_name(&self) -> Signal<String> {
        self.workflow_name
    }

    /// Read-only access to nodes list (memoized)
    pub fn nodes(&self) -> ReadSignal<Vec<Node>> {
        self.nodes.into()
    }

    /// Read-only access to nodes by ID map (memoized)
    pub fn nodes_by_id(&self) -> ReadSignal<HashMap<NodeId, Node>> {
        self.nodes_by_id.into()
    }

    /// Read-only access to connections (memoized)
    pub fn connections(&self) -> ReadSignal<Vec<Connection>> {
        self.connections.into()
    }

    /// Read-only access to viewport (memoized)
    pub fn viewport(&self) -> ReadSignal<Viewport> {
        self.viewport.into()
    }

    /// Access to undo stack signal
    pub fn undo_stack(&self) -> Signal<Vec<Workflow>> {
        self.undo_stack
    }

    /// Access to redo stack signal
    pub fn redo_stack(&self) -> Signal<Vec<Workflow>> {
        self.redo_stack
    }

    /// Save current state to undo stack before mutation
    pub fn save_undo_point(mut self) {
        self.undo_stack.write().push(self.workflow.read().clone());
        if self.undo_stack.read().len() > 60 {
            let _ = self.undo_stack.write().remove(0);
        }
        self.redo_stack.write().clear();
    }

    /// Add a new node at the specified position
    pub fn add_node(mut self, node_type: &str, x: f32, y: f32) -> NodeId {
        self.save_undo_point();
        self.workflow.write().add_node(node_type, x, y)
    }

    /// Add a node at the viewport center
    pub fn add_node_at_viewport_center(mut self, node_type: &str) -> NodeId {
        self.save_undo_point();
        let vp = self.workflow.read().viewport.clone();
        self.workflow.write().add_node(node_type, vp.x, vp.y)
    }

    /// Remove a node by ID - returns error if not found
    pub fn remove_node(mut self, node_id: NodeId) -> WorkflowResult<()> {
        let exists = self.workflow.read().nodes.iter().any(|n| n.id == node_id);
        if !exists {
            return Err(WorkflowError::NodeNotFound(node_id));
        }
        self.save_undo_point();
        self.workflow.write().remove_node(node_id);
        Ok(())
    }

    /// Add a connection between two nodes
    pub fn add_connection(
        mut self,
        source: NodeId,
        target: NodeId,
        source_port: &PortName,
        target_port: &PortName,
    ) -> WorkflowResult<()> {
        if source == target {
            return Err(WorkflowError::SelfConnection);
        }
        self.save_undo_point();
        self.workflow
            .write()
            .add_connection(source, target, source_port, target_port);
        Ok(())
    }

    /// Zoom the viewport
    pub fn zoom(mut self, delta: f32, center_x: f32, center_y: f32) {
        self.workflow.write().zoom(delta, center_x, center_y);
    }

    /// Pan the viewport
    pub fn pan(mut self, dx: f32, dy: f32) {
        self.workflow.write().viewport.x += dx;
        self.workflow.write().viewport.y += dy;
    }

    /// Fit view to show all nodes
    pub fn fit_view(mut self, width: f32, height: f32, padding: f32) {
        self.workflow.write().fit_view(width, height, padding);
    }

    /// Apply auto-layout to nodes
    pub fn apply_layout(mut self) {
        self.save_undo_point();
        self.workflow.write().apply_layout();
    }

    /// Undo last action - returns true if undo was performed
    pub fn undo(mut self) -> bool {
        match self.undo_stack.write().pop() {
            Some(snapshot) => {
                let current = self.workflow.read().clone();
                self.redo_stack.write().push(current);
                self.workflow.set(snapshot);
                true
            }
            None => false,
        }
    }

    /// Redo last undone action - returns true if redo was performed
    pub fn redo(mut self) -> bool {
        match self.redo_stack.write().pop() {
            Some(snapshot) => {
                let current = self.workflow.read().clone();
                self.undo_stack.write().push(current);
                self.workflow.set(snapshot);
                true
            }
            None => false,
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.read().is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.read().is_empty()
    }

    /// Update node position
    pub fn update_node_position(mut self, node_id: NodeId, dx: f32, dy: f32) {
        if !dx.is_finite() || !dy.is_finite() {
            return;
        }
        self.workflow.write().update_node_position(node_id, dx, dy);
    }

    /// Run the workflow asynchronously
    pub fn run(self) {
        let mut workflow_signal = self.workflow;
        let workflow_snapshot = workflow_signal.read().clone();

        spawn(async move {
            let workflow_result = run_workflow_detached(workflow_snapshot).await;
            workflow_signal.set(workflow_result);
        });
    }
}

pub fn use_workflow_state() -> WorkflowState {
    let workflow = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            let storage = window().and_then(|w| w.local_storage().ok()).flatten();
            if let Some(s) = storage {
                match s.get_item("flow-wasm-v1-workflow") {
                    Ok(Some(json)) => {
                        if let Ok(parsed) = serde_json::from_str::<Workflow>(&json) {
                            return parsed;
                        }
                    }
                    _ => {}
                }
            }
        }
        crate::ui::app_bootstrap::default_workflow()
    });

    let workflow_name = use_signal(|| "SignupWorkflow".to_string());
    let undo_stack = use_signal(Vec::<Workflow>::new);
    let redo_stack = use_signal(Vec::<Workflow>::new);

    // Derived memos for performance
    let nodes = use_memo(move || workflow.read().nodes.clone());
    let nodes_by_id = use_memo(move || {
        workflow
            .read()
            .nodes
            .iter()
            .map(|n| (n.id, n.clone()))
            .collect()
    });
    let connections = use_memo(move || workflow.read().connections.clone());
    let viewport = use_memo(move || workflow.read().viewport.clone());

    WorkflowState {
        workflow,
        workflow_name,
        undo_stack,
        redo_stack,
        nodes,
        nodes_by_id,
        connections,
        viewport,
    }
}

#[cfg(test)]
mod tests {
    use super::run_workflow_detached;
    use oya_frontend::graph::Workflow;

    #[tokio::test]
    async fn given_detached_workflow_when_running_then_history_is_recorded() {
        let mut workflow = Workflow::new();
        let _ = workflow.add_node("transform", 0.0, 0.0);

        let updated = run_workflow_detached(workflow).await;

        assert_eq!(updated.history.len(), 1);
        assert!(updated.history[0].success);
    }

    #[test]
    fn given_workflow_state_source_when_running_async_then_no_write_guard_spans_await() {
        let source = include_str!("use_workflow_state.rs");
        let bad_pattern = [".write()", ".run()", ".await"].join("");

        assert!(!source.contains(&bad_pattern));
    }
}
