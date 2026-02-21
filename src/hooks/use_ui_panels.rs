#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use dioxus::prelude::*;

/// Context menu state
#[derive(Clone, Debug, PartialEq)]
pub struct ContextMenuState {
    pub open: bool,
    pub x: f32,
    pub y: f32,
}

impl Default for ContextMenuState {
    fn default() -> Self {
        Self { open: false, x: 0.0, y: 0.0 }
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
    context_menu: Signal<ContextMenuState>,
}

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
        self.context_menu.into()
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
        self.palette_open.set(!current);
        if !current {
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
        self.context_menu.read().open
    }

    // === Bulk operations ===

    /// Close all panels
    pub fn close_all(mut self) {
        self.palette_open.set(false);
        self.settings_open.set(false);
        self.context_menu.set(ContextMenuState::default());
    }

    /// Check if any panel is open
    pub fn any_open(&self) -> bool {
        *self.settings_open.read() || *self.palette_open.read() || self.context_menu.read().open
    }
}

pub fn use_ui_panels() -> UiPanels {
    let settings_open = use_signal(|| false);
    let palette_open = use_signal(|| false);
    let palette_query = use_signal(String::new);
    let context_menu = use_signal(ContextMenuState::default);

    UiPanels {
        settings_open,
        palette_open,
        palette_query,
        context_menu,
    }
}
