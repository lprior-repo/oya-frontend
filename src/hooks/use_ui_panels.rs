#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use dioxus::prelude::*;

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

#[derive(Clone, Copy)]
pub struct UiPanels {
    pub settings_open: ReadOnlySignal<bool>,
    pub palette_open: ReadOnlySignal<bool>,
    pub palette_query: ReadOnlySignal<String>,
    pub context_menu: ReadOnlySignal<ContextMenuState>,

    pub toggle_settings: fn(),
    pub toggle_palette: fn(),
    pub open_palette: fn(),
    pub close_palette: fn(),
    pub set_palette_query: fn(String),
    pub show_context_menu: fn(x: f32, y: f32),
    pub close_context_menu: fn(),
    pub close_all: fn(),
}

pub fn use_ui_panels() -> UiPanels {
    todo!("Implement in Task 6")
}
