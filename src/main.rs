#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use crate::ui::{
    CanvasContextMenu, FlowEdges, FlowMinimap, FlowNodeComponent, FlowPosition, FlowToolbar,
    NodeCommandPalette, NodeSidebar, SelectedNodePanel,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use oya_frontend::graph::PortName;

const DRAG_THRESHOLD_PX: f32 = 4.0;

mod errors;
mod hooks;
mod ui;

#[cfg(any(test, target_arch = "wasm32"))]
const fn should_end_canvas_interaction(
    is_dragging: bool,
    is_panning: bool,
    is_marquee: bool,
    is_connecting: bool,
) -> bool {
    is_dragging || is_panning || is_marquee || is_connecting
}

#[cfg(target_arch = "wasm32")]
struct GlobalMouseupListenerInner {
    window: web_sys::Window,
    callback: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::MouseEvent)>,
}

#[cfg(target_arch = "wasm32")]
impl Drop for GlobalMouseupListenerInner {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;

        let _ = self.window.remove_event_listener_with_callback(
            "mouseup",
            self.callback.as_ref().unchecked_ref(),
        );
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
struct GlobalMouseupListener {
    _inner: std::rc::Rc<GlobalMouseupListenerInner>,
}

#[cfg(target_arch = "wasm32")]
fn register_global_mouseup_listener(
    canvas: crate::hooks::use_canvas_interaction::CanvasInteraction,
    selection: crate::hooks::use_selection::SelectionState,
) -> Option<GlobalMouseupListener> {
    use wasm_bindgen::{closure::Closure, JsCast};
    use web_sys::window;

    let window = window()?;
    let canvas_end = canvas;
    let selection_end = selection;
    let callback = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_evt| {
        if should_end_canvas_interaction(
            canvas_end.is_dragging(),
            canvas_end.is_panning(),
            canvas_end.is_marquee(),
            canvas_end.is_connecting(),
        ) {
            canvas_end.end_interaction();
        }
        selection_end.clear_pending_drag();
    });

    if window
        .add_event_listener_with_callback("mouseup", callback.as_ref().unchecked_ref())
        .is_err()
    {
        return None;
    }

    Some(GlobalMouseupListener {
        _inner: std::rc::Rc::new(GlobalMouseupListenerInner { window, callback }),
    })
}

// --- Application Shell ---

#[component]
fn App() -> Element {
    // Hook-based state management
    let workflow = crate::hooks::use_workflow_state();
    let selection = crate::hooks::use_selection();
    let canvas = crate::hooks::use_canvas_interaction();
    let panels = crate::hooks::use_ui_panels();
    let sidebar = crate::hooks::use_sidebar();

    #[cfg(target_arch = "wasm32")]
    {
        let _global_mouseup_listener = use_hook(move || {
            register_global_mouseup_listener(canvas, selection)
        });

        use_effect(move || {
            use wasm_bindgen::{JsCast, JsValue};
            use web_sys::window;

            let Some(win) = window() else {
                return;
            };

            if let Ok(tailwind) = js_sys::Reflect::get(win.as_ref(), &JsValue::from_str("tailwind")) {
                if let Ok(refresh) = js_sys::Reflect::get(&tailwind, &JsValue::from_str("refresh")) {
                    if let Some(refresh_fn) = refresh.dyn_ref::<js_sys::Function>() {
                        let _ = refresh_fn.call0(&tailwind);
                    }
                }
            }
        });
    }

    // Persist workflow to localStorage
    use_effect(move || {
        let wf_signal = workflow.workflow();
        let wf = wf_signal.read();
        if let Ok(_json) = serde_json::to_string(&*wf) {
            #[cfg(target_arch = "wasm32")]
            {
                use web_sys::window;
                let storage = window().and_then(|w| w.local_storage().ok()).flatten();
                if let Some(s) = storage {
                    let _ = s.set_item("flow-wasm-v1-workflow", &_json);
                }
            }
        }
    });

    // Derived computations
    let nodes = workflow.nodes();
    let nodes_by_id = workflow.nodes_by_id();
    let connections = workflow.connections();
    let node_count = use_memo(move || workflow.nodes().read().len());
    let edge_count = use_memo(move || workflow.connections().read().len());
    let zoom_label = use_memo(move || format!("{:.0}%", workflow.viewport().read().zoom * 100.0));
    let can_undo = use_memo(move || workflow.can_undo());
    let can_redo = use_memo(move || workflow.can_redo());
    let viewport_state = workflow.viewport();

    let vp = workflow.viewport();
    let vx = vp.read().x;
    let vy = vp.read().y;
    let vz = vp.read().zoom;

    // Temp edge computation for connecting mode
    let temp_edge = use_memo(move || {
        let mode = canvas.mode().read().clone();
        if let crate::hooks::InteractionMode::Connecting { from, handle } = mode {
            let to = *canvas.temp_edge_to().read();
            if let Some((to_pos, _)) = to {
                let node = nodes_by_id.read().get(&from).cloned();
                node.map(|n| {
                    let from_pos = if handle == "source" {
                        FlowPosition {
                            x: n.x + 110.0,
                            y: n.y + 68.0,
                        }
                    } else {
                        FlowPosition {
                            x: n.x + 110.0,
                            y: n.y,
                        }
                    };
                    (from_pos, to_pos)
                })
            } else {
                None
            }
        } else {
            None
        }
    });

    rsx! {
        script { src: "https://cdn.tailwindcss.com" }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/style.css") }
        style {
            "@keyframes dash {{ to {{ stroke-dashoffset: -16; }} }}"
            "@keyframes slide-in-right {{ from {{ transform: translateX(24px); opacity: 0; }} to {{ transform: translateX(0); opacity: 1; }} }}"
            ".animate-slide-in-right {{ animation: slide-in-right 0.22s ease-out; }}"
        }

        div { class: "relative flex h-screen w-screen flex-col overflow-hidden bg-[#f4f6fb] text-slate-900 [font-family:'Geist',_'Inter',sans-serif] select-none",
            FlowToolbar {
                workflow_name: workflow.workflow_name(),
                on_workflow_name_change: move |value| workflow.workflow_name().set(value),
                node_count: node_count,
                edge_count: edge_count,
                zoom_label: zoom_label,
                can_undo: can_undo,
                can_redo: can_redo,
                on_zoom_in: move |_| workflow.zoom(0.12, 640.0, 400.0),
                on_zoom_out: move |_| workflow.zoom(-0.12, 640.0, 400.0),
                on_fit_view: move |_| workflow.fit_view(1280.0, 760.0, 200.0),
                on_layout: move |_| workflow.apply_layout(),
                on_execute: move |_| workflow.run(),
                on_undo: move |_| { workflow.undo(); selection.clear(); },
                on_redo: move |_| { workflow.redo(); selection.clear(); },
                on_save: move |_| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        crate::ui::app_io::download_workflow_json(
                            &workflow.workflow_name().read(),
                            &workflow.workflow().read(),
                        );
                    }
                },
                on_settings: move |_| panels.toggle_settings()
            }

            if *panels.settings_open().read() {
                div { class: "absolute right-4 top-14 z-40 w-[280px] rounded-lg border border-slate-700 bg-slate-900/95 p-3 shadow-2xl shadow-slate-950/70 backdrop-blur",
                    div { class: "mb-2 flex items-center justify-between",
                        h4 { class: "text-[12px] font-semibold text-slate-100", "Workflow Settings" }
                        button {
                            class: "flex h-6 w-6 items-center justify-center rounded-md text-slate-500 transition-colors hover:bg-slate-800 hover:text-slate-100",
                            onclick: move |_| panels.close_settings(),
                            crate::ui::icons::XIcon { class: "h-3.5 w-3.5" }
                        }
                    }
                    p { class: "mb-3 text-[11px] leading-relaxed text-slate-400", "Use Save to export the current workflow as JSON. Undo and Redo track recent graph edits." }
                    div { class: "flex items-center gap-2",
                        button {
                            class: "flex h-8 flex-1 items-center justify-center rounded-md border border-slate-700 text-[12px] text-slate-300 transition-colors hover:bg-slate-800 hover:text-slate-100",
                            onclick: move |_| panels.close_settings(),
                            "Close"
                        }
                    }
                }
            }

            NodeCommandPalette {
                open: panels.palette_open(),
                query: panels.palette_query(),
                on_query_change: move |value| panels.set_palette_query(value),
                on_close: move |()| panels.close_palette(),
                on_pick: move |node_type| {
                    workflow.add_node_at_viewport_center(node_type);
                    panels.close_palette();
                }
            }

            CanvasContextMenu {
                open: panels.context_menu().read().open,
                x: panels.context_menu().read().x,
                y: panels.context_menu().read().y,
                on_close: move |_| panels.close_context_menu(),
                on_add_node: move |_| {
                    panels.close_context_menu();
                    panels.open_palette();
                },
                on_fit_view: move |_| {
                    panels.close_context_menu();
                    workflow.fit_view(1280.0, 760.0, 200.0);
                },
                on_layout: move |_| {
                    panels.close_context_menu();
                    workflow.apply_layout();
                }
            }

            div { class: "flex flex-1 overflow-hidden",
                NodeSidebar {
                    search: sidebar.search(),
                    on_search_change: move |value| sidebar.set_search(value),
                    on_pickup_node: move |node_type: &'static str| {
                        sidebar.pickup_node(node_type);
                    },
                    on_add_node: move |node_type: &'static str| {
                        sidebar.clear_pending_drop();
                        workflow.add_node_at_viewport_center(node_type);
                    }
                }

                main {
                    class: "relative flex-1 overflow-hidden bg-[#f8fafc] {canvas.cursor_class()}",
                    tabindex: "0",
                    onmouseenter: move |evt| {
                        let page = evt.page_coordinates();
                        let origin = if let Some(origin) = crate::ui::app_io::canvas_origin() {
                            origin
                        } else {
                            let element = evt.element_coordinates();
                            #[allow(clippy::cast_possible_truncation)]
                            let fallback_x = page.x as f32 - element.x as f32;
                            #[allow(clippy::cast_possible_truncation)]
                            let fallback_y = page.y as f32 - element.y as f32;
                            (fallback_x, fallback_y)
                        };
                        canvas.set_origin(origin);
                    },
                    oncontextmenu: move |evt| {
                        evt.prevent_default();
                        canvas.cancel_interaction();
                        selection.clear_pending_drag();
                        canvas.clear_drag_anchor();
                        let coordinates = evt.page_coordinates();
                        #[allow(clippy::cast_possible_truncation)]
                        let cx = coordinates.x as f32;
                        #[allow(clippy::cast_possible_truncation)]
                        let cy = coordinates.y as f32;
                        if cx.is_finite() && cy.is_finite() {
                            panels.show_context_menu(cx, cy);
                        }
                    },
                    onkeydown: move |evt| {
                        let key = evt.key().to_string().to_lowercase();

                        if panels.any_open() {
                            if key == "escape" {
                                evt.prevent_default();
                                panels.close_all();
                            }
                            return;
                        }

                        if key == " " || key == "space" {
                            evt.prevent_default();
                            canvas.enable_space_hand();
                            return;
                        }

                        if key == "escape" {
                            evt.prevent_default();
                            panels.close_all();
                            canvas.cancel_interaction();
                            selection.clear_pending_drag();
                            return;
                        }

                        if key == "k" {
                            evt.prevent_default();
                            panels.toggle_palette();
                            return;
                        }

                        if key == "+" || key == "=" || key == "add" {
                            evt.prevent_default();
                            workflow.zoom(0.12, 640.0, 400.0);
                            return;
                        }

                        if key == "-" || key == "_" || key == "subtract" {
                            evt.prevent_default();
                            workflow.zoom(-0.12, 640.0, 400.0);
                            return;
                        }

                        if key == "0" {
                            evt.prevent_default();
                            workflow.fit_view(1280.0, 760.0, 200.0);
                            return;
                        }

                        if key == "z" {
                            evt.prevent_default();
                            workflow.undo();
                            selection.clear();
                            return;
                        }

                        if key == "y" {
                            evt.prevent_default();
                            workflow.redo();
                            selection.clear();
                            return;
                        }

                        if key == "backspace" || key == "delete" {
                            let ids = selection.selected_ids().read().clone();
                            if ids.is_empty() {
                                return;
                            }

                            evt.prevent_default();
                            for node_id in ids {
                                let _ = workflow.remove_node(node_id);
                            }
                            selection.clear();
                        }
                    },
                    onkeyup: move |evt| {
                        let key = evt.key().to_string().to_lowercase();
                        if key == " " || key == "space" {
                            evt.prevent_default();
                            canvas.disable_space_hand();
                            canvas.end_interaction();
                            selection.clear_pending_drag();
                            canvas.clear_drag_anchor();
                        }
                    },
                    onwheel: move |evt| {
                        evt.prevent_default();
                        let page = evt.page_coordinates();
                        let (origin_x, origin_y) = *canvas.canvas_origin().read();
                        #[allow(clippy::cast_possible_truncation)]
                        let delta = -evt.delta().strip_units().y as f32 * 0.001;
                        #[allow(clippy::cast_possible_truncation)]
                        let zoom_x = page.x as f32 - origin_x;
                        #[allow(clippy::cast_possible_truncation)]
                        let zoom_y = page.y as f32 - origin_y;
                        if delta.is_finite() && zoom_x.is_finite() && zoom_y.is_finite() {
                            workflow.zoom(delta, zoom_x, zoom_y);
                        }
                    },
                    onmousemove: move |evt| {
                        let page = evt.page_coordinates();
                        let (origin_x, origin_y) = *canvas.canvas_origin().read();
                        #[allow(clippy::cast_possible_truncation)]
                        let (mx, my) = (page.x as f32 - origin_x, page.y as f32 - origin_y);
                        if !mx.is_finite() || !my.is_finite() {
                            return;
                        }
                        let (lx, ly) = *canvas.mouse_pos().read();
                        let dx = mx - lx;
                        let dy = my - ly;
                        canvas.update_mouse((mx, my));

                        if canvas.drag_anchor().is_some()
                            && !canvas.is_dragging()
                            && (canvas.is_panning() || canvas.is_marquee() || canvas.is_connecting())
                        {
                            canvas.clear_drag_anchor();
                            selection.clear_pending_drag();
                        }

                        let current_vp = viewport_state.read().clone();
                        let zoom = current_vp.zoom;
                        if !crate::ui::interaction_guards::is_valid_zoom(zoom) {
                            return;
                        }

                        if canvas.drag_anchor().is_some() && !canvas.is_dragging() {
                            if let Some((ax, ay)) = canvas.drag_anchor() {
                                let moved = (mx - ax).hypot(my - ay);
                                if moved >= DRAG_THRESHOLD_PX {
                                    if let Some(node_ids) = selection.take_pending_drag() {
                                        if let Some(primary_id) = node_ids.first().copied() {
                                            canvas.start_drag(primary_id, node_ids);
                                        }
                                    }
                                    canvas.clear_drag_anchor();
                                }
                            }
                        }

                        if canvas.is_dragging() {
                            let (canvas_w, canvas_h) = crate::ui::app_io::canvas_rect_size()
                                .map_or((960.0, 720.0), std::convert::identity);
                            let edge = 56.0_f32;
                            let max_pan = 18.0_f32;

                            let pan_x = if mx < edge {
                                ((edge - mx) / edge).clamp(0.0, 1.0) * max_pan
                            } else if mx > canvas_w - edge {
                                -(((mx - (canvas_w - edge)) / edge).clamp(0.0, 1.0) * max_pan)
                            } else {
                                0.0
                            };

                            let pan_y = if my < edge {
                                ((edge - my) / edge).clamp(0.0, 1.0) * max_pan
                            } else if my > canvas_h - edge {
                                -(((my - (canvas_h - edge)) / edge).clamp(0.0, 1.0) * max_pan)
                            } else {
                                0.0
                            };

                            if pan_x != 0.0 || pan_y != 0.0 {
                                workflow.pan(pan_x, pan_y);
                            }

                            let node_dx = (dx - pan_x) / zoom;
                            let node_dy = (dy - pan_y) / zoom;
                            if let Some(node_ids) = canvas.dragging_node_ids() {
                                for node_id in node_ids {
                                    workflow.update_node_position(node_id, node_dx, node_dy);
                                }
                            }
                        } else if canvas.is_connecting() {
                            let canvas_x = (mx - current_vp.x) / zoom;
                            let canvas_y = (my - current_vp.y) / zoom;

                            if let Some((source_id, source_kind)) = canvas.connecting_from() {
                                let node_list = workflow.nodes().read().clone();
                                let snapped = crate::ui::editor_interactions::snap_handle(
                                    &node_list,
                                    mx,
                                    my,
                                    &current_vp,
                                )
                                .filter(|(node_id, handle_kind, _)| {
                                    *node_id != source_id && *handle_kind != source_kind
                                });

                                if let Some((node_id, handle_kind, snapped_pos)) = snapped {
                                    canvas.set_hovered_handle(Some((node_id, handle_kind)));
                                    canvas.set_temp_edge(Some((snapped_pos, snapped_pos)));
                                } else {
                                    canvas.set_hovered_handle(None);
                                    canvas.set_temp_edge(Some((
                                        FlowPosition { x: canvas_x, y: canvas_y },
                                        FlowPosition { x: canvas_x, y: canvas_y },
                                    )));
                                }
                            }
                        } else if canvas.is_marquee() {
                            if let Some((start, _)) = canvas.marquee_rect() {
                                canvas.update_marquee((mx, my));
                                let start_canvas = (
                                    (start.0 - current_vp.x) / zoom,
                                    (start.1 - current_vp.y) / zoom,
                                );
                                let end_canvas = (
                                    (mx - current_vp.x) / zoom,
                                    (my - current_vp.y) / zoom,
                                );
                                let rect = crate::ui::editor_interactions::normalize_rect(
                                    start_canvas,
                                    end_canvas,
                                );
                                let selected = workflow
                                    .nodes()
                                    .read()
                                    .iter()
                                    .filter(|node| {
                                        crate::ui::editor_interactions::node_intersects_rect(
                                            node.x, node.y, rect,
                                        )
                                    })
                                    .map(|node| node.id)
                                    .collect::<Vec<_>>();
                                selection.set_multiple(selected);
                            }
                        } else if canvas.is_panning() {
                            workflow.pan(dx, dy);
                        }
                    },
                    onmouseup: move |evt| {
                        let from = canvas.connecting_from();
                        let over = canvas.hovered_handle().read().clone();
                        let is_dragging = canvas.is_dragging();
                        let pending_drop = sidebar.pending_drop().read().clone();
                        let mut should_clear_selection = false;

                        if let Some((start, end)) = canvas.marquee_rect() {
                            let rect = crate::ui::editor_interactions::normalize_rect(start, end);
                            let tiny_click = (rect.2 - rect.0).abs() < 2.0 && (rect.3 - rect.1).abs() < 2.0;
                            if tiny_click
                                && crate::ui::editor_interactions::rect_contains(rect, start)
                                && !is_dragging
                            {
                                should_clear_selection = true;
                            }
                        }

                        if let (Some((src_id, src_handle)), Some((tgt_id, _))) = (from, over) {
                            if src_id != tgt_id {
                                let (source, target) = if src_handle == "source" {
                                    (src_id, tgt_id)
                                } else {
                                    (tgt_id, src_id)
                                };

                                let _ = workflow.add_connection(
                                    source,
                                    target,
                                    &PortName("main".to_string()),
                                    &PortName("main".to_string()),
                                );
                            }
                        } else if !is_dragging && !canvas.is_marquee() {
                            if let Some(node_type) = pending_drop {
                                let coords = evt.page_coordinates();
                                let (origin_x, origin_y) = *canvas.canvas_origin().read();
                                #[allow(clippy::cast_possible_truncation)]
                                let mx = coords.x as f32 - origin_x;
                                #[allow(clippy::cast_possible_truncation)]
                                let my = coords.y as f32 - origin_y;
                                if mx.is_finite() && my.is_finite() {
                                    let current_vp = viewport_state.read().clone();
                                    if crate::ui::interaction_guards::is_valid_zoom(current_vp.zoom) {
                                        let canvas_x = (mx - current_vp.x) / current_vp.zoom - 110.0;
                                        let canvas_y = (my - current_vp.y) / current_vp.zoom - 34.0;
                                        workflow.add_node(&node_type, canvas_x, canvas_y);
                                    }
                                }
                            }
                        }

                        canvas.end_interaction();
                        sidebar.clear_pending_drop();
                        selection.clear_pending_drag();

                        if should_clear_selection {
                            selection.clear();
                        }
                    },
                    onmouseleave: move |_| {
                        if canvas.is_dragging() || canvas.is_panning() || canvas.is_marquee() || canvas.is_connecting() {
                            return;
                        }
                        canvas.cancel_interaction();
                        sidebar.clear_pending_drop();
                        selection.clear_pending_drag();
                    },
                    onmousedown: move |evt| {
                        panels.close_context_menu();
                        let trigger_button = evt.trigger_button();
                        if matches!(trigger_button, Some(MouseButton::Primary | MouseButton::Auxiliary)) {
                            evt.prevent_default();
                            selection.clear_pending_drag();
                            canvas.clear_drag_anchor();
                            let page = evt.page_coordinates();
                            let origin = if let Some(origin) = crate::ui::app_io::canvas_origin() {
                                origin
                            } else {
                                let coordinates = evt.element_coordinates();
                                #[allow(clippy::cast_possible_truncation)]
                                let fallback_x = page.x as f32 - coordinates.x as f32;
                                #[allow(clippy::cast_possible_truncation)]
                                let fallback_y = page.y as f32 - coordinates.y as f32;
                                (fallback_x, fallback_y)
                            };
                            canvas.set_origin(origin);
                            #[allow(clippy::cast_possible_truncation)]
                            let page_point = (page.x as f32, page.y as f32);
                            let Some(mouse_pos) = crate::ui::interaction_guards::safe_canvas_point(page_point, origin) else {
                                return;
                            };
                            canvas.update_mouse(mouse_pos);

                            let has_pending_drop = sidebar.has_pending_drop();
                            if matches!(trigger_button, Some(MouseButton::Auxiliary))
                                || (matches!(trigger_button, Some(MouseButton::Primary))
                                    && *canvas.is_space_hand().read())
                            {
                                canvas.start_pan();
                            } else if matches!(trigger_button, Some(MouseButton::Primary))
                                && !has_pending_drop
                            {
                                canvas.start_marquee(mouse_pos);
                            }
                        } else {
                            selection.clear();
                        }
                    },

                    div {
                        class: "absolute inset-0 pointer-events-none",
                        style: "background-image: radial-gradient(circle, rgba(148, 163, 184, 0.5) 1px, transparent 1px); background-size: calc(22px * {vz}) calc(22px * {vz}); background-position: {vx}px {vy}px;"
                    }

                    div {
                        class: "absolute origin-top-left",
                        style: "transform: translate({vx}px, {vy}px) scale({vz}); will-change: transform;",
                        FlowEdges {
                            edges: connections,
                            nodes: nodes,
                            temp_edge: temp_edge
                        }

                        for node in nodes.read().iter().cloned() {
                            {
                                let node_id = node.id;
                                let is_selected = selection.is_selected(node_id);
                                let workflow_clone = workflow;
                                let selection_clone = selection;
                                let canvas_clone = canvas;

                                rsx! {
                                    FlowNodeComponent {
                                        key: "{node_id}",
                                        node,
                                        selected: is_selected,
                                        on_mouse_down: move |evt: MouseEvent| {
                                            if evt.trigger_button() != Some(MouseButton::Primary) {
                                                return;
                                            }
                                            if *canvas_clone.is_space_hand().read() {
                                                return;
                                            }
                                            evt.stop_propagation();

                                            let page = evt.page_coordinates();
                                            let origin = if let Some(origin) = crate::ui::app_io::canvas_origin() {
                                                origin
                                            } else {
                                                let coordinates = evt.element_coordinates();
                                                #[allow(clippy::cast_possible_truncation)]
                                                let fallback_x = page.x as f32 - coordinates.x as f32;
                                                #[allow(clippy::cast_possible_truncation)]
                                                let fallback_y = page.y as f32 - coordinates.y as f32;
                                                (fallback_x, fallback_y)
                                            };
                                            canvas_clone.set_origin(origin);
                                            #[allow(clippy::cast_possible_truncation)]
                                            let page_point = (page.x as f32, page.y as f32);
                                            let Some(mouse_pos) = crate::ui::interaction_guards::safe_canvas_point(page_point, origin) else {
                                                return;
                                            };
                                            canvas_clone.update_mouse(mouse_pos);

                                            let currently_selected = selection_clone.selected_ids().read().clone();
                                            let drag_targets = if currently_selected.contains(&node_id) {
                                                if currently_selected.is_empty() {
                                                    vec![node_id]
                                                } else {
                                                    currently_selected
                                                }
                                            } else {
                                                vec![node_id]
                                            };
                                            selection_clone.set_multiple(drag_targets.clone());
                                            selection_clone.set_pending_drag(drag_targets);
                                            canvas_clone.start_drag_anchor(mouse_pos);
                                        },
                                        on_click: move |_| {
                                            selection_clone.select_single(node_id);
                                        },
                                        on_handle_mouse_down: move |args: (MouseEvent, String)| {
                                            let (evt, handle_type) = args;
                                            selection_clone.clear_pending_drag();
                                            canvas_clone.clear_drag_anchor();
                                            let page = evt.page_coordinates();
                                            let origin = if let Some(origin) = crate::ui::app_io::canvas_origin() {
                                                origin
                                            } else {
                                                let coordinates = evt.element_coordinates();
                                                #[allow(clippy::cast_possible_truncation)]
                                                let fallback_x = page.x as f32 - coordinates.x as f32;
                                                #[allow(clippy::cast_possible_truncation)]
                                                let fallback_y = page.y as f32 - coordinates.y as f32;
                                                (fallback_x, fallback_y)
                                            };
                                            canvas_clone.set_origin(origin);
                                            #[allow(clippy::cast_possible_truncation)]
                                            let page_point = (page.x as f32, page.y as f32);
                                            let Some(mouse_pos) = crate::ui::interaction_guards::safe_canvas_point(page_point, origin) else {
                                                return;
                                            };
                                            canvas_clone.update_mouse(mouse_pos);
                                            canvas_clone.start_connect(node_id, handle_type.clone());
                                            selection_clone.select_single(node_id);
                                            let current_vp = workflow_clone.viewport().read().clone();
                                            #[allow(clippy::cast_possible_truncation)]
                                            let page_point = (page.x as f32, page.y as f32);
                                            if let Some((canvas_x, canvas_y)) =
                                                crate::ui::interaction_guards::safe_canvas_from_viewport(
                                                    page_point,
                                                    origin,
                                                    &current_vp,
                                                )
                                            {
                                                canvas_clone.set_temp_edge(Some((
                                                    FlowPosition { x: canvas_x, y: canvas_y },
                                                    FlowPosition { x: canvas_x, y: canvas_y },
                                                )));
                                            }
                                        },
                                        on_handle_mouse_enter: move |handle_type| canvas_clone.set_hovered_handle(Some((node_id, handle_type))),
                                        on_handle_mouse_leave: move |()| canvas_clone.set_hovered_handle(None)
                                    }
                                }
                            }
                        }
                    }

                    if let Some((start, end)) = canvas.marquee_rect() {
                        {
                            let rect = crate::ui::editor_interactions::normalize_rect(start, end);
                            let left = rect.0;
                            let top = rect.1;
                            let width = (rect.2 - rect.0).max(1.0);
                            let height = (rect.3 - rect.1).max(1.0);

                            rsx! {
                                div {
                                    class: "pointer-events-none absolute border border-indigo-400/70 bg-indigo-500/10",
                                    style: "left: {left}px; top: {top}px; width: {width}px; height: {height}px;",
                                }
                            }
                        }
                    }

                    FlowMinimap {
                        nodes: nodes,
                        edges: connections,
                        selected_node_id: selection.selected_id()
                    }
                }

                SelectedNodePanel {
                    selected_node_id: selection.selected_id(),
                    selected_node_ids: selection.selected_ids(),
                    nodes_by_id,
                    workflow: workflow.workflow(),
                    undo_stack: workflow.undo_stack(),
                    redo_stack: workflow.redo_stack(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::should_end_canvas_interaction;

    #[test]
    fn given_idle_canvas_when_global_mouseup_fires_then_interaction_stays_idle() {
        let should_end = should_end_canvas_interaction(false, false, false, false);

        assert!(!should_end);
    }

    #[test]
    fn given_active_canvas_mode_when_global_mouseup_fires_then_interaction_ends() {
        let should_end = should_end_canvas_interaction(false, true, false, false);

        assert!(should_end);
    }

    #[test]
    fn given_app_source_when_registering_mouseup_then_listener_is_not_forgotten() {
        let source = include_str!("main.rs");
        let leak_pattern = ["listener", ".forget()"].join("");

        assert!(!source.contains(&leak_pattern));
    }
}

fn main() {
    launch(App);
}
