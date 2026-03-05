#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use dioxus::prelude::*;
use oya_frontend::graph::NodeId;

fn toggle_palette_state(current_open: bool) -> (bool, bool) {
    let next_open = !current_open;
    let should_clear_query = next_open;
    (next_open, should_clear_query)
}

fn toggle_inline_target(current: Option<NodeId>, node_id: NodeId) -> Option<NodeId> {
    if current == Some(node_id) {
        None
    } else {
        Some(node_id)
    }
}

/// Context menu state
#[derive(Clone, Debug, PartialEq, Store)]
pub struct ContextMenuState {
    pub open: bool,
    pub x: f32,
    pub y: f32,
}

impl Default for ContextMenuState {
    fn default() -> Self {
        Self {
            open: false,
            x: 0.0,
            y: 0.0,
        }
    }
}

/// UI panels hook - manages modal and overlay state.
///
/// Follows functional reactive pattern with methods for panel control.
#[derive(Clone, Copy, PartialEq)]
pub struct UiPanels {
    settings_open: Signal<bool>,
    palette_open: Signal<bool>,
    palette_query: Signal<String>,
    context_menu: Store<ContextMenuState>,
    context_menu_state: Memo<ContextMenuState>,
    inline_panel_node_id: Signal<Option<NodeId>>,
}

#[allow(dead_code)]
impl UiPanels {
    /// Read-only access to settings panel state
    pub fn settings_open(&self) -> ReadSignal<bool> {
        self.settings_open.into()
    }

    /// Read-only access to command palette state
    pub fn palette_open(&self) -> ReadSignal<bool> {
        self.palette_open.into()
    }

    /// Read-only access to palette query string
    pub fn palette_query(&self) -> ReadSignal<String> {
        self.palette_query.into()
    }

    /// Read-only access to context menu state
    pub fn context_menu(&self) -> ReadSignal<ContextMenuState> {
        self.context_menu_state.into()
    }

    // === Settings panel ===

    /// Toggle settings panel
    pub fn toggle_settings(mut self) {
        let current = *self.settings_open.read();
        self.settings_open.set(!current);
    }

    /// Open settings panel
    pub fn open_settings(mut self) {
        self.settings_open.set(true);
    }

    /// Close settings panel
    pub fn close_settings(mut self) {
        self.settings_open.set(false);
    }

    // === Command palette ===

    /// Toggle command palette
    pub fn toggle_palette(mut self) {
        let current = *self.palette_open.read();
        let (next_open, should_clear_query) = toggle_palette_state(current);
        self.palette_open.set(next_open);
        if should_clear_query {
            self.palette_query.set(String::new());
        }
    }

    /// Open command palette
    pub fn open_palette(mut self) {
        self.palette_open.set(true);
        self.palette_query.set(String::new());
    }

    /// Close command palette
    pub fn close_palette(mut self) {
        self.palette_open.set(false);
    }

    /// Set palette query string
    pub fn set_palette_query(mut self, query: String) {
        self.palette_query.set(query);
    }

    /// Clear palette query
    pub fn clear_palette_query(mut self) {
        self.palette_query.set(String::new());
    }

    // === Context menu ===

    /// Show context menu at position
    pub fn show_context_menu(mut self, x: f32, y: f32) {
        self.context_menu.set(ContextMenuState { open: true, x, y });
    }

    /// Close context menu
    pub fn close_context_menu(mut self) {
        self.context_menu.set(ContextMenuState::default());
    }

    /// Check if context menu is open
    pub fn is_context_menu_open(&self) -> bool {
        self.context_menu_state.read().open
    }

    // === Bulk operations ===

    /// Close all panels
    pub fn close_all(mut self) {
        self.palette_open.set(false);
        self.settings_open.set(false);
        self.context_menu.set(ContextMenuState::default());
        self.inline_panel_node_id.set(None);
    }

    /// Check if any panel is open
    pub fn any_open(&self) -> bool {
        *self.settings_open.read()
            || *self.palette_open.read()
            || self.context_menu_state.read().open
            || self.inline_panel_node_id.read().is_some()
    }

    // === Inline panel ===

    pub fn inline_panel_node_id(&self) -> ReadSignal<Option<NodeId>> {
        self.inline_panel_node_id.into()
    }

    pub fn open_inline_panel(mut self, node_id: NodeId) {
        self.inline_panel_node_id.set(Some(node_id));
    }

    pub fn close_inline_panel(mut self) {
        self.inline_panel_node_id.set(None);
    }

    pub fn toggle_inline_panel(mut self, node_id: NodeId) {
        let current = *self.inline_panel_node_id.read();
        self.inline_panel_node_id
            .set(toggle_inline_target(current, node_id));
    }

    pub fn is_inline_panel_open(&self, node_id: NodeId) -> bool {
        *self.inline_panel_node_id.read() == Some(node_id)
    }
}

pub fn use_ui_panels() -> UiPanels {
    let settings_open = use_signal(|| false);
    let palette_open = use_signal(|| false);
    let palette_query = use_signal(String::new);
    let context_menu = use_store(ContextMenuState::default);
    let context_menu_state = use_memo(move || context_menu.cloned());
    let inline_panel_node_id = use_signal(|| None);

    UiPanels {
        settings_open,
        palette_open,
        palette_query,
        context_menu,
        context_menu_state,
        inline_panel_node_id,
    }
}

#[cfg(test)]
mod tests {
    use super::{toggle_inline_target, toggle_palette_state, ContextMenuState, UiPanels};
    use dioxus::prelude::*;
    use dioxus::signals::Signal;
    use oya_frontend::graph::NodeId;

    fn create_test_state() -> UiPanels {
        UiPanels {
            settings_open: Signal::new(false),
            palette_open: Signal::new(false),
            palette_query: Signal::new(String::new()),
            context_menu: Store::new(ContextMenuState::default()),
            context_menu_state: Memo::new(ContextMenuState::default),
            inline_panel_node_id: Signal::new(None),
        }
    }

    #[test]
    fn given_palette_closed_when_toggling_then_it_opens_and_query_should_clear() {
        let (open, clear_query) = toggle_palette_state(false);
        assert!(open);
        assert!(clear_query);
    }

    #[test]
    fn given_palette_open_when_toggling_then_it_closes_and_query_not_forced_clear() {
        let (open, clear_query) = toggle_palette_state(true);
        assert!(!open);
        assert!(!clear_query);
    }

    #[test]
    fn given_same_node_when_toggling_inline_panel_then_panel_closes() {
        let id = NodeId::new();
        let next = toggle_inline_target(Some(id), id);
        assert_eq!(next, None);
    }

    // Contract tests for state transitions

    #[test]
    fn test_toggle_settings_opens_from_closed() {
        let mut state = create_test_state();
        
        state.toggle_settings();
        
        assert!(*state.settings_open().read());
    }

    #[test]
    fn test_toggle_settings_closes_from_open() {
        let mut state = create_test_state();
        state.open_settings();
        
        state.toggle_settings();
        
        assert!(!*state.settings_open().read());
    }

    #[test]
    fn test_toggle_palette_opens_and_clears_query() {
        let mut state = create_test_state();
        
        state.toggle_palette();
        
        assert!(*state.palette_open().read());
        assert!(state.palette_query().read().is_empty());
    }

    #[test]
    fn test_toggle_palette_closes_without_clearing_query() {
        let mut state = create_test_state();
        state.open_palette();
        state.set_palette_query("test query".to_string());
        
        state.toggle_palette();
        
        assert!(!*state.palette_open().read());
    }

    #[test]
    fn test_close_all_clears_everything() {
        let mut state = create_test_state();
        state.open_settings();
        state.open_palette();
        state.show_context_menu(100.0, 200.0);
        state.open_inline_panel(NodeId::new());
        
        state.close_all();
        
        assert!(!*state.settings_open().read());
        assert!(!*state.palette_open().read());
        assert!(!state.is_context_menu_open());
        assert!(state.inline_panel_node_id().read().is_none());
    }

    #[test]
    fn test_context_menu_show_at_position() {
        let mut state = create_test_state();
        
        state.show_context_menu(150.0, 250.0);
        
        assert!(state.is_context_menu_open());
        let ctx_signal = state.context_menu();
        let ctx = ctx_signal.read();
        assert_eq!(ctx.x, 150.0);
        assert_eq!(ctx.y, 250.0);
    }

    #[test]
    fn test_context_menu_close() {
        let mut state = create_test_state();
        state.show_context_menu(100.0, 100.0);
        
        state.close_context_menu();
        
        assert!(!state.is_context_menu_open());
    }

    #[test]
    fn test_inline_panel_toggle_opens() {
        let mut state = create_test_state();
        let node_id = NodeId::new();
        
        state.toggle_inline_panel(node_id);
        
        assert!(state.is_inline_panel_open(node_id));
    }

    #[test]
    fn test_inline_panel_toggle_closes_existing() {
        let mut state = create_test_state();
        let node_id = NodeId::new();
        state.open_inline_panel(node_id);
        
        state.toggle_inline_panel(node_id);
        
        assert!(!state.is_inline_panel_open(node_id));
    }

    #[test]
    fn test_any_open_returns_true_when_panel_open() {
        let state = create_test_state();
        
        assert!(!state.any_open());
        
        let mut state2 = create_test_state();
        state2.open_settings();
        
        assert!(state2.any_open());
    }

    #[test]
    fn test_open_palette_clears_query() {
        let mut state = create_test_state();
        state.set_palette_query("previous".to_string());
        
        state.open_palette();
        
        assert!(state.palette_query().read().is_empty());
    }
}
