#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use dioxus::prelude::*;

fn pending_drop_after_pickup(node_type: &'static str) -> String {
    node_type.to_string()
}

const fn pending_drop_after_clear() -> Option<String> {
    None
}

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
        self.pending_drop
            .set(Some(pending_drop_after_pickup(node_type)));
    }

    /// Clear pending drop (after drop or cancel)
    pub fn clear_pending_drop(mut self) {
        self.pending_drop.set(pending_drop_after_clear());
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

#[cfg(test)]
mod tests {
    use super::{pending_drop_after_clear, pending_drop_after_pickup, SidebarState};
    use dioxus::prelude::ReadableExt;
    use dioxus::signals::Signal;

    fn create_test_state() -> SidebarState {
        SidebarState {
            search: Signal::new(String::new()),
            pending_drop: Signal::new(None),
        }
    }

    #[test]
    fn given_node_type_when_picking_up_then_pending_drop_is_set() {
        assert_eq!(pending_drop_after_pickup("run"), "run".to_string());
    }

    #[test]
    fn given_pending_drop_when_clearing_then_pending_drop_is_none() {
        assert_eq!(pending_drop_after_clear(), None);
    }

    // Contract tests for state transitions

    #[test]
    fn given_empty_state_when_pickup_node_then_pending_drop_is_some() {
        let mut state = create_test_state();
        
        state.pickup_node("http-handler");
        
        assert!(state.has_pending_drop());
    }

    #[test]
    fn given_empty_state_when_pickup_node_then_returns_correct_type() {
        let mut state = create_test_state();
        
        state.pickup_node("http-handler");
        
        assert_eq!(*state.pending_drop().read(), Some("http-handler".to_string()));
    }

    #[test]
    fn given_pending_drop_state_when_clear_pending_drop_then_pending_drop_is_none() {
        let mut state = create_test_state();
        state.pickup_node("run");
        
        state.clear_pending_drop();
        
        assert!(!state.has_pending_drop());
    }

    #[test]
    fn given_no_pending_drop_when_clear_pending_drop_then_remains_none() {
        let mut state = create_test_state();
        
        state.clear_pending_drop();
        state.clear_pending_drop();
        
        assert!(!state.has_pending_drop());
    }

    #[test]
    fn given_pending_drop_when_set_pending_drop_none_then_clears() {
        let mut state = create_test_state();
        state.pickup_node("run");
        
        state.set_pending_drop(None);
        
        assert!(!state.has_pending_drop());
    }

    #[test]
    fn given_empty_state_when_set_pending_drop_some_then_sets() {
        let mut state = create_test_state();
        
        state.set_pending_drop(Some("condition".to_string()));
        
        assert!(state.has_pending_drop());
    }

    #[test]
    fn test_invariant_pending_drop_always_some_or_none() {
        let mut state = create_test_state();
        
        // Should always be valid (Some or None)
        let _ = *state.pending_drop().read();
        
        state.pickup_node("test");
        let _ = *state.pending_drop().read();
        
        state.clear_pending_drop();
        let _ = *state.pending_drop().read();
    }

    #[test]
    fn test_search_does_not_affect_pending_drop() {
        let mut state = create_test_state();
        state.pickup_node("run");
        
        state.set_search("search query".to_string());
        
        assert!(state.has_pending_drop());
        assert_eq!(*state.pending_drop().read(), Some("run".to_string()));
    }

    #[test]
    fn test_pickup_overwrites_existing_pending_drop() {
        let mut state = create_test_state();
        state.pickup_node("first");
        
        state.pickup_node("second");
        
        assert_eq!(*state.pending_drop().read(), Some("second".to_string()));
    }
}
