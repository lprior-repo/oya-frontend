#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use oya_frontend::graph::NodeId;
use crate::ui::edges::Position as FlowPosition;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum InteractionMode {
    Idle,
    Panning,
    Dragging { node_ids: Vec<NodeId> },
    Connecting { from: NodeId, handle: String },
    Marquee { start: (f32, f32) },
}

#[derive(Clone, Copy)]
pub struct CanvasInteraction {
    pub mode: ReadOnlySignal<InteractionMode>,
    pub is_space_hand: ReadOnlySignal<bool>,
    pub mouse_pos: ReadOnlySignal<(f32, f32)>,
    pub canvas_origin: ReadOnlySignal<(f32, f32)>,
    pub temp_edge_to: ReadOnlySignal<Option<FlowPosition>>,
    pub hovered_handle: ReadOnlySignal<Option<(NodeId, String)>>,

    pub start_pan: fn(),
    pub start_drag: fn(node_id: NodeId, selected_ids: Vec<NodeId>),
    pub start_connect: fn(node_id: NodeId, handle: String),
    pub start_marquee: fn(pos: (f32, f32)),
    pub update_mouse: fn(pos: (f32, f32)),
    pub end_interaction: fn(),
    pub set_origin: fn((f32, f32)),
    pub enable_space_hand: fn(),
    pub cancel_interaction: fn(),
}

pub fn use_canvas_interaction() -> CanvasInteraction {
    todo!("Implement in Task 5")
}
