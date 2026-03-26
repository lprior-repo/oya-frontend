#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use crate::errors::{WorkflowError, WorkflowResult};
use dioxus::prelude::*;
use oya_frontend::graph::{
    Connection, ConnectionError, ConnectionResult, Node, NodeId, PortName, Viewport, Workflow,
};
use std::collections::HashMap;

fn push_undo_snapshot(undo_stack: &mut Vec<Workflow>, snapshot: Workflow, cap: usize) {
    undo_stack.push(snapshot);
    if undo_stack.len() > cap {
        let _ = undo_stack.remove(0);
    }
}

fn apply_undo(
    workflow: &mut Workflow,
    undo_stack: &mut Vec<Workflow>,
    redo_stack: &mut Vec<Workflow>,
) -> bool {
    match undo_stack.pop() {
        Some(snapshot) => {
            let current = workflow.clone();
            redo_stack.push(current);
            *workflow = snapshot;
            true
        }
        None => false,
    }
}

fn apply_redo(
    workflow: &mut Workflow,
    undo_stack: &mut Vec<Workflow>,
    redo_stack: &mut Vec<Workflow>,
) -> bool {
    match redo_stack.pop() {
        Some(snapshot) => {
            let current = workflow.clone();
            undo_stack.push(current);
            *workflow = snapshot;
            true
        }
        None => false,
    }
}

fn viewport_center_node_origin(
    viewport: &Viewport,
    canvas_width: f32,
    canvas_height: f32,
) -> Option<(f32, f32)> {
    if !viewport.x.is_finite()
        || !viewport.y.is_finite()
        || !viewport.zoom.is_finite()
        || viewport.zoom <= 0.0
        || !canvas_width.is_finite()
        || !canvas_height.is_finite()
        || canvas_width <= 0.0
        || canvas_height <= 0.0
    {
        return None;
    }

    let center_x = (canvas_width * 0.5 - viewport.x) / viewport.zoom;
    let center_y = (canvas_height * 0.5 - viewport.y) / viewport.zoom;
    if !center_x.is_finite() || !center_y.is_finite() {
        return None;
    }

    Some((center_x - 110.0, center_y - 34.0))
}

fn merge_run_result(mut current: Workflow, completed: Workflow) -> Workflow {
    let Workflow {
        nodes,
        execution_queue,
        current_step,
        history,
        execution_records,
        ..
    } = completed;

    let mut completed_nodes: HashMap<NodeId, Node> =
        nodes.into_iter().map(|node| (node.id, node)).collect();

    current.nodes.iter_mut().for_each(|node| {
        if let Some(executed) = completed_nodes.remove(&node.id) {
            node.last_output = executed.last_output;
            node.executing = executed.executing;
            node.skipped = executed.skipped;
            node.error = executed.error;
            node.execution_state = executed.execution_state;
            node.metadata = executed.metadata;
            node.execution_data = executed.execution_data;
        } else {
            node.executing = false;
        }
    });

    current.execution_queue = execution_queue;
    current.current_step = current_step;
    current.history = history;
    current.execution_records = execution_records;
    current
}

fn add_connection_transaction(
    workflow: &mut Workflow,
    undo_stack: &mut Vec<Workflow>,
    redo_stack: &mut Vec<Workflow>,
    source: NodeId,
    target: NodeId,
    source_port: &PortName,
    target_port: &PortName,
) -> WorkflowResult<()> {
    let snapshot = workflow.clone();
    match workflow.add_connection_checked(source, target, source_port, target_port) {
        Ok(ConnectionResult::Created) => {
            push_undo_snapshot(undo_stack, snapshot, 60);
            redo_stack.clear();
            Ok(())
        }
        Err(error) => Err(map_connection_error(error)),
    }
}

fn remove_nodes_transaction(
    workflow: &mut Workflow,
    undo_stack: &mut Vec<Workflow>,
    redo_stack: &mut Vec<Workflow>,
    node_ids: &[NodeId],
) -> WorkflowResult<()> {
    if node_ids.is_empty() {
        return Ok(());
    }

    if let Some(missing_id) = node_ids
        .iter()
        .copied()
        .find(|id| !workflow.nodes.iter().any(|node| node.id == *id))
    {
        return Err(WorkflowError::NodeNotFound(missing_id));
    }

    let snapshot = workflow.clone();
    node_ids
        .iter()
        .for_each(|node_id| workflow.remove_node(*node_id));
    push_undo_snapshot(undo_stack, snapshot, 60);
    redo_stack.clear();
    Ok(())
}

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

async fn run_workflow_detached(mut workflow: Workflow, ingress_url: String) -> Workflow {
    workflow.restate_ingress_url = ingress_url;
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
    pub fn undo_stack(&self) -> ReadSignal<Vec<Workflow>> {
        self.undo_stack.into()
    }

    /// Access to redo stack signal
    pub fn redo_stack(&self) -> ReadSignal<Vec<Workflow>> {
        self.redo_stack.into()
    }

    /// Save current state to undo stack before mutation
    pub fn save_undo_point(mut self) {
        let current = self.workflow.read().clone();
        push_undo_snapshot(&mut self.undo_stack.write(), current, 60);
        self.redo_stack.write().clear();
    }

    /// Add a new node at the specified position
    pub fn add_node(mut self, node_type: &str, x: f32, y: f32) -> NodeId {
        self.save_undo_point();
        self.workflow.write().add_node(node_type, x, y)
    }

    /// Add a node at the viewport center
    pub fn add_node_at_viewport_center(self, node_type: &str) -> NodeId {
        self.add_node_at_viewport_center_with_canvas(node_type, 1280.0, 760.0)
    }

    /// Add a node at the viewport center using explicit canvas dimensions
    pub fn add_node_at_viewport_center_with_canvas(
        mut self,
        node_type: &str,
        canvas_width: f32,
        canvas_height: f32,
    ) -> NodeId {
        self.save_undo_point();
        let viewport = self.workflow.read().viewport.clone();
        if let Some((x, y)) = viewport_center_node_origin(&viewport, canvas_width, canvas_height) {
            self.workflow.write().add_node(node_type, x, y)
        } else {
            self.workflow.write().add_node(node_type, 0.0, 0.0)
        }
    }

    /// Remove a node by ID - returns error if not found
    pub fn remove_node(self, node_id: NodeId) -> WorkflowResult<()> {
        self.remove_nodes(&[node_id])
    }

    /// Remove multiple nodes as a single undo transaction
    pub fn remove_nodes(mut self, node_ids: &[NodeId]) -> WorkflowResult<()> {
        let mut workflow = self.workflow.write();
        let mut undo_stack = self.undo_stack.write();
        let mut redo_stack = self.redo_stack.write();
        remove_nodes_transaction(&mut workflow, &mut undo_stack, &mut redo_stack, node_ids)
    }

    /// Add a connection between two nodes
    pub fn add_connection(
        mut self,
        source: NodeId,
        target: NodeId,
        source_port: &PortName,
        target_port: &PortName,
    ) -> WorkflowResult<()> {
        let mut workflow = self.workflow.write();
        let mut undo_stack = self.undo_stack.write();
        let mut redo_stack = self.redo_stack.write();
        add_connection_transaction(
            &mut workflow,
            &mut undo_stack,
            &mut redo_stack,
            source,
            target,
            source_port,
            target_port,
        )
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
        let mut workflow = self.workflow.read().clone();
        let did_undo = apply_undo(
            &mut workflow,
            &mut self.undo_stack.write(),
            &mut self.redo_stack.write(),
        );
        if did_undo {
            self.workflow.set(workflow);
        }
        did_undo
    }

    /// Redo last undone action - returns true if redo was performed
    pub fn redo(mut self) -> bool {
        let mut workflow = self.workflow.read().clone();
        let did_redo = apply_redo(
            &mut workflow,
            &mut self.undo_stack.write(),
            &mut self.redo_stack.write(),
        );
        if did_redo {
            self.workflow.set(workflow);
        }
        did_redo
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

    /// Run the workflow asynchronously, using `ingress_url` for Restate service calls.
    pub fn run(self, ingress_url: String) {
        let mut workflow_signal = self.workflow;
        let workflow_snapshot = workflow_signal.read().clone();

        spawn(async move {
            let workflow_result = run_workflow_detached(workflow_snapshot, ingress_url).await;
            let merged = merge_run_result(workflow_signal.read().clone(), workflow_result);
            workflow_signal.set(merged);
        });
    }

    /// Find downstream nodes (nodes connected FROM the given node)
    pub fn downstream_nodes(&self, node_id: NodeId) -> Vec<NodeId> {
        self.connections
            .read()
            .iter()
            .filter(|c| c.source == node_id)
            .map(|c| c.target)
            .collect()
    }

    /// Find upstream nodes (nodes connected TO the given node)
    pub fn upstream_nodes(&self, node_id: NodeId) -> Vec<NodeId> {
        self.connections
            .read()
            .iter()
            .filter(|c| c.target == node_id)
            .map(|c| c.source)
            .collect()
    }

    /// Move a node by a delta amount (for keyboard navigation)
    pub fn move_node_by(self, node_id: NodeId, dx: f32, dy: f32) {
        self.update_node_position(node_id, dx, dy);
    }

    /// Get the first node in the workflow (for initial selection)
    pub fn first_node_id(&self) -> Option<NodeId> {
        self.nodes.read().first().map(|n| n.id)
    }
}

fn map_connection_error(error: ConnectionError) -> WorkflowError {
    match error {
        ConnectionError::SelfConnection => WorkflowError::SelfConnection,
        ConnectionError::MissingSourceNode(node_id)
        | ConnectionError::MissingTargetNode(node_id) => WorkflowError::NodeNotFound(node_id),
        ConnectionError::WouldCreateCycle => WorkflowError::CycleDetected,
        ConnectionError::Duplicate => WorkflowError::DuplicateConnection,
        ConnectionError::TypeMismatch {
            source_type,
            target_type,
        } => WorkflowError::InvalidConnection(format!(
            "Type mismatch: {source_type} is not compatible with {target_type}"
        )),
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
                        if let Ok(mut parsed) = serde_json::from_str::<Workflow>(&json) {
                            for node in &mut parsed.nodes {
                                let config = node.config.clone();
                                node.apply_config_update(&config);
                            }
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
    use super::{
        add_connection_transaction, apply_redo, apply_undo, map_connection_error, merge_run_result,
        push_undo_snapshot, remove_nodes_transaction, run_workflow_detached,
        viewport_center_node_origin,
    };
    use crate::errors::WorkflowError;
    use oya_frontend::graph::{ConnectionError, NodeId, PortName, Viewport, Workflow};
    use serde_json::json;

    #[tokio::test]
    async fn given_detached_workflow_when_running_then_history_is_recorded() {
        let mut workflow = Workflow::new();
        let _ = workflow.add_node("http-handler", 0.0, 0.0);

        let updated = run_workflow_detached(workflow, "http://localhost:8080".to_string()).await;

        assert_eq!(updated.history.len(), 1);
        assert!(updated.history[0].success);
    }

    #[test]
    fn given_workflow_state_source_when_running_async_then_no_write_guard_spans_await() {
        let source = include_str!("use_workflow_state.rs");
        let bad_pattern = [".write()", ".run()", ".await"].join("");

        assert!(!source.contains(&bad_pattern));
    }

    #[test]
    fn given_connection_error_when_mapping_to_workflow_error_then_taxonomy_is_preserved() {
        assert_eq!(
            map_connection_error(ConnectionError::SelfConnection),
            WorkflowError::SelfConnection
        );
        assert_eq!(
            map_connection_error(ConnectionError::WouldCreateCycle),
            WorkflowError::CycleDetected
        );
        assert_eq!(
            map_connection_error(ConnectionError::Duplicate),
            WorkflowError::DuplicateConnection
        );
        let missing_source = NodeId::new();
        assert_eq!(
            map_connection_error(ConnectionError::MissingSourceNode(missing_source)),
            WorkflowError::NodeNotFound(missing_source)
        );
        let missing_target = NodeId::new();
        assert_eq!(
            map_connection_error(ConnectionError::MissingTargetNode(missing_target)),
            WorkflowError::NodeNotFound(missing_target)
        );

        let mismatch = map_connection_error(ConnectionError::TypeMismatch {
            source_type: "event".to_string(),
            target_type: "signal".to_string(),
        });
        assert!(matches!(mismatch, WorkflowError::InvalidConnection(_)));
    }

    #[test]
    fn given_more_than_sixty_snapshots_when_pushing_undo_then_stack_is_capped() {
        let mut undo_stack = Vec::new();
        let workflow = Workflow::new();

        for _ in 0..65 {
            push_undo_snapshot(&mut undo_stack, workflow.clone(), 60);
        }

        assert_eq!(undo_stack.len(), 60);
    }

    #[test]
    fn given_undo_then_redo_sequence_when_applied_then_snapshots_restore_correctly() {
        let mut workflow = Workflow::new();
        let mut undo_stack = Vec::new();
        let mut redo_stack = Vec::new();

        let mut older = Workflow::new();
        let _ = older.add_node("http-handler", 0.0, 0.0);
        let mut newer = Workflow::new();
        let _ = newer.add_node("http-handler", 0.0, 0.0);
        let _ = newer.add_node("run", 0.0, 0.0);

        workflow.clone_from(&newer);
        undo_stack.push(older.clone());

        assert!(apply_undo(&mut workflow, &mut undo_stack, &mut redo_stack));
        assert_eq!(workflow.nodes.len(), 1);
        assert_eq!(redo_stack.len(), 1);

        assert!(apply_redo(&mut workflow, &mut undo_stack, &mut redo_stack));
        assert_eq!(workflow.nodes.len(), 2);
    }

    #[test]
    fn given_viewport_and_canvas_when_computing_center_origin_then_origin_is_centered() {
        let viewport = Viewport {
            x: -200.0,
            y: -100.0,
            zoom: 2.0,
        };

        let origin = viewport_center_node_origin(&viewport, 1280.0, 760.0);

        assert_eq!(origin, Some((310.0, 206.0)));
    }

    #[test]
    fn given_invalid_zoom_when_computing_center_origin_then_none_is_returned() {
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            zoom: 0.0,
        };

        let origin = viewport_center_node_origin(&viewport, 1280.0, 760.0);

        assert_eq!(origin, None);
    }

    #[test]
    fn given_failed_connection_attempt_when_adding_then_undo_and_redo_are_unchanged() {
        let mut workflow = Workflow::new();
        let node = workflow.add_node("run", 0.0, 0.0);
        let mut undo_stack = vec![Workflow::new()];
        let mut redo_stack = vec![Workflow::new()];
        let main = PortName::from("main");
        let workflow_before = workflow.clone();
        let undo_before = undo_stack.clone();
        let redo_before = redo_stack.clone();

        let result = add_connection_transaction(
            &mut workflow,
            &mut undo_stack,
            &mut redo_stack,
            node,
            node,
            &main,
            &main,
        );

        assert!(matches!(result, Err(WorkflowError::SelfConnection)));
        assert_eq!(workflow, workflow_before);
        assert_eq!(undo_stack, undo_before);
        assert_eq!(redo_stack, redo_before);
    }

    #[test]
    fn given_successful_connection_attempt_when_adding_then_undo_is_pushed_and_redo_cleared() {
        let mut workflow = Workflow::new();
        let source = workflow.add_node("http-handler", 0.0, 0.0);
        let target = workflow.add_node("run", 100.0, 0.0);
        let workflow_before = workflow.clone();
        let mut undo_stack = Vec::new();
        let mut redo_stack = vec![Workflow::new()];
        let main = PortName::from("main");

        let result = add_connection_transaction(
            &mut workflow,
            &mut undo_stack,
            &mut redo_stack,
            source,
            target,
            &main,
            &main,
        );

        assert!(result.is_ok());
        assert_eq!(workflow.connections.len(), 1);
        assert_eq!(undo_stack, vec![workflow_before]);
        assert!(redo_stack.is_empty());
    }

    #[test]
    fn given_local_edits_when_merging_run_result_then_layout_edits_are_preserved() {
        let mut baseline = Workflow::new();
        let node_id = baseline.add_node("run", 0.0, 0.0);

        let mut current = baseline.clone();
        if let Some(node) = current.nodes.iter_mut().find(|node| node.id == node_id) {
            node.x = 500.0;
            node.y = 260.0;
            node.description = "local-edit".to_string();
        }

        let mut completed = baseline.clone();
        if let Some(node) = completed.nodes.iter_mut().find(|node| node.id == node_id) {
            node.last_output = Some(json!({"ok": true}));
            node.skipped = true;
        }

        let merged = merge_run_result(current, completed);
        let merged_node = merged.nodes.iter().find(|node| node.id == node_id).cloned();

        assert!(merged_node.is_some());
        let merged_node = merged_node.unwrap_or_default();
        assert_eq!(merged_node.x, 500.0);
        assert_eq!(merged_node.y, 260.0);
        assert_eq!(merged_node.description, "local-edit");
        assert_eq!(merged_node.last_output, Some(json!({"ok": true})));
        assert!(merged_node.skipped);
    }

    #[test]
    fn given_batch_remove_when_removing_nodes_then_single_snapshot_is_pushed() {
        let mut workflow = Workflow::new();
        let first = workflow.add_node("run", 0.0, 0.0);
        let second = workflow.add_node("run", 120.0, 0.0);
        let _third = workflow.add_node("run", 240.0, 0.0);
        let workflow_before = workflow.clone();
        let mut undo_stack = Vec::new();
        let mut redo_stack = vec![Workflow::new()];

        let result = remove_nodes_transaction(
            &mut workflow,
            &mut undo_stack,
            &mut redo_stack,
            &[first, second],
        );

        assert!(result.is_ok());
        assert_eq!(workflow.nodes.len(), 1);
        assert_eq!(undo_stack, vec![workflow_before]);
        assert!(redo_stack.is_empty());
    }

    #[test]
    fn given_missing_node_in_batch_when_removing_nodes_then_no_snapshot_is_pushed() {
        let mut workflow = Workflow::new();
        let existing = workflow.add_node("run", 0.0, 0.0);
        let missing = NodeId::new();
        let workflow_before = workflow.clone();
        let mut undo_stack = Vec::new();
        let mut redo_stack = vec![Workflow::new()];

        let result = remove_nodes_transaction(
            &mut workflow,
            &mut undo_stack,
            &mut redo_stack,
            &[existing, missing],
        );

        assert!(matches!(result, Err(WorkflowError::NodeNotFound(id)) if id == missing));
        assert_eq!(workflow, workflow_before);
        assert!(undo_stack.is_empty());
        assert_eq!(redo_stack.len(), 1);
    }
}
