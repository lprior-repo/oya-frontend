use super::{get_str_val, get_u64_val};
use crate::ui::icons::icon_by_name;
use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub(super) fn ExecutionTab(config: Value) -> Element {
    let status = get_str_val(&config, "status");
    let is_executed = !status.is_empty();

    let journal_idx = get_u64_val(&config, "journalIndex");
    let retry_count = get_u64_val(&config, "retryCount");

    rsx! {
        div { class: "flex flex-col gap-4",
            div { class: "flex flex-col gap-2",
                label { class: "text-[11px] font-medium uppercase tracking-wide text-slate-500", "Invocation Status" }
                if is_executed {
                    {
                        let (bg_color, text_color, border_color, icon_name, is_spin) = match status.as_str() {
                            "running" => ("bg-indigo-500/15", "text-indigo-400", "border-indigo-500/30", "loader", true),
                            "suspended" => ("bg-pink-500/15", "text-pink-400", "border-pink-500/30", "pause", false),
                            "completed" => ("bg-emerald-500/15", "text-emerald-400", "border-emerald-500/30", "check-circle", false),
                            "failed" => ("bg-red-500/15", "text-red-400", "border-red-500/30", "alert-circle", false),
                            "retrying" => ("bg-amber-500/15", "text-amber-400", "border-amber-500/30", "refresh", true),
                            _ => ("bg-slate-500/15", "text-slate-400", "border-slate-500/30", "help-circle", false),
                        };
                        let label = match status.as_str() {
                            "running" => "Running",
                            "suspended" => "Suspended",
                            "completed" => "Completed",
                            "failed" => "Failed",
                            "retrying" => "Retrying",
                            other => other,
                        };
                        let icon_class = if is_spin { "h-3 w-3 animate-spin".to_string() } else { "h-3 w-3".to_string() };
                        rsx! {
                            div {
                                class: "inline-flex self-start items-center gap-1.5 rounded-md border px-2.5 py-1 text-[11px] font-medium {bg_color} {text_color} {border_color}",
                                {icon_by_name(icon_name, icon_class)}
                                "{label}"
                            }
                        }
                    }
                } else {
                    span { class: "text-[11px] text-slate-500", "Not yet executed" }
                }
            }

            if let Some(idx) = journal_idx {
                div { class: "flex flex-col gap-1",
                    label { class: "text-[11px] font-medium uppercase tracking-wide text-slate-500", "Journal Entry" }
                    div { class: "flex items-center gap-2",
                        span { class: "rounded bg-slate-800 px-2 py-0.5 font-mono text-[11px] text-slate-300", "#{idx}" }
                        span { class: "text-[10px] text-slate-500", "Position in durable execution log" }
                    }
                }
            }

            if let Some(count) = retry_count {
                if count > 0 {
                    div { class: "flex flex-col gap-1",
                        label { class: "text-[11px] font-medium uppercase tracking-wide text-slate-500", "Retry Attempts" }
                        div { class: "flex items-center gap-2",
                            span { class: "rounded bg-red-500/10 px-2 py-0.5 font-mono text-[11px] text-red-400", "{count}" }
                            span { class: "text-[10px] text-slate-500", "Times retried before success/failure" }
                        }
                    }
                }
            }

            div { class: "h-px bg-slate-800" }
            div { class: "rounded-lg border border-dashed border-slate-700 bg-slate-800/50 p-3",
                p { class: "text-[11px] leading-relaxed text-slate-400", "Restate persists each step in a durable journal. On failure, execution replays from the journal, skipping already-completed steps. This ensures exactly-once semantics." }
            }
        }
    }
}
