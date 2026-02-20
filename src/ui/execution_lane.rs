use dioxus::prelude::*;

#[derive(Clone)]
struct LaneItem {
    node_id: oya_frontend::graph::NodeId,
    name: String,
    description: String,
    journal: Option<u64>,
    status: &'static str,
}

fn lane_status(node: &oya_frontend::graph::Node) -> &'static str {
    if node.executing {
        "running"
    } else if node.error.is_some() {
        "failed"
    } else if node.skipped {
        "skipped"
    } else if node
        .config
        .get("status")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|status| status == "completed")
    {
        "completed"
    } else if node
        .config
        .get("status")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|status| status == "retrying")
    {
        "retrying"
    } else {
        "pending"
    }
}

fn status_chip(status: &str) -> (&'static str, &'static str, &'static str) {
    match status {
        "running" => ("Running", "bg-indigo-400", "text-indigo-200"),
        "failed" => ("Failed", "bg-rose-400", "text-rose-200"),
        "completed" => ("Completed", "bg-emerald-400", "text-emerald-200"),
        "retrying" => ("Retrying", "bg-amber-400", "text-amber-200"),
        "skipped" => ("Skipped", "bg-slate-500", "text-slate-200"),
        _ => ("Pending", "bg-slate-600", "text-slate-300"),
    }
}

fn status_badge(status: &str) -> (&'static str, &'static str) {
    match status {
        "running" => (
            "Running",
            "border-indigo-500/30 bg-indigo-500/15 text-indigo-300",
        ),
        "failed" => ("Failed", "border-rose-500/30 bg-rose-500/15 text-rose-300"),
        "completed" => (
            "Completed",
            "border-emerald-500/30 bg-emerald-500/15 text-emerald-300",
        ),
        "skipped" => (
            "Skipped",
            "border-slate-600/50 bg-slate-700/60 text-slate-300",
        ),
        "retrying" => (
            "Retrying",
            "border-amber-500/30 bg-amber-500/15 text-amber-300",
        ),
        _ => ("Pending", "border-slate-700 bg-slate-800/80 text-slate-300"),
    }
}

#[component]
pub fn FlowExecutionLane(
    nodes: ReadSignal<Vec<oya_frontend::graph::Node>>,
    queue: ReadSignal<Vec<oya_frontend::graph::NodeId>>,
    current_step: ReadSignal<usize>,
    on_jump_to_node: EventHandler<oya_frontend::graph::NodeId>,
) -> Element {
    let node_list = nodes.read().clone();
    let queue_list = queue.read().clone();
    let current = *current_step.read();

    let lane_items = queue_list
        .iter()
        .map(|node_id| {
            let node = node_list.iter().find(|node| node.id == *node_id);
            node.map_or(
                LaneItem {
                    node_id: *node_id,
                    name: "Unknown node".to_string(),
                    description: "No details available".to_string(),
                    journal: None,
                    status: "pending",
                },
                |found| LaneItem {
                    node_id: *node_id,
                    name: found.name.clone(),
                    description: found.description.clone(),
                    journal: found
                        .config
                        .get("journalIndex")
                        .and_then(serde_json::Value::as_u64),
                    status: lane_status(found),
                },
            )
        })
        .collect::<Vec<_>>();

    let completed_count = lane_items
        .iter()
        .filter(|item| item.status == "completed")
        .count();
    let failed_count = lane_items
        .iter()
        .filter(|item| item.status == "failed")
        .count();
    let running_count = lane_items
        .iter()
        .filter(|item| item.status == "running")
        .count();

    let total = queue_list.len();
    let current_display = if total == 0 {
        0
    } else {
        current.saturating_add(1).min(total)
    };

    rsx! {
        section { class: "pointer-events-none absolute inset-x-0 bottom-3 z-30 px-3",
            div { class: "pointer-events-auto mx-auto w-full max-w-7xl rounded-xl border border-slate-700/80 bg-slate-900/95 p-3 shadow-[0_14px_40px_rgba(2,6,23,0.65)] backdrop-blur",
                div { class: "mb-2 flex flex-wrap items-center justify-between gap-2",
                    div { class: "flex items-center gap-2",
                        span { class: "rounded border border-indigo-500/30 bg-indigo-500/10 px-2 py-0.5 text-[10px] font-semibold uppercase tracking-[0.14em] text-indigo-300", "Execution" }
                        span { class: "text-[11px] text-slate-400", "{current_display} of {total} steps" }
                    }
                    div { class: "flex items-center gap-2 text-[10px]",
                        span { class: "rounded border border-slate-700 bg-slate-800/80 px-2 py-0.5 text-slate-300", "Done {completed_count}" }
                        span { class: "rounded border border-indigo-500/30 bg-indigo-500/10 px-2 py-0.5 text-indigo-300", "Running {running_count}" }
                        span { class: "rounded border border-rose-500/30 bg-rose-500/10 px-2 py-0.5 text-rose-300", "Errors {failed_count}" }
                    }
                }

                div { class: "flex items-stretch gap-2 overflow-x-auto pb-1",
                    if lane_items.is_empty() {
                        div { class: "rounded-lg border border-dashed border-slate-700 bg-slate-800/40 px-3 py-2 text-[11px] text-slate-400",
                            "Run the workflow to populate a live step timeline"
                        }
                    }
                    for (index, item) in lane_items.iter().cloned().enumerate() {
                        {
                            let (status_label, status_class) = status_badge(item.status);
                            let (_, dot_class, _) = status_chip(item.status);
                            let is_current = index == current;
                            let item_class = if is_current {
                                "border-indigo-500/60 bg-indigo-500/12 shadow-[0_0_0_1px_rgba(99,102,241,0.35)] ring-1 ring-indigo-500/35"
                            } else {
                                "border-slate-700 bg-slate-800/70 hover:border-slate-500"
                            };

                            rsx! {
                                div { class: "flex items-center gap-2",
                                    if index > 0 {
                                        div { class: "h-[2px] w-6 shrink-0 rounded bg-slate-700" }
                                    }
                                    button {
                                        key: "{item.node_id}",
                                        class: "group flex min-w-[210px] max-w-[280px] flex-col items-start gap-1 rounded-lg border px-2.5 py-2 text-left transition-colors {item_class}",
                                        onclick: move |_| on_jump_to_node.call(item.node_id),
                                        div { class: "flex w-full items-center justify-between gap-2",
                                            div { class: "flex items-center gap-2",
                                                div { class: "flex h-6 w-6 shrink-0 items-center justify-center rounded-full border border-slate-600 bg-slate-800 text-[10px] font-semibold text-slate-200", "{index + 1}" }
                                                span { class: "h-2 w-2 rounded-full {dot_class}" }
                                                span { class: "truncate text-[11px] font-semibold text-slate-100", "{item.name}" }
                                            }
                                            span { class: "shrink-0 rounded-full border px-2 py-0.5 text-[9px] font-medium uppercase tracking-wide {status_class}", "{status_label}" }
                                        }
                                        p { class: "line-clamp-1 text-[10px] text-slate-400", "{item.description}" }
                                        if let Some(journal) = item.journal {
                                            span { class: "text-[10px] font-mono text-slate-500", "journal #{journal}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
