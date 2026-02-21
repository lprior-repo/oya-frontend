#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use crate::hooks::{CanvasInteraction, SelectionState, WorkflowState};
use crate::ui::{FlowEdges, FlowMinimap, FlowNodeComponent};
use dioxus::prelude::*;

#[component]
pub fn Canvas(
    workflow: WorkflowState,
    selection: SelectionState,
    canvas: CanvasInteraction,
) -> Element {
    let vp = workflow.viewport();
    let vx = vp.read().x;
    let vy = vp.read().y;
    let vz = vp.read().zoom;

    let cursor = canvas.cursor_class();

    rsx! {
        main {
            class: "relative flex-1 overflow-hidden bg-[#f8fafc] {cursor}",
            tabindex: "0",

            // Background grid
            div {
                class: "absolute inset-0 pointer-events-none",
                style: "background-image: radial-gradient(circle, rgba(148, 163, 184, 0.5) 1px, transparent 1px); background-size: calc(22px * {vz}) calc(22px * {vz}); background-position: {vx}px {vy}px;",
            }

            // Transformed canvas content
            div {
                class: "absolute origin-top-left",
                style: "transform: translate({vx}px, {vy}px) scale({vz}); will-change: transform;",

                FlowEdges {
                    edges: workflow.connections(),
                    nodes: workflow.nodes(),
                    temp_edge: canvas.temp_edge_to(),
                }

                for node in workflow.nodes().read().iter().cloned() {
                    {
                        let node_id = node.id;
                        let is_selected = selection.is_selected(node_id);
                        let canvas_clone = canvas;

                        rsx! {
                            FlowNodeComponent {
                                key: "{node_id}",
                                node: node.clone(),
                                selected: is_selected,
                                on_mouse_down: move |_| {
                                    canvas_clone.start_drag(node_id, vec![node_id]);
                                },
                                on_click: move |_| {
                                    selection.select_single(node_id);
                                },
                                on_handle_mouse_down: move |(_evt, handle)| {
                                    canvas_clone.start_connect(node_id, handle);
                                },
                                on_handle_mouse_enter: move |handle| {
                                    canvas_clone.set_hovered_handle(Some((node_id, handle)));
                                },
                                on_handle_mouse_leave: move |()| {
                                    canvas_clone.set_hovered_handle(None);
                                }
                            }
                        }
                    }
                }
            }

            FlowMinimap {
                nodes: workflow.nodes(),
                edges: workflow.connections(),
                selected_node_id: selection.selected_id(),
            }
        }
    }
}
