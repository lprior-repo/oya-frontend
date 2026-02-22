#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use dioxus::prelude::*;
use oya_frontend::graph::{NodeId, RunRecord};
use std::collections::HashMap;

const fn status_badge_classes(success: bool) -> &'static str {
    if success {
        "bg-emerald-50 text-emerald-700 border-emerald-200"
    } else {
        "bg-red-50 text-red-700 border-red-200"
    }
}

const fn status_icon_classes(success: bool) -> &'static str {
    if success {
        "h-3 w-3 text-emerald-500"
    } else {
        "h-3 w-3 text-red-500"
    }
}

fn format_timestamp(ts: &chrono::DateTime<chrono::Utc>) -> String {
    ts.format("%H:%M:%S").to_string()
}

fn format_elapsed(ts: &chrono::DateTime<chrono::Utc>) -> String {
    let elapsed = chrono::Utc::now().signed_duration_since(*ts);
    if elapsed.num_minutes() < 1 {
        "just now".to_string()
    } else if elapsed.num_hours() < 1 {
        format!("{}m ago", elapsed.num_minutes())
    } else if elapsed.num_days() < 1 {
        format!("{}h ago", elapsed.num_hours())
    } else {
        format!("{}d ago", elapsed.num_days())
    }
}

fn panel_height_class(collapsed: bool) -> &'static str {
    if collapsed {
        "h-10"
    } else {
        "h-[280px]"
    }
}

fn chevron_rotation_class(collapsed: bool) -> &'static str {
    if collapsed {
        "-rotate-90"
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::{format_elapsed, status_badge_classes};

    #[test]
    fn given_recent_timestamp_when_formatting_elapsed_then_it_returns_just_now() {
        let timestamp = chrono::Utc::now() - chrono::Duration::seconds(30);
        assert_eq!(format_elapsed(&timestamp), "just now");
    }

    #[test]
    fn given_hour_old_timestamp_when_formatting_elapsed_then_it_returns_hours_ago() {
        let timestamp = chrono::Utc::now() - chrono::Duration::hours(2);
        assert_eq!(format_elapsed(&timestamp), "2h ago");
    }

    #[test]
    fn given_success_status_when_requesting_badge_classes_then_success_classes_are_returned() {
        assert_eq!(
            status_badge_classes(true),
            "bg-emerald-50 text-emerald-700 border-emerald-200"
        );
    }
}

#[component]
pub fn ExecutionHistoryPanel(
    history: Memo<Vec<RunRecord>>,
    nodes_by_id: ReadSignal<HashMap<NodeId, oya_frontend::graph::Node>>,
    on_select_node: EventHandler<NodeId>,
    collapsed: Signal<bool>,
) -> Element {
    let mut expanded_runs: Signal<std::collections::HashSet<uuid::Uuid>> =
        use_signal(std::collections::HashSet::new);
    let history_len = history.read().len();
    let is_collapsed = *collapsed.read();
    let height_class = panel_height_class(is_collapsed);
    let chevron_class = chevron_rotation_class(is_collapsed);

    rsx! {
        aside {
            class: "flex flex-col border-t border-slate-200 bg-white/95 transition-all duration-200 {height_class}",

            div {
                class: "flex items-center justify-between px-3 py-2 border-b border-slate-100",
                button {
                    class: "flex items-center gap-2 text-slate-700 hover:text-slate-900 transition-colors",
                    onclick: move |_| {
                        let _ = collapsed.try_write().map(|mut c| *c = !*c);
                    },
                    crate::ui::icons::ClockIcon { class: "h-4 w-4 text-slate-500" }
                    span { class: "text-[12px] font-semibold", "Execution History" }
                    span { class: "rounded bg-slate-100 px-1.5 py-0.5 text-[10px] text-slate-600", "{history_len}" }
                    div { class: "transition-transform {chevron_class}",
                        crate::ui::icons::ChevronDownIcon { class: "h-3 w-3 text-slate-400" }
                    }
                }
            }

            if !is_collapsed {
                div { class: "flex-1 overflow-y-auto",
                    if history.read().is_empty() {
                        div { class: "flex flex-col items-center justify-center h-full text-center px-4",
                            crate::ui::icons::ClockIcon { class: "h-8 w-8 text-slate-300 mb-2" }
                            p { class: "text-[12px] text-slate-500", "No executions yet" }
                            p { class: "text-[10px] text-slate-400 mt-1", "Run the workflow to see history" }
                        }
                    } else {
                        div { class: "flex flex-col",
                            for run in history.read().iter().rev() {
                                {
                                    let run_id = run.id;
                                    let is_expanded = expanded_runs.read().contains(&run_id);
                                    let status_class = status_badge_classes(run.success);
                                    let icon_class = status_icon_classes(run.success);
                                    let timestamp_str = format_timestamp(&run.timestamp);
                                    let elapsed_str = format_elapsed(&run.timestamp);
                                    let node_count = run.results.len();
                                    let item_chevron_class = chevron_rotation_class(!is_expanded);

                                    rsx! {
                                        div {
                                            class: "border-b border-slate-100 last:border-b-0",
                                            key: "{run_id}",

                                            button {
                                                class: "flex w-full items-center gap-2 px-3 py-2 hover:bg-slate-50 transition-colors",
                                                onclick: move |_| {
                                                    let _ = expanded_runs.try_write().map(|mut set| {
                                                        if set.contains(&run_id) {
                                                            set.remove(&run_id);
                                                        } else {
                                                            set.insert(run_id);
                                                        }
                                                    });
                                                },

                                                div { class: "transition-transform {item_chevron_class}",
                                                    crate::ui::icons::ChevronDownIcon { class: "h-3 w-3 text-slate-400" }
                                                }

                                                div { class: "flex-1 flex items-center gap-2",
                                                    span { class: "font-mono text-[11px] text-slate-600", "{timestamp_str}" }
                                                    span { class: "text-[10px] text-slate-400", "{elapsed_str}" }
                                                }

                                                span { class: "text-[10px] text-slate-500", "{node_count} nodes" }

                                                div { class: "flex items-center gap-1 px-1.5 py-0.5 rounded border {status_class}",
                                                    if run.success {
                                                        crate::ui::icons::CheckIcon { class: "{icon_class}" }
                                                    } else {
                                                        crate::ui::icons::XCircleIcon { class: "{icon_class}" }
                                                    }
                                                    span { class: "text-[10px] font-medium",
                                                        if run.success { "Success" } else { "Failed" }
                                                    }
                                                }
                                            }

                                            if is_expanded {
                                                div { class: "bg-slate-50/50 px-3 pb-2",
                                                    for (node_id, result) in run.results.iter() {
                                                        {
                                                            let node_name = nodes_by_id
                                                                .read()
                                                                .get(node_id)
                                                                .map_or_else(
                                                                    || "Unknown".to_string(),
                                                                    |n| n.name.clone(),
                                                                );
                                                            let node_id_for_click = *node_id;
                                                            let result_preview = serde_json::to_string(result)
                                                                .unwrap_or_else(|_| "{}".to_string());
                                                            let truncated_result = if result_preview.len() > 30 {
                                                                format!("{}...", &result_preview[..27])
                                                            } else {
                                                                result_preview
                                                            };

                                                            rsx! {
                                                                button {
                                                                    class: "flex w-full items-center gap-2 px-2 py-1.5 rounded hover:bg-white transition-colors text-left",
                                                                    key: "{node_id}",
                                                                    onclick: move |_| {
                                                                        on_select_node.call(node_id_for_click);
                                                                    },

                                                                    div { class: "w-1.5 h-1.5 rounded-full bg-indigo-400 shrink-0" }
                                                                    span { class: "text-[11px] text-slate-700 flex-1 truncate", "{node_name}" }
                                                                    span { class: "text-[10px] font-mono text-slate-400 shrink-0",
                                                                        "{truncated_result}"
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
                        }
                    }
                }
            }
        }
    }
}
