#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use crate::ui::edges::Position as FlowPosition;
use dioxus::prelude::*;
use oya_frontend::graph::NodeId;

fn drag_mode_from_selection(node_id: NodeId, selected_ids: Vec<NodeId>) -> InteractionMode {
    if selected_ids.is_empty() {
        InteractionMode::Dragging {
            node_ids: vec![node_id],
        }
    } else if !selected_ids.contains(&node_id) {
        let mut all_ids = selected_ids;
        all_ids.push(node_id);
        InteractionMode::Dragging { node_ids: all_ids }
    } else {
        InteractionMode::Dragging {
            node_ids: selected_ids,
        }
    }
}

fn update_marquee_mode(mode: &InteractionMode, pos: (f32, f32)) -> InteractionMode {
    match mode {
        InteractionMode::Marquee { start, .. } => InteractionMode::Marquee {
            start: *start,
            current: pos.into(),
        },
        _ => mode.clone(),
    }
}

fn cursor_class_for(mode: &InteractionMode, cursor_tool: CursorTool) -> &'static str {
    match mode {
        InteractionMode::Panning => "cursor-grabbing",
        InteractionMode::Idle if cursor_tool == CursorTool::SpaceHand => "cursor-grab",
        _ => "cursor-default",
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CursorTool {
    #[default]
    Select,
    SpaceHand,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum InteractionMode {
    #[default]
    Idle,
    Panning,
    Dragging {
        node_ids: Vec<NodeId>,
    },
    Connecting {
        from: NodeId,
        handle: HandleName,
    },
    Marquee {
        start: CanvasPoint,
        current: CanvasPoint,
    },
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CanvasPoint {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for CanvasPoint {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<CanvasPoint> for (f32, f32) {
    fn from(point: CanvasPoint) -> Self {
        (point.x, point.y)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HandleName(String);

impl HandleName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for HandleName {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for HandleName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum DragAnchor {
    #[default]
    None,
    Active {
        x: f32,
        y: f32,
    },
}

impl DragAnchor {
    pub fn active(x: f32, y: f32) -> Self {
        Self::Active { x, y }
    }

    #[allow(dead_code)]
    pub fn none() -> Self {
        Self::None
    }

    pub fn as_point(&self) -> Option<(f32, f32)> {
        match self {
            DragAnchor::None => None,
            DragAnchor::Active { x, y } => Some((*x, *y)),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TempEdge {
    #[default]
    None,
    Active {
        source: FlowPosition,
        target: FlowPosition,
    },
}

impl TempEdge {
    pub fn active(source: FlowPosition, target: FlowPosition) -> Self {
        Self::Active { source, target }
    }

    #[allow(dead_code)]
    pub fn none() -> Self {
        Self::None
    }

    pub fn as_positions(&self) -> Option<(FlowPosition, FlowPosition)> {
        match self {
            TempEdge::None => None,
            TempEdge::Active { source, target } => Some((*source, *target)),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum HoveredHandle {
    #[default]
    None,
    Active {
        node_id: NodeId,
        handle: HandleName,
    },
}

impl HoveredHandle {
    pub fn active(node_id: NodeId, handle: HandleName) -> Self {
        Self::Active { node_id, handle }
    }

    #[allow(dead_code)]
    pub fn none() -> Self {
        Self::None
    }

    pub fn as_tuple(&self) -> Option<(NodeId, String)> {
        match self {
            HoveredHandle::None => None,
            HoveredHandle::Active { node_id, handle } => Some((*node_id, handle.0.clone())),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct CanvasInteraction {
    mode: Signal<InteractionMode>,
    cursor_tool: Signal<CursorTool>,
    mouse_pos: Signal<CanvasPoint>,
    canvas_origin: Signal<CanvasPoint>,
    temp_edge: Signal<TempEdge>,
    hovered_handle: Signal<HoveredHandle>,
    drag_anchor: Signal<DragAnchor>,
}

#[allow(dead_code)]
impl CanvasInteraction {
    pub fn mode(&self) -> ReadSignal<InteractionMode> {
        self.mode.into()
    }

    pub fn cursor_tool(&self) -> ReadSignal<CursorTool> {
        self.cursor_tool.into()
    }

    pub fn mouse_pos(&self) -> ReadSignal<CanvasPoint> {
        self.mouse_pos.into()
    }

    pub fn canvas_origin(&self) -> ReadSignal<CanvasPoint> {
        self.canvas_origin.into()
    }

    pub fn temp_edge(&self) -> ReadSignal<TempEdge> {
        self.temp_edge.into()
    }

    pub fn hovered_handle(&self) -> ReadSignal<HoveredHandle> {
        self.hovered_handle.into()
    }

    pub fn start_pan(mut self) {
        self.mode.set(InteractionMode::Panning);
    }

    pub fn start_drag(mut self, node_id: NodeId, selected_ids: Vec<NodeId>) {
        let next_mode = drag_mode_from_selection(node_id, selected_ids);
        self.mode.set(next_mode);
    }

    pub fn start_connect(mut self, node_id: NodeId, handle: String) {
        let handle_name = HandleName::new(handle.clone());
        self.hovered_handle
            .set(HoveredHandle::active(node_id, handle_name));
        self.mode.set(InteractionMode::Connecting {
            from: node_id,
            handle: HandleName::new(handle),
        });
    }

    pub fn start_marquee(mut self, pos: (f32, f32)) {
        self.mode.set(InteractionMode::Marquee {
            start: CanvasPoint::from(pos),
            current: CanvasPoint::from(pos),
        });
    }

    pub fn update_marquee(mut self, pos: (f32, f32)) {
        let mode = self.mode.read().clone();
        self.mode.set(update_marquee_mode(&mode, pos));
    }

    pub fn update_mouse(mut self, pos: (f32, f32)) {
        self.mouse_pos.set(CanvasPoint::from(pos));
    }

    pub fn set_origin(mut self, origin: (f32, f32)) {
        self.canvas_origin.set(CanvasPoint::from(origin));
    }

    pub fn set_temp_edge(mut self, positions: Option<(FlowPosition, FlowPosition)>) {
        match positions {
            Some((source, target)) => self.temp_edge.set(TempEdge::active(source, target)),
            None => self.temp_edge.set(TempEdge::None),
        }
    }

    pub fn set_hovered_handle(mut self, handle: Option<(NodeId, String)>) {
        match handle {
            Some((node_id, handle)) => {
                self.hovered_handle
                    .set(HoveredHandle::active(node_id, HandleName::new(handle)));
            }
            None => self.hovered_handle.set(HoveredHandle::None),
        }
    }

    pub fn start_drag_anchor(mut self, pos: (f32, f32)) {
        self.drag_anchor.set(DragAnchor::active(pos.0, pos.1));
    }

    pub fn clear_drag_anchor(mut self) {
        self.drag_anchor.set(DragAnchor::None);
    }

    pub fn enable_space_hand(mut self) {
        self.cursor_tool.set(CursorTool::SpaceHand);
    }

    pub fn disable_space_hand(mut self) {
        self.cursor_tool.set(CursorTool::Select);
    }

    pub fn end_interaction(mut self) {
        self.mode.set(InteractionMode::Idle);
        self.temp_edge.set(TempEdge::None);
        self.hovered_handle.set(HoveredHandle::None);
        self.drag_anchor.set(DragAnchor::None);
    }

    pub fn cancel_interaction(mut self) {
        self.mode.set(InteractionMode::Idle);
        self.temp_edge.set(TempEdge::None);
        self.hovered_handle.set(HoveredHandle::None);
        self.cursor_tool.set(CursorTool::Select);
        self.drag_anchor.set(DragAnchor::None);
    }

    pub fn is_dragging(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Dragging { .. })
    }

    pub fn is_connecting(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Connecting { .. })
    }

    pub fn is_marquee(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Marquee { .. })
    }

    pub fn is_panning(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Panning)
    }

    pub fn is_idle(&self) -> bool {
        matches!(*self.mode.read(), InteractionMode::Idle)
    }

    pub fn is_space_hand_active(&self) -> bool {
        *self.cursor_tool.read() == CursorTool::SpaceHand
    }

    pub fn cursor_class(&self) -> &'static str {
        let mode = self.mode.read().clone();
        let cursor_tool = *self.cursor_tool.read();
        cursor_class_for(&mode, cursor_tool)
    }

    pub fn dragging_node_ids(&self) -> Option<Vec<NodeId>> {
        match &*self.mode.read() {
            InteractionMode::Dragging { node_ids } => Some(node_ids.clone()),
            _ => None,
        }
    }

    pub fn drag_anchor(&self) -> Option<(f32, f32)> {
        self.drag_anchor.read().as_point()
    }

    pub fn connecting_from(&self) -> Option<(NodeId, String)> {
        match &*self.mode.read() {
            InteractionMode::Connecting { from, handle } => {
                Some((*from, handle.as_str().to_string()))
            }
            _ => None,
        }
    }

    pub fn marquee_rect(&self) -> Option<((f32, f32), (f32, f32))> {
        match &*self.mode.read() {
            InteractionMode::Marquee { start, current } => {
                Some(((start.x, start.y), (current.x, current.y)))
            }
            _ => None,
        }
    }
}

pub fn use_canvas_interaction() -> CanvasInteraction {
    let mode = use_signal(InteractionMode::default);
    let cursor_tool = use_signal(CursorTool::default);
    let mouse_pos = use_signal(CanvasPoint::default);
    let canvas_origin = use_signal(CanvasPoint::default);
    let temp_edge = use_signal(TempEdge::default);
    let hovered_handle = use_signal(HoveredHandle::default);
    let drag_anchor = use_signal(DragAnchor::default);

    CanvasInteraction {
        mode,
        cursor_tool,
        mouse_pos,
        canvas_origin,
        temp_edge,
        hovered_handle,
        drag_anchor,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        cursor_class_for, drag_mode_from_selection, update_marquee_mode, CanvasPoint, CursorTool,
        DragAnchor, HandleName, HoveredHandle, InteractionMode,
    };
    use oya_frontend::graph::NodeId;

    #[test]
    fn given_empty_selected_ids_when_starting_drag_then_uses_node_id_as_fallback() {
        let id = NodeId::new();
        let next = drag_mode_from_selection(id, Vec::new());
        assert_eq!(next, InteractionMode::Dragging { node_ids: vec![id] });
    }

    #[test]
    fn given_marquee_mode_when_updating_then_start_is_preserved_and_current_updates() {
        let mode = InteractionMode::Marquee {
            start: CanvasPoint::from((1.0, 2.0)),
            current: CanvasPoint::from((3.0, 4.0)),
        };

        let next = update_marquee_mode(&mode, (9.0, 10.0));
        assert_eq!(
            next,
            InteractionMode::Marquee {
                start: CanvasPoint::from((1.0, 2.0)),
                current: CanvasPoint::from((9.0, 10.0))
            }
        );
    }

    #[test]
    fn given_space_hand_enabled_and_idle_when_getting_cursor_class_then_cursor_grab_is_returned() {
        let class = cursor_class_for(&InteractionMode::Idle, CursorTool::SpaceHand);
        assert_eq!(class, "cursor-grab");
    }

    #[test]
    fn given_non_empty_selected_ids_when_starting_drag_then_dragging_mode_includes_all() {
        let id = NodeId::new();
        let id2 = NodeId::new();
        let next = drag_mode_from_selection(id, vec![id2]);
        assert!(matches!(next, InteractionMode::Dragging { .. }));
        if let InteractionMode::Dragging { node_ids } = next {
            assert!(node_ids.contains(&id));
            assert!(node_ids.contains(&id2));
        }
    }

    #[test]
    fn given_node_id_not_in_selected_ids_when_starting_drag_then_includes_node_id() {
        let id = NodeId::new();
        let other_id = NodeId::new();
        let next = drag_mode_from_selection(id, vec![other_id]);
        if let InteractionMode::Dragging { node_ids } = next {
            assert!(node_ids.contains(&id));
            assert!(node_ids.contains(&other_id));
        }
    }

    #[test]
    fn given_drag_anchor_active_when_as_point_then_returns_some() {
        let anchor = DragAnchor::active(10.0, 20.0);
        assert_eq!(anchor.as_point(), Some((10.0, 20.0)));
    }

    #[test]
    fn given_drag_anchor_none_when_as_point_then_returns_none() {
        let anchor = DragAnchor::none();
        assert_eq!(anchor.as_point(), None);
    }

    #[test]
    fn given_hovered_handle_active_when_as_tuple_then_returns_some() {
        let id = NodeId::new();
        let handle = HoveredHandle::active(id, HandleName::new("output"));
        let result = handle.as_tuple();
        assert_eq!(result, Some((id, "output".to_string())));
    }
}
