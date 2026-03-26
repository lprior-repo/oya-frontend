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
use oya_frontend::restate_client::types::{Invocation, InvocationStatus, JournalEntry};

const fn status_to_ui_string(status: InvocationStatus) -> &'static str {
    match status {
        InvocationStatus::Pending => "pending",
        InvocationStatus::Scheduled => "scheduled",
        InvocationStatus::Ready => "ready",
        InvocationStatus::Running => "running",
        InvocationStatus::Paused => "paused",
        InvocationStatus::BackingOff => "backing-off",
        InvocationStatus::Suspended => "suspended",
        InvocationStatus::Completed => "completed",
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RestateInvocationDetailsProps {
    pub invocation: Invocation,
    pub journal: Vec<JournalEntry>,
    #[props(default)]
    pub loading: bool,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn RestateInvocationDetails(props: RestateInvocationDetailsProps) -> Element {
    let inv = &props.invocation;
    let journal = &props.journal;
    let status_str = status_to_ui_string(inv.status);

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
                            div { class: "font-mono text-sm break-all", {inv.id.clone()} }
                        }

                        div {
                            class: "space-y-2",
                            div { class: "text-sm text-gray-500", "Workflow" }
                            div { class: "font-medium", {inv.target.clone()} }
                        }

                        div {
                            class: "space-y-2",
                            div { class: "text-sm text-gray-500", "Status" }
                            div {
                                class: {
                                    let base = "px-2 py-1 rounded text-sm ";
                                    match inv.status {
                                        InvocationStatus::Completed => format!("{base} bg-green-100 text-green-800"),
                                        InvocationStatus::Running => format!("{base} bg-blue-100 text-blue-800"),
                                        InvocationStatus::Paused | InvocationStatus::BackingOff => format!("{base} bg-red-100 text-red-800"),
                                        _ => format!("{base} bg-gray-100 text-gray-800"),
                                    }
                                },
                                {status_str}
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
                            div { class: "text-sm", {inv.journal_size.to_string()} }
                        }
                    }

                    // Journal Entries
                    div {
                        class: "mt-6",
                        h3 {
                            class: "text-md font-semibold mb-3",
                            "Journal Entries"
                        }

                        if props.loading {
                            div {
                                class: "text-gray-500 text-sm",
                                "Loading journal\u{2026}"
                            }
                        } else if journal.is_empty() {
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
                                            {entry.index.to_string()}
                                        }

                                        span {
                                            class: {
                                                let base = "px-2 py-0.5 rounded text-xs ";
                                                if entry.completed {
                                                    format!("{base} bg-green-100 text-green-800")
                                                } else {
                                                    format!("{base} bg-yellow-100 text-yellow-800")
                                                }
                                            },
                                            {entry.raw_entry_type.clone()}
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
    chrono::DateTime::from_timestamp_millis(ts).map_or_else(
        || ts.to_string(),
        |dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
    )
}
