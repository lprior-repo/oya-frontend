#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use oya_frontend::graph::NodeId;
use dioxus::prelude::*;

/// Selection state hook - manages which nodes are selected.
///
/// Follows functional reactive pattern:
/// - State stored in Copy signals
/// - Methods for mutations
/// - `ReadSignal` accessors for derived views
#[derive(Clone, Copy, PartialEq)]
pub struct SelectionState {
    selected_id: Signal<Option<NodeId>>,
    selected_ids: Signal<Vec<NodeId>>,
}

#[allow(dead_code)]
impl SelectionState {
    /// Access to the primary selected node ID signal
    pub fn selected_id(&self) -> Signal<Option<NodeId>> {
        self.selected_id
    }

    /// Access to all selected node IDs signal
    pub fn selected_ids(&self) -> Signal<Vec<NodeId>> {
        self.selected_ids
    }

    // === Mutation methods (take `self` since Copy) ===

    /// Select a single node (clears multi-selection)
    pub fn select_single(mut self, id: NodeId) {
        self.selected_id.set(Some(id));
        self.selected_ids.set(vec![id]);
    }

    /// Toggle a node in the selection
    pub fn toggle(mut self, id: NodeId) {
        let ids = self.selected_ids.read().clone();
        let mut new_ids = ids.clone();
        if let Some(pos) = new_ids.iter().position(|&x| x == id) {
            new_ids.remove(pos);
            if new_ids.is_empty() {
                self.selected_id.set(None);
            } else {
                self.selected_id.set(new_ids.first().copied());
            }
        } else {
            new_ids.push(id);
            self.selected_id.set(Some(id));
        }
        self.selected_ids.set(new_ids);
    }

    /// Add a node to selection without clearing
    pub fn add_to_selection(mut self, id: NodeId) {
        let ids = self.selected_ids.read().clone();
        if !ids.contains(&id) {
            let mut new_ids = ids;
            new_ids.push(id);
            self.selected_ids.set(new_ids);
        }
    }

    /// Set multiple selected nodes at once
    pub fn set_multiple(mut self, ids: Vec<NodeId>) {
        self.selected_id.set(ids.first().copied());
        self.selected_ids.set(ids);
    }

    /// Clear all selection
    pub fn clear(mut self) {
        self.selected_id.set(None);
        self.selected_ids.set(Vec::new());
    }

    /// Check if a node is selected
    pub fn is_selected(&self, id: NodeId) -> bool {
        self.selected_ids.read().contains(&id)
    }

    /// Get the number of selected nodes
    pub fn count(&self) -> usize {
        self.selected_ids.read().len()
    }

    /// Check if anything is selected
    pub fn has_selection(&self) -> bool {
        !self.selected_ids.read().is_empty()
    }
}

pub fn use_selection() -> SelectionState {
    let selected_id = use_signal(|| None::<NodeId>);
    let selected_ids = use_signal(Vec::<NodeId>::new);

    SelectionState {
        selected_id,
        selected_ids,
    }
}
