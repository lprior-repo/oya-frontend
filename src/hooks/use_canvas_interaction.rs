#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use oya_frontend::graph::NodeId;
use crate::ui::edges::Position as FlowPosition;
use dioxus::prelude::*;

/// Interaction mode state machine - ensures illegal states are unrepresentable.
#[derive(Clone, Debug, PartialEq)]
pub enum InteractionMode {
    Idle,
    Panning,
    Dragging { node_ids: Vec<NodeId> },
    Connecting { from: NodeId, handle: String },
    Marquee { start: (f32, f32), current: (f32, f32) },
}

impl Default for InteractionMode {
    fn default() -> Self {
        Self::Idle
    }
}

/// Canvas interaction hook - manages all canvas interaction state.
///
/// Uses a state machine (InteractionMode) to ensure consistent state.
/// Follows functional reactive pattern with methods for state transitions.
#[derive(Clone, Copy)]
pub struct CanvasInteraction {
    mode: Signal<InteractionMode>,
    is_space_hand: Signal<bool>,
    mouse_pos: Signal<(f32, f32)>,
    canvas_origin: Signal<(f32, f32)>,
    temp_edge_to: Signal<Option<FlowPosition>>,
    hovered_handle: Signal<Option<(NodeId, String)>>,
}

impl CanvasInteraction {
    /// Read-only access to current interaction mode
    pub fn mode(&self) -> ReadSignal<InteractionMode> {
        self.mode.into()
    }

    /// Read-only access to space-hand tool state
    pub fn is_space_hand(&self) -> ReadSignal<bool> {
        self.is_space_hand.into()
    }

    /// Read-only access to mouse position
    pub fn mouse_pos(&self) -> ReadSignal<(f32, f32)> {
        self.mouse_pos.into()
    }

    /// Read-only access to canvas origin (for coordinate transforms)
    pub fn canvas_origin(&self) -> ReadSignal<(f32, f32)> {
        self.canvas_origin.into()
    }

    /// Read-only access to temporary edge target position
    pub fn temp_edge_to(&self) -> ReadSignal<Option<FlowPosition>> {
        self.temp_edge_to.into()
    }

    /// Read-only access to currently hovered handle
    pub fn hovered_handle(&self) -> ReadSignal<Option<(NodeId, String)>> {
        self.hovered_handle.into()
    }

    // === State transitions ===
    // Note: Methods take `self` (not `&self`) because Signal::set requires mutable access.
    // Since CanvasInteraction is Copy, consuming `self` is cheap and allows mutation.

    /// Start panning mode
    pub fn start_pan(mut self) {
        self.mode.set(InteractionMode::Panning);
    }

    /// Start dragging nodes
    pub fn start_drag(mut self, _node_id: NodeId, selected_ids: Vec<NodeId>) {
        self.mode.set(InteractionMode::Dragging { node_ids: selected_ids });
    }

    /// Start connecting from a handle
    pub fn start_connect(mut self, node_id: NodeId, handle: String) {
        self.hovered_handle.set(Some((node_id, handle.clone())));
        self.mode.set(InteractionMode::Connecting { from: node_id, handle });
    }

    /// Start marquee selection
    pub fn start_marquee(mut self, pos: (f32, f32)) {
        self.mode.set(InteractionMode::Marquee { start: pos, current: pos });
    }

    /// Update marquee current position
    pub fn update_marquee(mut self, pos: (f32, f32)) {
        let mode = self.mode.read().clone();
        if let InteractionMode::Marquee { start, .. } = mode {
            self.mode.set(InteractionMode::Marquee { start, current: pos });
        }
    }

    /// Update mouse position
    pub fn update_mouse(mut self, pos: (f32, f32)) {
        self.mouse_pos.set(pos);
    }

    /// Set canvas origin for coordinate transforms
    pub fn set_origin(mut self, origin: (f32, f32)) {
        self.canvas_origin.set(origin);
    }

    /// Set temporary edge target position
    pub fn set_temp_edge(mut self, pos: Option<FlowPosition>) {
        self.temp_edge_to.set(pos);
    }

    /// Set hovered handle
    pub fn set_hovered_handle(mut self, handle: Option<(NodeId, String)>) {
        self.hovered_handle.set(handle);
    }

    /// Enable space-hand tool mode
    pub fn enable_space_hand(mut self) {
        self.is_space_hand.set(true);
    }

    /// Disable space-hand tool mode
    pub fn disable_space_hand(mut self) {
        self.is_space_hand.set(false);
    }

    /// End current interaction (return to idle)
    pub fn end_interaction(mut self) {
        self.mode.set(InteractionMode::Idle);
        self.temp_edge_to.set(None);
        self.hovered_handle.set(None);
    }

    /// Cancel current interaction and reset state
    pub fn cancel_interaction(mut self) {
        self.mode.set(InteractionMode::Idle);
        self.temp_edge_to.set(None);
        self.hovered_handle.set(None);
        self.is_space_hand.set(false);
    }

    // === Query methods ===

    /// Check if currently in dragging mode
    pub fn is_dragging(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Dragging { .. })
    }

    /// Check if currently in connecting mode
    pub fn is_connecting(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Connecting { .. })
    }

    /// Check if currently in marquee selection mode
    pub fn is_marquee(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Marquee { .. })
    }

    /// Check if currently in panning mode
    pub fn is_panning(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Panning)
    }

    /// Check if idle
    pub fn is_idle(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Idle)
    }

    /// Get cursor CSS class based on current mode
    pub fn cursor_class(&self) -> &'static str {
        match *self.mode.read() {
            InteractionMode::Panning => "cursor-grabbing",
            InteractionMode::Idle if *self.is_space_hand.read() => "cursor-grab",
            _ => "cursor-default",
        }
    }

    /// Get dragging node IDs if in dragging mode
    pub fn dragging_node_ids(&self) -> Option<Vec<NodeId>> {
        match &*self.mode.read() {
            InteractionMode::Dragging { node_ids } => Some(node_ids.clone()),
            _ => None,
        }
    }

    /// Get connection source if in connecting mode
    pub fn connecting_from(&self) -> Option<(NodeId, String)> {
        match &*self.mode.read() {
            InteractionMode::Connecting { from, handle } => Some((*from, handle.clone())),
            _ => None,
        }
    }

    /// Get marquee rect if in marquee mode
    pub fn marquee_rect(&self) -> Option<((f32, f32), (f32, f32))> {
        match &*self.mode.read() {
            InteractionMode::Marquee { start, current } => Some((*start, *current)),
            _ => None,
        }
    }
}

pub fn use_canvas_interaction() -> CanvasInteraction {
    let mode = use_signal(InteractionMode::default);
    let is_space_hand = use_signal(|| false);
    let mouse_pos = use_signal(|| (0.0_f32, 0.0_f32));
    let canvas_origin = use_signal(|| (0.0_f32, 0.0_f32));
    let temp_edge_to = use_signal(|| None::<FlowPosition>);
    let hovered_handle = use_signal(|| None::<(NodeId, String)>);

    CanvasInteraction {
        mode,
        is_space_hand,
        mouse_pos,
        canvas_origin,
        temp_edge_to,
        hovered_handle,
    }
}
