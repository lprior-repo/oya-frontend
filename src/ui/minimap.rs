use dioxus::prelude::*;
use oya_frontend::graph::{Connection, Node, NodeId};
use std::collections::HashMap;

const NODE_WIDTH: f32 = 220.0;
const NODE_HEIGHT: f32 = 68.0;

#[component]
pub fn FlowMinimap(
    nodes: ReadSignal<Vec<Node>>,
    edges: ReadSignal<Vec<Connection>>,
    selected_node_id: ReadSignal<Option<NodeId>>,
) -> Element {
    let node_values = nodes.read().clone();
    let edge_values = edges.read().clone();

    let bounds = node_values
        .iter()
        .fold(None::<(f32, f32, f32, f32)>, |acc, node| {
            let right = node.x + NODE_WIDTH;
            let bottom = node.y + NODE_HEIGHT;

            match acc {
                Some((min_x, min_y, max_x, max_y)) => Some((
                    min_x.min(node.x),
                    min_y.min(node.y),
                    max_x.max(right),
                    max_y.max(bottom),
                )),
                None => Some((node.x, node.y, right, bottom)),
            }
        });

    let (min_x, min_y, max_x, max_y) = bounds.map_or((0.0, 0.0, 800.0, 600.0), |value| value);
    let node_by_id: HashMap<_, _> = node_values.iter().map(|node| (node.id, node)).collect();
    let viewbox = format!(
        "{} {} {} {}",
        min_x - 40.0,
        min_y - 40.0,
        max_x - min_x + 80.0,
        max_y - min_y + 80.0
    );

    rsx! {
        div { class: "pointer-events-none absolute bottom-4 right-4 h-[110px] w-[160px] overflow-hidden rounded-lg border border-slate-700 bg-slate-900/90 shadow-2xl shadow-slate-950/70 backdrop-blur-sm",
            svg { view_box: "{viewbox}", class: "h-full w-full",
                for edge in edge_values {
                    {
                        let source = node_by_id.get(&edge.source).copied();
                        let target = node_by_id.get(&edge.target).copied();

                        match (source, target) {
                            (Some(src), Some(tgt)) => rsx! {
                                line {
                                    key: "{edge.id}",
                                    x1: "{src.x + NODE_WIDTH / 2.0}",
                                    y1: "{src.y + NODE_HEIGHT}",
                                    x2: "{tgt.x + NODE_WIDTH / 2.0}",
                                    y2: "{tgt.y}",
                                    class: "stroke-slate-700",
                                    stroke_width: "3"
                                }
                            },
                            _ => rsx! {},
                        }
                    }
                }

                for node in node_values {
                    {
                        let classes = if selected_node_id.read().is_some_and(|id| id == node.id) {
                            "fill-indigo-500/40 stroke-indigo-400"
                        } else {
                            "fill-slate-800 stroke-slate-700"
                        };

                        rsx! {
                            rect {
                                key: "{node.id}",
                                x: "{node.x}",
                                y: "{node.y}",
                                width: "{NODE_WIDTH}",
                                height: "{NODE_HEIGHT}",
                                rx: "4",
                                class: "{classes}",
                                stroke_width: "2"
                            }
                        }
                    }
                }
            }
        }
    }
}
