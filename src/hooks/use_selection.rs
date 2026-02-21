#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use oya_frontend::graph::NodeId;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct SelectionState {
    pub selected_id: ReadOnlySignal<Option<NodeId>>,
    pub selected_ids: ReadOnlySignal<Vec<NodeId>>,

    pub select_single: fn(NodeId),
    pub clear: fn(),
    pub is_selected: fn(NodeId) -> bool,
}

pub fn use_selection() -> SelectionState {
    todo!("Implement in Task 4")
}
