#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use crate::errors::WorkflowResult;
use oya_frontend::graph::{Connection, Node, NodeId, PortName, Viewport, Workflow};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct WorkflowState {
    pub workflow: ReadOnlySignal<Workflow>,
    pub workflow_name: ReadOnlySignal<String>,
    pub nodes: ReadOnlySignal<Vec<Node>>,
    pub nodes_by_id: ReadOnlySignal<HashMap<NodeId, Node>>,
    pub connections: ReadOnlySignal<Vec<Connection>>,
    pub viewport: ReadOnlySignal<Viewport>,

    pub add_node: fn(node_type: &str, x: f32, y: f32) -> NodeId,
    pub remove_node: fn(node_id: NodeId) -> WorkflowResult<()>,
    pub add_connection: fn(source: NodeId, target: NodeId, source_port: &PortName, target_port: &PortName) -> WorkflowResult<()>,
    pub zoom: fn(delta: f32, center_x: f32, center_y: f32),
    pub pan: fn(dx: f32, dy: f32),
    pub undo: fn() -> bool,
    pub redo: fn() -> bool,
    pub save_undo_point: fn(),
}

pub fn use_workflow_state() -> WorkflowState {
    todo!("Implement in Task 3")
}
