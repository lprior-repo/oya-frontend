use dioxus::prelude::*;
use oya_frontend::graph::{Connection, Node};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

const NODE_WIDTH: f32 = 220.0;
const NODE_HEIGHT: f32 = 68.0;
const BEND_CLAMP: f32 = 200.0;

#[derive(Clone, Copy, PartialEq)]
struct EdgeAnchor {
    from: Position,
    to: Position,
}

#[derive(Clone)]
struct DragState {
    edge_id: String,
    start_y: f32,
    start_bend: f32,
}

fn get_source_point(node: &Node) -> Position {
    Position {
        x: node.x + NODE_WIDTH / 2.0,
        y: node.y + NODE_HEIGHT,
    }
}

fn get_target_point(node: &Node) -> Position {
    Position {
        x: node.x + NODE_WIDTH / 2.0,
        y: node.y,
    }
}

fn create_smooth_step_path(from: Position, to: Position, bend_y: f32) -> (String, Position) {
    let mid_y = f32::midpoint(from.y, to.y) + bend_y.clamp(-BEND_CLAMP, BEND_CLAMP);
    let radius: f32 = 8.0;

    let dx = to.x - from.x;
    let dy = to.y - from.y;

    if dx.abs() < 2.0 {
        return (
            format!("M {} {} L {} {}", from.x, from.y, to.x, to.y),
            Position {
                x: f32::midpoint(from.x, to.x),
                y: mid_y,
            },
        );
    }

    let sign_x = if dx > 0.0 { 1.0 } else { -1.0 };
    let r = radius.min(dx.abs() / 2.0).min(dy.abs() / 4.0);

    (
        format!(
            "M {fx} {fy} L {fx} {my_r} Q {fx} {my} {fx_r} {my} L {tx_r} {my} Q {tx} {my} {tx} {my_r2} L {tx} {ty}",
            fx = from.x,
            fy = from.y,
            my = mid_y,
            my_r = mid_y - r,
            my_r2 = mid_y + r,
            fx_r = from.x + sign_x * r,
            tx_r = to.x - sign_x * r,
            tx = to.x,
            ty = to.y
        ),
        Position {
            x: f32::midpoint(from.x, to.x),
            y: mid_y,
        },
    )
}

fn resolve_edge_anchors(edges: &[Connection], nodes: &[Node]) -> HashMap<String, EdgeAnchor> {
    let node_by_id: HashMap<_, _> = nodes.iter().map(|node| (node.id, node)).collect();

    edges
        .iter()
        .filter_map(|edge| {
            let source = node_by_id.get(&edge.source)?;
            let target = node_by_id.get(&edge.target)?;
            let from = get_source_point(source);
            let to = get_target_point(target);
            Some((edge.id.to_string(), EdgeAnchor { from, to }))
        })
        .collect()
}

#[component]
pub fn FlowEdges(
    edges: ReadSignal<Vec<Connection>>,
    nodes: ReadSignal<Vec<Node>>,
    temp_edge: ReadSignal<Option<(Position, Position)>>,
) -> Element {
    let mut hovered_edge = use_signal(|| None::<String>);
    let mut bend_offsets = use_signal(HashMap::<String, f32>::new);
    let mut drag_state = use_signal(|| None::<DragState>);

    let edge_anchors = use_memo(move || {
        let node_list = nodes.read();
        let edge_list = edges.read();
        resolve_edge_anchors(&edge_list, &node_list)
    });

    let temp_path = use_memo(move || {
        (*temp_edge.read()).map(|(from, to)| create_smooth_step_path(from, to, 0.0).0)
    });

    let svg_pointer_class = if drag_state.read().is_some() {
        "pointer-events-auto"
    } else {
        "pointer-events-none"
    };

    rsx! {
        svg {
            class: "absolute inset-0 overflow-visible {svg_pointer_class}",
            style: "width: 100%; height: 100%; z-index: 0;",
            onmousemove: move |evt| {
                if let Some(state) = drag_state.read().clone() {
                    let coordinates = evt.page_coordinates();
                    #[allow(clippy::cast_possible_truncation)]
                    let page_y = coordinates.y as f32;
                    let next_bend = (state.start_bend + (page_y - state.start_y)).clamp(-BEND_CLAMP, BEND_CLAMP);
                    bend_offsets.write().insert(state.edge_id, next_bend);
                }
            },
            onmouseup: move |_| {
                drag_state.set(None);
            },
            onmouseleave: move |_| {
                drag_state.set(None);
            },
            defs {
                marker {
                    id: "arrowhead",
                    marker_width: "10",
                    marker_height: "8",
                    ref_x: "9",
                    ref_y: "4",
                    orient: "auto",
                    path {
                        d: "M 0 0 L 10 4 L 0 8 z",
                        class: "fill-slate-600"
                    }
                }
                marker {
                    id: "arrowhead-active",
                    marker_width: "10",
                    marker_height: "8",
                    ref_x: "9",
                    ref_y: "4",
                    orient: "auto",
                    path {
                        d: "M 0 0 L 10 4 L 0 8 z",
                        class: "fill-indigo-500"
                    }
                }
            }

            for edge in edges.read().iter() {
                {
                    let edge_id = edge.id.to_string();
                    let anchor = edge_anchors.read().get(&edge_id).copied();

                    if let Some(anchor) = anchor {
                        let bend = bend_offsets.read().get(&edge_id).copied().unwrap_or(0.0);
                        let (path, midpoint) = create_smooth_step_path(anchor.from, anchor.to, bend);
                        let dragging_this = drag_state
                            .read()
                            .as_ref()
                            .map(|state| state.edge_id == edge_id)
                            .unwrap_or(false);
                        let hovered_this = hovered_edge
                            .read()
                            .as_ref()
                            .map(|id| id == &edge_id)
                            .unwrap_or(false);
                        let handle_opacity = if hovered_this || dragging_this { "1" } else { "0" };

                        rsx! {
                            g { key: "{edge_id}",
                                path {
                                    d: "{path}",
                                    fill: "none",
                                    stroke: "transparent",
                                    stroke_width: "16",
                                    pointer_events: "stroke",
                                    class: "pointer-events-auto",
                                    onmouseenter: {
                                        let edge_id = edge_id.clone();
                                        move |_| hovered_edge.set(Some(edge_id.clone()))
                                    },
                                    onmouseleave: {
                                        let edge_id = edge_id.clone();
                                        move |_| {
                                            let is_dragging = drag_state
                                                .read()
                                                .as_ref()
                                                .map(|state| state.edge_id == edge_id)
                                                .unwrap_or(false);
                                            if !is_dragging {
                                                hovered_edge.set(None);
                                            }
                                        }
                                    }
                                }
                                path {
                                    d: "{path}",
                                    fill: "none",
                                    stroke: "rgba(71, 85, 105, 0.75)",
                                    stroke_width: "2",
                                    marker_end: "url(#arrowhead)"
                                }
                                circle {
                                    cx: "{midpoint.x}",
                                    cy: "{midpoint.y}",
                                    r: "5",
                                    fill: "rgba(99, 102, 241, 0.95)",
                                    stroke: "rgba(226, 232, 240, 0.95)",
                                    stroke_width: "1.5",
                                    opacity: "{handle_opacity}",
                                    class: "pointer-events-auto cursor-ns-resize transition-opacity duration-100",
                                    onmousedown: {
                                        let edge_id = edge_id.clone();
                                        move |evt| {
                                            evt.stop_propagation();
                                            let coordinates = evt.page_coordinates();
                                            #[allow(clippy::cast_possible_truncation)]
                                            let page_y = coordinates.y as f32;
                                            let current_bend = bend_offsets.read().get(&edge_id).copied().unwrap_or(0.0);
                                            drag_state.set(Some(DragState {
                                                edge_id: edge_id.clone(),
                                                start_y: page_y,
                                                start_bend: current_bend,
                                            }));
                                            hovered_edge.set(Some(edge_id.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }

            if let Some(path) = temp_path.read().as_ref() {
                path {
                    d: "{path}",
                    fill: "none",
                    stroke: "rgba(99, 102, 241, 0.6)",
                    stroke_width: "2",
                    stroke_dasharray: "6 4"
                }
            }
        }
    }
}
