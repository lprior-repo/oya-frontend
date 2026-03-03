#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Restate Invocation Details Panel
//!
//! Shows detailed information about a Restate invocation including:
//! - Invocation ID, workflow name, status, timing
//! - Journal entries list
//! - State changes

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InvocationInfo {
    pub id: String,
    pub target: String,
    pub status: String,
    pub created_at: i64,
    pub completed_at: Option<i64>,
    pub journal_size: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JournalEntryInfo {
    pub index: u32,
    pub entry_type: String,
    pub name: Option<String>,
    pub completed: bool,
    pub invoked_target: Option<String>,
    pub invoked_id: Option<String>,
    pub promise_name: Option<String>,
    pub sleep_wakeup_at: Option<i64>,
    pub entry_json: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RestateInvocationDetailsProps {
    pub invocation: InvocationInfo,
    pub journal: Vec<JournalEntryInfo>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn RestateInvocationDetails(props: RestateInvocationDetailsProps) -> Element {
    let inv = &props.invocation;
    let journal = &props.journal;

    rsx! {
        div {
            class: "fixed inset-0 bg-black/50 flex items-center justify-center z-50",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "bg-white dark:bg-gray-900 rounded-lg shadow-xl max-w-4xl w-full max-h-[80vh] overflow-hidden",
                onclick: |_| {},

                // Header
                div {
                    class: "flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700",

                    h2 {
                        class: "text-lg font-semibold",
                        "Restate Invocation Details"
                    }

                    button {
                        class: "p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded",
                        onclick: move |_| props.on_close.call(()),
                        "✕"
                    }
                }

                // Content
                div {
                    class: "p-4 overflow-y-auto max-h-[calc(80vh-80px)]",

                    // Invocation Info
                    div {
                        class: "grid grid-cols-2 gap-4 mb-6",

                        div {
                            class: "space-y-2",
                            div { class: "text-sm text-gray-500", "Invocation ID" }
                            div { class: "font-mono text-sm break-all", {&inv.id} }
                        }

                        div {
                            class: "space-y-2",
                            div { class: "text-sm text-gray-500", "Workflow" }
                            div { class: "font-medium", {&inv.target} }
                        }

                        div {
                            class: "space-y-2",
                            div { class: "text-sm text-gray-500", "Status" }
                            div {
                                class: {
                                    let status = &inv.status;
                                    let base = "px-2 py-1 rounded text-sm ";
                                    match status.as_str() {
                                        "completed" => format!("{} bg-green-100 text-green-800", base),
                                        "running" => format!("{} bg-blue-100 text-blue-800", base),
                                        "failed" => format!("{} bg-red-100 text-red-800", base),
                                        _ => format!("{} bg-gray-100 text-gray-800", base),
                                    }
                                },
                                {&inv.status}
                            }
                        }

                        div {
                            class: "space-y-2",
                            div { class: "text-sm text-gray-500", "Started" }
                            div { class: "text-sm", {format_time(inv.created_at)} }
                        }

                        if let Some(finished) = inv.completed_at {
                            div {
                                class: "space-y-2",
                                div { class: "text-sm text-gray-500", "Finished" }
                                div { class: "text-sm", {format_time(finished)} }
                            }
                        }

                        div {
                            class: "space-y-2",
                            div { class: "text-sm text-gray-500", "Journal Size" }
                            div { class: "text-sm", {inv.journal_size} }
                        }
                    }

                    // Journal Entries
                    div {
                        class: "mt-6",
                        h3 {
                            class: "text-md font-semibold mb-3",
                            "Journal Entries"
                        }

                        if journal.is_empty() {
                            div {
                                class: "text-gray-500 text-sm",
                                "No journal entries"
                            }
                        } else {
                            div {
                                class: "space-y-2",
                                for entry in journal {
                                    div {
                                        class: "flex items-center gap-3 p-2 bg-gray-50 dark:bg-gray-800 rounded",

                                        span {
                                            class: "font-mono text-sm text-gray-500 w-8",
                                            {entry.index}
                                        }

                                        span {
                                            class: {
                                                let base = "px-2 py-0.5 rounded text-xs ";
                                                if entry.completed {
                                                    format!("{} bg-green-100 text-green-800", base)
                                                } else {
                                                    format!("{} bg-yellow-100 text-yellow-800", base)
                                                }
                                            },
                                            {&entry.entry_type}
                                        }

                                        span {
                                            class: "flex-1 text-sm",
                                            {entry.name.clone().unwrap_or_default()}
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

fn format_time(ts: i64) -> String {
    if let Some(dt) = chrono::DateTime::from_timestamp_millis(ts) {
        dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    } else {
        ts.to_string()
    }
}
