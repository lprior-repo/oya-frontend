#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cognitive_complexity)]
#![forbid(unsafe_code)]

use crate::hooks::use_canvas_interaction::CanvasInteraction;
use crate::hooks::use_selection::SelectionState;
use crate::hooks::use_sidebar::SidebarState;
use crate::hooks::use_ui_panels::UiPanels;
use crate::hooks::use_workflow_state::WorkflowState;
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// onmouseenter
// ---------------------------------------------------------------------------

/// Handle canvas `onmouseenter`.
///
/// Resolves and stores the canvas origin so subsequent handlers can
/// compute mouse-relative positions. Falls back to computing the origin
/// from page and element coordinates if `app_io::canvas_origin()` returns
/// `None` (non-WASM targets).
pub fn handle_canvas_mouseenter_event(evt: &MouseEvent, canvas: CanvasInteraction) {
    let page = evt.page_coordinates();
    let origin = if let Some(origin) = crate::ui::app_io::canvas_origin() {
        origin
    } else {
        let element = evt.element_coordinates();
        let fallback_x = page.x as f32 - element.x as f32;
        let fallback_y = page.y as f32 - element.y as f32;
        (fallback_x, fallback_y)
    };
    canvas.set_origin(origin);
}

// ---------------------------------------------------------------------------
// onwheel
// ---------------------------------------------------------------------------

/// Handle canvas `onwheel` for zoom-to-cursor behavior.
///
/// Computes the zoom delta from wheel scroll and applies it at the
/// cursor position so the point under the cursor stays fixed.
pub fn handle_canvas_wheel_event(
    evt: &WheelEvent,
    canvas: CanvasInteraction,
    workflow: &WorkflowState,
) {
    evt.prevent_default();
    let page = evt.page_coordinates();
    let origin = *canvas.canvas_origin().read();
    let origin_x = origin.x;
    let origin_y = origin.y;
    let delta = -evt.delta().strip_units().y as f32 * 0.001;
    let zoom_x = page.x as f32 - origin_x;
    let zoom_y = page.y as f32 - origin_y;
    if delta.is_finite() && zoom_x.is_finite() && zoom_y.is_finite() {
        (*workflow).zoom(delta, zoom_x, zoom_y);
    }
}

// ---------------------------------------------------------------------------
// onmouseleave
// ---------------------------------------------------------------------------

/// Handle canvas `onmouseleave`.
///
/// Cancels any ongoing interaction unless actively dragging, panning,
/// marquee-selecting, or connecting -- in those cases we want the
/// interaction to continue even if the cursor leaves the canvas element.
pub fn handle_canvas_mouseleave_event(
    canvas: CanvasInteraction,
    sidebar: SidebarState,
    selection: SelectionState,
) {
    if canvas.is_dragging()
        || canvas.is_panning()
        || canvas.is_marquee()
        || canvas.is_connecting()
    {
        return;
    }
    canvas.cancel_interaction();
    sidebar.clear_pending_drop();
    selection.clear_pending_drag();
}

// ---------------------------------------------------------------------------
// onmousedown
// ---------------------------------------------------------------------------

/// Handle canvas `onmousedown`.
///
/// Routes the gesture based on the trigger button:
/// - Middle button or primary + space-hand: start panning
/// - Primary (no pending sidebar drop): start marquee selection
/// - Other buttons: clear selection
///
/// Also resolves and stores the canvas origin and current mouse position.
pub fn handle_canvas_mousedown_event(
    evt: &MouseEvent,
    panels: &UiPanels,
    canvas: CanvasInteraction,
    selection: SelectionState,
    sidebar: SidebarState,
) {
    panels.close_context_menu();
    panels.close_inline_panel();
    let trigger_button = evt.trigger_button();
    if matches!(
        trigger_button,
        Some(MouseButton::Primary | MouseButton::Auxiliary)
    ) {
        evt.prevent_default();
        selection.clear_pending_drag();
        canvas.clear_drag_anchor();
        let page = evt.page_coordinates();
        let origin = if let Some(origin) = crate::ui::app_io::canvas_origin() {
            origin
        } else {
            let coordinates = evt.element_coordinates();
            let fallback_x = page.x as f32 - coordinates.x as f32;
            let fallback_y = page.y as f32 - coordinates.y as f32;
            (fallback_x, fallback_y)
        };
        canvas.set_origin(origin);
        let page_point = (page.x as f32, page.y as f32);
        let Some(mouse_pos) =
            crate::ui::interaction_guards::safe_canvas_point(page_point, origin)
        else {
            return;
        };
        canvas.update_mouse(mouse_pos);

        let has_pending_drop = sidebar.has_pending_drop();
        if matches!(trigger_button, Some(MouseButton::Auxiliary))
            || (matches!(trigger_button, Some(MouseButton::Primary))
                && canvas.is_space_hand_active())
        {
            canvas.start_pan();
        } else if matches!(trigger_button, Some(MouseButton::Primary)) && !has_pending_drop {
            canvas.start_marquee(mouse_pos);
        }
    } else {
        selection.clear();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_source_file_when_checking_for_unwrap_then_none_found() {
        let source = include_str!("use_canvas_mouse.rs");
        let needle_unwrap = concat!(".", "unwrap", "()");
        assert!(
            !source.contains(needle_unwrap),
            "Module must not contain unwrap calls"
        );
        let needle_expect = concat!(".", "expect", "(");
        assert!(
            !source.contains(needle_expect),
            "Module must not contain expect calls"
        );
    }
}
