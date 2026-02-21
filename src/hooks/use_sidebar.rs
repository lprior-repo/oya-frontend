#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use dioxus::prelude::*;

/// Sidebar state hook - manages sidebar search and drag-drop state.
///
/// Follows functional reactive pattern with methods for state mutations.
#[derive(Clone, Copy, PartialEq)]
pub struct SidebarState {
    search: Signal<String>,
    pending_drop: Signal<Option<String>>,
}

#[allow(dead_code)]
impl SidebarState {
    /// Read-only access to search query
    pub fn search(&self) -> ReadSignal<String> {
        self.search.into()
    }

    /// Read-only access to pending drop node type
    pub fn pending_drop(&self) -> ReadSignal<Option<String>> {
        self.pending_drop.into()
    }

    /// Set search query
    pub fn set_search(mut self, query: String) {
        self.search.set(query);
    }

    /// Set pending drop node type
    pub fn set_pending_drop(mut self, node_type: Option<String>) {
        self.pending_drop.set(node_type);
    }

    /// Start dragging a node type from sidebar
    pub fn pickup_node(mut self, node_type: &'static str) {
        self.pending_drop.set(Some(node_type.to_string()));
    }

    /// Clear pending drop (after drop or cancel)
    pub fn clear_pending_drop(mut self) {
        self.pending_drop.set(None);
    }

    /// Check if there's a pending drop
    pub fn has_pending_drop(&self) -> bool {
        self.pending_drop.read().is_some()
    }
}

pub fn use_sidebar() -> SidebarState {
    let search = use_signal(String::new);
    let pending_drop = use_signal(|| None::<String>);

    SidebarState {
        search,
        pending_drop,
    }
}
