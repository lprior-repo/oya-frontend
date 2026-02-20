use crate::ui::icons::{icon_by_name, BoxIcon, SearchIcon};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
struct NodeTemplate {
    node_type: &'static str,
    label: &'static str,
    description: &'static str,
    icon: &'static str,
    category: &'static str,
}

const NODE_TEMPLATES: [NodeTemplate; 24] = [
    NodeTemplate {
        node_type: "http-handler",
        label: "HTTP Handler",
        description: "Handle HTTP/gRPC invocation",
        icon: "globe",
        category: "entry",
    },
    NodeTemplate {
        node_type: "kafka-handler",
        label: "Kafka Consumer",
        description: "Consume events from Kafka topic",
        icon: "kafka",
        category: "entry",
    },
    NodeTemplate {
        node_type: "cron-trigger",
        label: "Cron Trigger",
        description: "Schedule periodic execution",
        icon: "clock",
        category: "entry",
    },
    NodeTemplate {
        node_type: "workflow-submit",
        label: "Workflow Submit",
        description: "Submit workflow with key",
        icon: "play-circle",
        category: "entry",
    },
    NodeTemplate {
        node_type: "run",
        label: "Durable Step",
        description: "ctx.run() - persisted side effect",
        icon: "shield",
        category: "durable",
    },
    NodeTemplate {
        node_type: "service-call",
        label: "Service Call",
        description: "Request-response to service",
        icon: "arrow-right",
        category: "durable",
    },
    NodeTemplate {
        node_type: "object-call",
        label: "Object Call",
        description: "Call virtual object handler",
        icon: "box",
        category: "durable",
    },
    NodeTemplate {
        node_type: "workflow-call",
        label: "Workflow Call",
        description: "Submit or attach to workflow",
        icon: "workflow",
        category: "durable",
    },
    NodeTemplate {
        node_type: "send-message",
        label: "Send Message",
        description: "Fire-and-forget one-way call",
        icon: "send",
        category: "durable",
    },
    NodeTemplate {
        node_type: "delayed-send",
        label: "Delayed Message",
        description: "Schedule future handler call",
        icon: "clock-send",
        category: "durable",
    },
    NodeTemplate {
        node_type: "get-state",
        label: "Get State",
        description: "ctx.get() - read persisted state",
        icon: "download",
        category: "state",
    },
    NodeTemplate {
        node_type: "set-state",
        label: "Set State",
        description: "ctx.set() - write persisted state",
        icon: "upload",
        category: "state",
    },
    NodeTemplate {
        node_type: "clear-state",
        label: "Clear State",
        description: "ctx.clear() / clearAll()",
        icon: "eraser",
        category: "state",
    },
    NodeTemplate {
        node_type: "condition",
        label: "If / Else",
        description: "Conditional branching",
        icon: "git-branch",
        category: "flow",
    },
    NodeTemplate {
        node_type: "switch",
        label: "Switch",
        description: "Multi-path routing",
        icon: "git-fork",
        category: "flow",
    },
    NodeTemplate {
        node_type: "loop",
        label: "Loop / Iterate",
        description: "Iterate over collection",
        icon: "repeat",
        category: "flow",
    },
    NodeTemplate {
        node_type: "parallel",
        label: "Parallel",
        description: "Promise.all() concurrent steps",
        icon: "layers",
        category: "flow",
    },
    NodeTemplate {
        node_type: "compensate",
        label: "Compensate",
        description: "Saga compensation / rollback",
        icon: "undo",
        category: "flow",
    },
    NodeTemplate {
        node_type: "sleep",
        label: "Sleep / Timer",
        description: "ctx.sleep() - durable pause",
        icon: "timer",
        category: "timing",
    },
    NodeTemplate {
        node_type: "timeout",
        label: "Timeout",
        description: "orTimeout() - deadline guard",
        icon: "alarm",
        category: "timing",
    },
    NodeTemplate {
        node_type: "durable-promise",
        label: "Durable Promise",
        description: "ctx.promise() - await external",
        icon: "sparkles",
        category: "signal",
    },
    NodeTemplate {
        node_type: "awakeable",
        label: "Awakeable",
        description: "Pause for external completion",
        icon: "bell",
        category: "signal",
    },
    NodeTemplate {
        node_type: "resolve-promise",
        label: "Resolve Promise",
        description: "Resolve a durable promise",
        icon: "check-circle",
        category: "signal",
    },
    NodeTemplate {
        node_type: "signal-handler",
        label: "Signal Handler",
        description: "Shared handler for signals",
        icon: "radio",
        category: "signal",
    },
];

fn category_dot(category: &str) -> &'static str {
    match category {
        "trigger" => "bg-emerald-400",
        "action" => "bg-indigo-400",
        "logic" => "bg-amber-400",
        "output" => "bg-pink-400",
        "restate" => "bg-blue-400",
        _ => "bg-slate-400",
    }
}

fn category_icon_colors(category: &str) -> &'static str {
    match category {
        "trigger" => "bg-emerald-500/10 text-emerald-300 border-emerald-500/20",
        "action" => "bg-indigo-500/10 text-indigo-300 border-indigo-500/20",
        "logic" => "bg-amber-500/10 text-amber-300 border-amber-500/20",
        "output" => "bg-pink-500/10 text-pink-300 border-pink-500/20",
        "restate" => "bg-blue-500/10 text-blue-300 border-blue-500/20",
        _ => "bg-slate-500/10 text-slate-300 border-slate-500/20",
    }
}

fn category_label(category: &str) -> &'static str {
    match category {
        "entry" => "Entry Points",
        "durable" => "Durable Steps",
        "state" => "State",
        "flow" => "Control Flow",
        "timing" => "Timing & Events",
        "signal" => "Signals & Promises",
        _ => "Other",
    }
}

#[component]
pub fn NodeSidebar(
    search: ReadSignal<String>,
    on_search_change: EventHandler<String>,
    on_pickup_node: EventHandler<&'static str>,
    on_add_node: EventHandler<&'static str>,
) -> Element {
    let query = search.read().to_lowercase();

    rsx! {
        aside { class: "flex h-full w-[260px] shrink-0 flex-col border-r border-slate-800 bg-slate-900/95 backdrop-blur",
            div { class: "flex items-center gap-2 border-b border-slate-800 px-4 py-3",
                div { class: "flex h-6 w-6 items-center justify-center rounded-md bg-indigo-500/10",
                    BoxIcon { class: "h-3.5 w-3.5 text-indigo-300" }
                }
                span { class: "text-[13px] font-semibold text-slate-100", "Nodes" }
            }

            div { class: "px-3 py-2.5",
                div { class: "relative",
                    SearchIcon { class: "pointer-events-none absolute left-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-500" }
                    input {
                        r#type: "text",
                        placeholder: "Search nodes...",
                        value: "{search.read()}",
                        class: "h-8 w-full rounded-md border border-slate-700 bg-slate-950 pl-8 pr-3 text-[12px] text-slate-100 placeholder:text-slate-500 outline-none transition-colors focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/30",
                        oninput: move |evt| on_search_change.call(evt.value())
                    }
                }
            }

            div { class: "flex-1 overflow-y-auto px-3 pb-4",
                for category in ["entry", "durable", "state", "flow", "timing", "signal"] {
                    {
                        let templates = NODE_TEMPLATES
                            .iter()
                            .filter(|template| {
                                template.category == category
                                    && (template.label.to_lowercase().contains(&query)
                                        || template.description.to_lowercase().contains(&query))
                            })
                            .copied()
                            .collect::<Vec<_>>();

                        if templates.is_empty() {
                            rsx! {}
                        } else {
                            rsx! {
                                div { class: "mb-4", key: "{category}",
                                    div { class: "flex items-center gap-2 px-1 py-2",
                                        div { class: "h-1.5 w-1.5 rounded-full {category_dot(category)}" }
                                        span { class: "text-[11px] font-medium uppercase tracking-wider text-slate-500", "{category_label(category)}" }
                                    }

                                    div { class: "flex flex-col gap-1",
                                        for template in templates {
                                            button {
                                                key: "{template.node_type}",
                                                class: "group flex cursor-grab items-center gap-2.5 rounded-md px-2.5 py-2 text-left transition-all duration-100 hover:bg-slate-800 active:scale-[0.98] active:cursor-grabbing",
                                                onmousedown: move |_| on_pickup_node.call(template.node_type),
                                                onclick: move |_| on_add_node.call(template.node_type),
                                                div { class: "flex h-7 w-7 shrink-0 items-center justify-center rounded-md border transition-colors {category_icon_colors(template.category)}",
                                                    {icon_by_name(template.icon, "h-3.5 w-3.5".to_string())}
                                                }
                                                div { class: "flex min-w-0 flex-col",
                                                    span { class: "truncate text-[12px] font-medium leading-tight text-slate-100", "{template.label}" }
                                                    span { class: "truncate text-[10px] leading-tight text-slate-500", "{template.description}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if NODE_TEMPLATES.iter().all(|template| {
                    !(template.label.to_lowercase().contains(&query)
                        || template.description.to_lowercase().contains(&query))
                }) {
                    div { class: "flex flex-col items-center justify-center py-12 text-center",
                        SearchIcon { class: "mb-3 h-8 w-8 text-slate-700" }
                        p { class: "text-[12px] text-slate-500", "No nodes found" }
                    }
                }
            }
        }
    }
}
