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
use oya_frontend::graph::{NodeId, PortName, Workflow};

mod ui;
mod errors;

// --- Application Shell ---

#[component]
fn App() -> Element {
    let mut workflow = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            let storage = window().and_then(|w| w.local_storage().ok()).flatten();
            if let Some(s) = storage {
                match s.get_item("flow-wasm-v1-workflow") {
                    Ok(Some(json)) => match serde_json::from_str::<Workflow>(&json) {
                        Ok(parsed) => return parsed,
                        Err(_) => {}
                    },
                    _ => {}
                }
            }
        }
        crate::ui::app_bootstrap::default_workflow()
    });

    let mut selected_node_id = use_signal(|| None::<NodeId>);
    let mut selected_node_ids = use_signal(Vec::<NodeId>::new);
    let mut dragging_node_ids = use_signal(Vec::<NodeId>::new);
    let mut is_panning = use_signal(|| false);
    let mut is_space_hand_tool = use_signal(|| false);
    let mut last_mouse_pos = use_signal(|| (0.0, 0.0));
    let mut marquee_start = use_signal(|| None::<(f32, f32)>);
    let mut marquee_current = use_signal(|| None::<(f32, f32)>);
    let mut connecting_from = use_signal(|| None::<(NodeId, String)>);
    let mut temp_edge_to = use_signal(|| None::<FlowPosition>);
    let mut hovered_handle = use_signal(|| None::<(NodeId, String)>);
    let mut workflow_name = use_signal(|| "SignupWorkflow".to_string());
    let mut sidebar_search = use_signal(String::new);
    let mut pending_sidebar_drop = use_signal(|| None::<String>);
    let mut undo_stack = use_signal(Vec::<Workflow>::new);
    let mut redo_stack = use_signal(Vec::<Workflow>::new);
    let mut show_settings = use_signal(|| false);
    let mut show_command_palette = use_signal(|| false);
    let mut command_palette_query = use_signal(String::new);
    let mut context_menu_open = use_signal(|| false);
    let mut context_menu_x = use_signal(|| 0.0_f32);
    let mut context_menu_y = use_signal(|| 0.0_f32);
    let mut canvas_origin = use_signal(|| (0.0_f32, 0.0_f32));

    use_effect(move || {
        let wf = workflow.read();
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

    let nodes = use_memo(move || workflow.read().nodes.clone());
    let nodes_by_id = use_memo(move || {
        workflow
            .read()
            .nodes
            .iter()
            .cloned()
            .map(|node| (node.id, node))
            .collect::<std::collections::HashMap<_, _>>()
    });
    let connections = use_memo(move || workflow.read().connections.clone());
    let node_count = use_memo(move || workflow.read().nodes.len());
    let edge_count = use_memo(move || workflow.read().connections.len());
    let zoom_label = use_memo(move || format!("{:.0}%", workflow.read().viewport.zoom * 100.0));
    let can_undo = use_memo(move || !undo_stack.read().is_empty());
    let can_redo = use_memo(move || !redo_stack.read().is_empty());
    let vp = use_memo(move || workflow.read().viewport.clone());

    let temp_edge = use_memo(move || {
        let conn = connecting_from.read();
        let to = *temp_edge_to.read();

        match (conn.as_ref(), to) {
            (Some((id, handle_type)), Some(to_pos)) => {
                let node = nodes_by_id.read().get(id).cloned();
                node.map(|n| {
                    let from = if handle_type == "source" {
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
                    (from, to_pos)
                })
            }
            _ => None,
        }
    });

    let vx = vp.read().x;
    let vy = vp.read().y;
    let vz = vp.read().zoom;
    let canvas_cursor = use_memo(move || {
        if *is_panning.read() {
            "cursor-grabbing"
        } else if *is_space_hand_tool.read() {
            "cursor-grab"
        } else {
            "cursor-default"
        }
    });

    rsx! {
        script { src: "https://cdn.tailwindcss.com" }
        style {
            "@keyframes dash {{ to {{ stroke-dashoffset: -16; }} }}"
            "@keyframes slide-in-right {{ from {{ transform: translateX(24px); opacity: 0; }} to {{ transform: translateX(0); opacity: 1; }} }}"
            ".animate-slide-in-right {{ animation: slide-in-right 0.22s ease-out; }}"
        }

        div { class: "relative flex h-screen w-screen flex-col overflow-hidden bg-[#f4f6fb] text-slate-900 [font-family:'Geist',_'Inter',sans-serif] select-none",
            FlowToolbar {
                workflow_name: workflow_name,
                on_workflow_name_change: move |value| workflow_name.set(value),
                node_count: node_count,
                edge_count: edge_count,
                zoom_label: zoom_label,
                can_undo: can_undo,
                can_redo: can_redo,
                on_zoom_in: move |_| workflow.write().zoom(0.12, 640.0, 400.0),
                on_zoom_out: move |_| workflow.write().zoom(-0.12, 640.0, 400.0),
                on_fit_view: move |_| workflow.write().fit_view(1280.0, 760.0, 200.0),
                on_layout: move |_| {
                    let snapshot = workflow.read().clone();
                    undo_stack.write().push(snapshot);
                    redo_stack.write().clear();
                    workflow.write().apply_layout();
                },
                on_execute: move |_| {
                    spawn(async move {
                        workflow.write().run().await;
                    });
                },
                on_undo: move |_| {
                    let previous = undo_stack.write().pop();
                    if let Some(snapshot) = previous {
                        let current = workflow.read().clone();
                        redo_stack.write().push(current);
                        workflow.set(snapshot);
                        selected_node_id.set(None);
                        selected_node_ids.set(Vec::new());
                    }
                },
                on_redo: move |_| {
                    let next = redo_stack.write().pop();
                    if let Some(snapshot) = next {
                        let current = workflow.read().clone();
                        undo_stack.write().push(current);
                        workflow.set(snapshot);
                        selected_node_id.set(None);
                        selected_node_ids.set(Vec::new());
                    }
                },
                on_save: move |_| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        crate::ui::app_io::download_workflow_json(
                            &workflow_name.read(),
                            &workflow.read(),
                        );
                    }
                },
                on_settings: move |_| {
                    let is_open = *show_settings.read();
                    show_settings.set(!is_open);
                }
            }

            if *show_settings.read() {
                div { class: "absolute right-4 top-14 z-40 w-[280px] rounded-lg border border-slate-700 bg-slate-900/95 p-3 shadow-2xl shadow-slate-950/70 backdrop-blur",
                    div { class: "mb-2 flex items-center justify-between",
                        h4 { class: "text-[12px] font-semibold text-slate-100", "Workflow Settings" }
                        button {
                            class: "flex h-6 w-6 items-center justify-center rounded-md text-slate-500 transition-colors hover:bg-slate-800 hover:text-slate-100",
                            onclick: move |_| show_settings.set(false),
                            crate::ui::icons::XIcon { class: "h-3.5 w-3.5" }
                        }
                    }
                    p { class: "mb-3 text-[11px] leading-relaxed text-slate-400", "Use Save to export the current workflow as JSON. Undo and Redo track recent graph edits." }
                    div { class: "flex items-center gap-2",
                        button {
                            class: "flex h-8 flex-1 items-center justify-center rounded-md border border-slate-700 text-[12px] text-slate-300 transition-colors hover:bg-slate-800 hover:text-slate-100",
                            onclick: move |_| show_settings.set(false),
                            "Close"
                        }
                    }
                }
            }

            NodeCommandPalette {
                open: show_command_palette,
                query: command_palette_query,
                on_query_change: move |value| command_palette_query.set(value),
                on_close: move |()| {
                    show_command_palette.set(false);
                },
                on_pick: move |node_type| {
                    let snapshot = workflow.read().clone();
                    undo_stack.write().push(snapshot);
                    if undo_stack.read().len() > 60 {
                        let _ = undo_stack.write().remove(0);
                    }
                    redo_stack.write().clear();
                    workflow.write().add_node_at_viewport_center(node_type);
                    show_command_palette.set(false);
                    command_palette_query.set(String::new());
                }
            }

            CanvasContextMenu {
                open: context_menu_open,
                x: context_menu_x,
                y: context_menu_y,
                on_close: move |_| context_menu_open.set(false),
                on_add_node: move |_| {
                    context_menu_open.set(false);
                    show_command_palette.set(true);
                },
                on_fit_view: move |_| {
                    context_menu_open.set(false);
                    workflow.write().fit_view(1280.0, 760.0, 200.0);
                },
                on_layout: move |_| {
                    context_menu_open.set(false);
                    let snapshot = workflow.read().clone();
                    undo_stack.write().push(snapshot);
                    if undo_stack.read().len() > 60 {
                        let _ = undo_stack.write().remove(0);
                    }
                    redo_stack.write().clear();
                    workflow.write().apply_layout();
                }
            }

            div { class: "flex flex-1 overflow-hidden",
                NodeSidebar {
                    search: sidebar_search,
                    on_search_change: move |value| sidebar_search.set(value),
                    on_pickup_node: move |node_type: &'static str| {
                        pending_sidebar_drop.set(Some(node_type.to_string()));
                    },
                    on_add_node: move |node_type: &'static str| {
                        let snapshot = workflow.read().clone();
                        undo_stack.write().push(snapshot);
                        if undo_stack.read().len() > 60 {
                            let _ = undo_stack.write().remove(0);
                        }
                        redo_stack.write().clear();
                        pending_sidebar_drop.set(None);
                        workflow.write().add_node_at_viewport_center(node_type);
                    }
                }

                main {
                    class: "relative flex-1 overflow-hidden bg-[#f8fafc] {canvas_cursor}",
                    tabindex: "0",
                    onmouseenter: move |evt| {
                        let page = evt.page_coordinates();
                        let element = evt.element_coordinates();
                        #[allow(clippy::cast_possible_truncation)]
                        let origin_x = page.x as f32 - element.x as f32;
                        #[allow(clippy::cast_possible_truncation)]
                        let origin_y = page.y as f32 - element.y as f32;
                        canvas_origin.set((origin_x, origin_y));
                    },
                    oncontextmenu: move |evt| {
                        evt.prevent_default();
                        let coordinates = evt.page_coordinates();
                        #[allow(clippy::cast_possible_truncation)]
                        context_menu_x.set(coordinates.x as f32);
                        #[allow(clippy::cast_possible_truncation)]
                        context_menu_y.set(coordinates.y as f32);
                        context_menu_open.set(true);
                        show_command_palette.set(false);
                    },
                    onkeydown: move |evt| {
                        let key = evt.key().to_string().to_lowercase();

                        if key == " " || key == "space" {
                            evt.prevent_default();
                            is_space_hand_tool.set(true);
                            return;
                        }

                        if key == "escape" {
                            evt.prevent_default();
                            show_command_palette.set(false);
                            context_menu_open.set(false);
                            connecting_from.set(None);
                            temp_edge_to.set(None);
                            hovered_handle.set(None);
                            return;
                        }

                        if key == "k" {
                            evt.prevent_default();
                            context_menu_open.set(false);
                            let is_open = *show_command_palette.read();
                            show_command_palette.set(!is_open);
                            if !is_open {
                                command_palette_query.set(String::new());
                            }
                            return;
                        }

                        if key == "+" || key == "=" || key == "add" {
                            evt.prevent_default();
                            workflow.write().zoom(0.12, 640.0, 400.0);
                            return;
                        }

                        if key == "-" || key == "_" || key == "subtract" {
                            evt.prevent_default();
                            workflow.write().zoom(-0.12, 640.0, 400.0);
                            return;
                        }

                        if key == "0" {
                            evt.prevent_default();
                            workflow.write().fit_view(1280.0, 760.0, 200.0);
                            return;
                        }

                        if key == "z" {
                            evt.prevent_default();
                            let previous = undo_stack.write().pop();
                            if let Some(snapshot) = previous {
                                let current = workflow.read().clone();
                                redo_stack.write().push(current);
                                workflow.set(snapshot);
                                selected_node_id.set(None);
                                selected_node_ids.set(Vec::new());
                            }
                            return;
                        }

                        if key == "y" {
                            evt.prevent_default();
                            let next = redo_stack.write().pop();
                            if let Some(snapshot) = next {
                                let current = workflow.read().clone();
                                undo_stack.write().push(current);
                                workflow.set(snapshot);
                                selected_node_id.set(None);
                                selected_node_ids.set(Vec::new());
                            }
                            return;
                        }

                        if key == "backspace" || key == "delete" {
                            let mut ids = selected_node_ids.read().clone();
                            if ids.is_empty() {
                                if let Some(node_id) = *selected_node_id.read() {
                                    ids.push(node_id);
                                }
                            }
                            if ids.is_empty() {
                                return;
                            }

                            evt.prevent_default();
                            let snapshot = workflow.read().clone();
                            undo_stack.write().push(snapshot);
                            if undo_stack.read().len() > 60 {
                                let _ = undo_stack.write().remove(0);
                            }
                            redo_stack.write().clear();

                            let mut wf = workflow.write();
                            for node_id in ids {
                                wf.remove_node(node_id);
                            }
                            selected_node_id.set(None);
                            selected_node_ids.set(Vec::new());
                        }
                    },
                    onkeyup: move |evt| {
                        let key = evt.key().to_string().to_lowercase();
                        if key == " " || key == "space" {
                            evt.prevent_default();
                            is_space_hand_tool.set(false);
                            is_panning.set(false);
                        }
                    },
                    onwheel: move |evt| {
                        evt.prevent_default();
                        let page = evt.page_coordinates();
                        let (origin_x, origin_y) = *canvas_origin.read();
                        #[allow(clippy::cast_possible_truncation)]
                        let delta = -evt.delta().strip_units().y as f32 * 0.001;
                        #[allow(clippy::cast_possible_truncation)]
                        let zoom_x = page.x as f32 - origin_x;
                        #[allow(clippy::cast_possible_truncation)]
                        let zoom_y = page.y as f32 - origin_y;
                        workflow.write().zoom(delta, zoom_x, zoom_y);
                    },
                    onmousemove: move |evt| {
                        let page = evt.page_coordinates();
                        let (origin_x, origin_y) = *canvas_origin.read();
                        #[allow(clippy::cast_possible_truncation)]
                        let (mx, my) = (page.x as f32 - origin_x, page.y as f32 - origin_y);
                        let (lx, ly) = *last_mouse_pos.read();
                        let dx = mx - lx;
                        let dy = my - ly;
                        last_mouse_pos.set((mx, my));

                        let current_vp = workflow.read().viewport.clone();
                        let zoom = current_vp.zoom;

                        if !dragging_node_ids.read().is_empty() {
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
                                let mut wf = workflow.write();
                                wf.viewport.x += pan_x;
                                wf.viewport.y += pan_y;
                            }

                            let node_dx = (dx - pan_x) / zoom;
                            let node_dy = (dy - pan_y) / zoom;
                            for node_id in dragging_node_ids.read().iter().copied() {
                                workflow.write().update_node_position(node_id, node_dx, node_dy);
                            }
                        } else if connecting_from.read().is_some() {
                            let canvas_x = (mx - current_vp.x) / zoom;
                            let canvas_y = (my - current_vp.y) / zoom;

                            if let Some((source_id, source_kind)) = connecting_from.read().clone() {
                                let node_list = workflow.read().nodes.clone();
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
                                    hovered_handle.set(Some((node_id, handle_kind)));
                                    temp_edge_to.set(Some(snapped_pos));
                                } else {
                                    hovered_handle.set(None);
                                    temp_edge_to.set(Some(FlowPosition {
                                        x: canvas_x,
                                        y: canvas_y,
                                    }));
                                }
                            }
                        } else if let Some(start) = *marquee_start.read() {
                            marquee_current.set(Some((mx, my)));
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
                                .read()
                                .nodes
                                .iter()
                                .filter(|node| {
                                    crate::ui::editor_interactions::node_intersects_rect(
                                        node.x, node.y, rect,
                                    )
                                })
                                .map(|node| node.id)
                                .collect::<Vec<_>>();
                            selected_node_id.set(selected.first().copied());
                            selected_node_ids.set(selected);
                        } else if *is_panning.read() {
                            let mut wf = workflow.write();
                            wf.viewport.x += dx;
                            wf.viewport.y += dy;
                        }
                    },
                    onmouseup: move |evt| {
                        let from = connecting_from.read().clone();
                        let over = hovered_handle.read().clone();
                        let is_dragging = !dragging_node_ids.read().is_empty();
                        let pending_drop = pending_sidebar_drop.read().clone();
                        let mut should_clear_selection = false;

                        if let (Some((src_id, src_handle)), Some((tgt_id, _))) = (from, over) {
                            if src_id != tgt_id {
                                let (source, target) = if src_handle == "source" {
                                    (src_id, tgt_id)
                                } else {
                                    (tgt_id, src_id)
                                };

                                let snapshot = workflow.read().clone();
                                undo_stack.write().push(snapshot);
                                if undo_stack.read().len() > 60 {
                                    let _ = undo_stack.write().remove(0);
                                }
                                redo_stack.write().clear();

                                workflow
                                    .write()
                                    .add_connection(source, target, &PortName("main".to_string()), &PortName("main".to_string()));
                            }
                        } else if !is_dragging && marquee_start.read().is_none() {
                            if let Some(node_type) = pending_drop {
                                let coords = evt.page_coordinates();
                                let (origin_x, origin_y) = *canvas_origin.read();
                                #[allow(clippy::cast_possible_truncation)]
                                let mx = coords.x as f32 - origin_x;
                                #[allow(clippy::cast_possible_truncation)]
                                let my = coords.y as f32 - origin_y;
                                let current_vp = workflow.read().viewport.clone();
                                let canvas_x = (mx - current_vp.x) / current_vp.zoom - 110.0;
                                let canvas_y = (my - current_vp.y) / current_vp.zoom - 34.0;

                                let snapshot = workflow.read().clone();
                                undo_stack.write().push(snapshot);
                                if undo_stack.read().len() > 60 {
                                    let _ = undo_stack.write().remove(0);
                                }
                                redo_stack.write().clear();

                                workflow.write().add_node(&node_type, canvas_x, canvas_y);
                            }
                        }

                        if let (Some(start), Some(end)) = (*marquee_start.read(), *marquee_current.read()) {
                            let rect = crate::ui::editor_interactions::normalize_rect(start, end);
                            let tiny_click = (rect.2 - rect.0).abs() < 2.0 && (rect.3 - rect.1).abs() < 2.0;
                            if tiny_click
                                && crate::ui::editor_interactions::rect_contains(rect, start)
                                && !is_dragging
                            {
                                should_clear_selection = true;
                            }
                        }

                        dragging_node_ids.set(Vec::new());
                        connecting_from.set(None);
                        temp_edge_to.set(None);
                        pending_sidebar_drop.set(None);
                        is_panning.set(false);
                        hovered_handle.set(None);
                        marquee_start.set(None);
                        marquee_current.set(None);

                        if should_clear_selection {
                            selected_node_id.set(None);
                            selected_node_ids.set(Vec::new());
                        }
                    },
                    onmouseleave: move |_| {
                        let is_active_interaction = !dragging_node_ids.read().is_empty()
                            || *is_panning.read()
                            || marquee_start.read().is_some()
                            || connecting_from.read().is_some();
                        if is_active_interaction {
                            return;
                        }
                        dragging_node_ids.set(Vec::new());
                        connecting_from.set(None);
                        temp_edge_to.set(None);
                        pending_sidebar_drop.set(None);
                        is_panning.set(false);
                        hovered_handle.set(None);
                        marquee_start.set(None);
                        marquee_current.set(None);
                    },
                    onmousedown: move |evt| {
                        context_menu_open.set(false);
                        let trigger_button = evt.trigger_button();
                        if matches!(trigger_button, Some(MouseButton::Primary | MouseButton::Auxiliary)) {
                            evt.prevent_default();
                            let page = evt.page_coordinates();
                            let coordinates = evt.element_coordinates();
                            #[allow(clippy::cast_possible_truncation)]
                            let origin_x = page.x as f32 - coordinates.x as f32;
                            #[allow(clippy::cast_possible_truncation)]
                            let origin_y = page.y as f32 - coordinates.y as f32;
                            canvas_origin.set((origin_x, origin_y));
                            #[allow(clippy::cast_possible_truncation)]
                            let mouse_pos = (coordinates.x as f32, coordinates.y as f32);
                            last_mouse_pos.set(mouse_pos);

                            let has_pending_drop = pending_sidebar_drop.read().is_some();
                            if matches!(trigger_button, Some(MouseButton::Auxiliary))
                                || (matches!(trigger_button, Some(MouseButton::Primary))
                                    && *is_space_hand_tool.read())
                            {
                                is_panning.set(true);
                            } else if matches!(trigger_button, Some(MouseButton::Primary))
                                && !has_pending_drop
                            {
                                marquee_start.set(Some(mouse_pos));
                                marquee_current.set(Some(mouse_pos));
                            }
                        } else {
                            selected_node_id.set(None);
                            selected_node_ids.set(Vec::new());
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
                                let selected_ids = selected_node_ids.read().clone();
                                let is_selected = selected_ids.contains(&node_id);

                                rsx! {
                                    FlowNodeComponent {
                                        key: "{node_id}",
                                        node,
                                        selected: is_selected,
                                        on_mouse_down: move |_| {
                                            let snapshot = workflow.read().clone();
                                            undo_stack.write().push(snapshot);
                                            if undo_stack.read().len() > 60 {
                                                let _ = undo_stack.write().remove(0);
                                            }
                                            redo_stack.write().clear();

                                            let currently_selected = selected_node_ids.read().clone();
                                            let drag_targets = if currently_selected.contains(&node_id) {
                                                if currently_selected.is_empty() {
                                                    vec![node_id]
                                                } else {
                                                    currently_selected
                                                }
                                            } else {
                                                vec![node_id]
                                            };
                                            dragging_node_ids.set(drag_targets);

                                            if !selected_node_ids.read().contains(&node_id) {
                                                selected_node_ids.set(vec![node_id]);
                                            }
                                            selected_node_id.set(Some(node_id));
                                        },
                                        on_click: move |_| {
                                            selected_node_id.set(Some(node_id));
                                            selected_node_ids.set(vec![node_id]);
                                        },
                                        on_handle_mouse_down: move |args: (MouseEvent, String)| {
                                            let (evt, handle_type) = args;
                                            connecting_from.set(Some((node_id, handle_type.clone())));
                                            hovered_handle.set(Some((node_id, handle_type)));
                                            selected_node_id.set(Some(node_id));
                                            selected_node_ids.set(vec![node_id]);
                                            let current_vp = workflow.read().viewport.clone();
                                            let coord = evt.page_coordinates();
                                            let (origin_x, origin_y) = *canvas_origin.read();
                                            #[allow(clippy::cast_possible_truncation)]
                                            let local_x = coord.x as f32 - origin_x;
                                            #[allow(clippy::cast_possible_truncation)]
                                            let local_y = coord.y as f32 - origin_y;
                                            let canvas_x = (local_x - current_vp.x) / current_vp.zoom;
                                            #[allow(clippy::cast_possible_truncation)]
                                            let canvas_y = (local_y - current_vp.y) / current_vp.zoom;
                                            temp_edge_to.set(Some(FlowPosition { x: canvas_x, y: canvas_y }));
                                        },
                                        on_handle_mouse_enter: move |handle_type| hovered_handle.set(Some((node_id, handle_type))),
                                        on_handle_mouse_leave: move |()| hovered_handle.set(None)
                                    }
                                }
                            }
                        }
                    }

                    if let (Some(start), Some(end)) = (*marquee_start.read(), *marquee_current.read()) {
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
                        selected_node_id: selected_node_id
                    }
                }

                SelectedNodePanel {
                    selected_node_id,
                    selected_node_ids,
                    nodes_by_id,
                    workflow,
                    undo_stack,
                    redo_stack,
                }
            }
        }
    }
}

fn main() {
    launch(App);
}
