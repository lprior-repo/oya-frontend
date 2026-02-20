use crate::ui::icons::{icon_by_name, BoxIcon, HelpCircleIcon, SearchIcon};
use dioxus::prelude::*;

use super::model::{NodeTemplate, NODE_TEMPLATES};

pub(super) fn category_dot(category: &str) -> &'static str {
    match category {
        "entry" => "bg-blue-400",
        "durable" => "bg-green-400",
        "state" => "bg-cyan-400",
        "flow" => "bg-pink-400",
        "timing" => "bg-purple-400",
        "signal" => "bg-amber-400",
        _ => "bg-slate-400",
    }
}

fn category_icon_colors(category: &str) -> &'static str {
    match category {
        "entry" => "bg-blue-500/10 text-blue-300 border-blue-500/20",
        "durable" => "bg-green-500/10 text-green-300 border-green-500/20",
        "state" => "bg-cyan-500/10 text-cyan-300 border-cyan-500/20",
        "flow" => "bg-pink-500/10 text-pink-300 border-pink-500/20",
        "timing" => "bg-purple-500/10 text-purple-300 border-purple-500/20",
        "signal" => "bg-amber-500/10 text-amber-300 border-amber-500/20",
        _ => "bg-slate-500/10 text-slate-300 border-slate-500/20",
    }
}

pub(super) fn category_label(category: &str) -> &'static str {
    match category {
        "entry" => "Entry Points",
        "durable" => "Actions",
        "state" => "Memory",
        "flow" => "Logic",
        "timing" => "Timing",
        "signal" => "Signals",
        _ => "Other",
    }
}

pub(super) fn template_matches_query(template: &NodeTemplate, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let query = query.to_lowercase();
    template.label.to_lowercase().contains(&query)
        || template.description.to_lowercase().contains(&query)
        || template.friendly.to_lowercase().contains(&query)
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
        aside { class: "flex h-full w-[280px] shrink-0 flex-col border-r border-slate-800 bg-slate-900/95 backdrop-blur",
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
                        let indices: Vec<usize> = NODE_TEMPLATES
                            .iter()
                            .enumerate()
                            .filter(|(_, template)| {
                                template.category == category
                                    && template_matches_query(template, &query)
                            })
                            .map(|(i, _)| i)
                            .collect();

                        if indices.is_empty() {
                            rsx! {}
                        } else {
                            rsx! {
                                div { class: "mb-4", key: "{category}",
                                    div { class: "flex items-center gap-2 px-1 py-2",
                                        div { class: "h-1.5 w-1.5 rounded-full {category_dot(category)}" }
                                        span { class: "text-[11px] font-medium uppercase tracking-wider text-slate-500", "{category_label(category)}" }
                                    }

                                    div { class: "flex flex-col gap-1",
                                        for idx in indices {
                                            NodeButton { template: &NODE_TEMPLATES[idx], on_pickup_node: on_pickup_node, on_add_node: on_add_node }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if NODE_TEMPLATES.iter().all(|template| {
                    !template_matches_query(template, &query)
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

#[component]
fn NodeButton(
    template: &'static NodeTemplate,
    on_pickup_node: EventHandler<&'static str>,
    on_add_node: EventHandler<&'static str>,
) -> Element {
    let mut show_tooltip = use_signal(|| false);

    rsx! {
        div { class: "relative",
            button {
                key: "{template.node_type}",
                class: "group flex w-full cursor-grab items-center gap-2.5 rounded-md px-2.5 py-2 text-left transition-all duration-100 hover:bg-slate-800 active:scale-[0.98] active:cursor-grabbing",
                onmousedown: move |_| on_pickup_node.call(template.node_type),
                onclick: move |_| on_add_node.call(template.node_type),
                onmouseenter: move |_| show_tooltip.set(true),
                onmouseleave: move |_| show_tooltip.set(false),
                div { class: "flex h-7 w-7 shrink-0 items-center justify-center rounded-md border transition-colors {category_icon_colors(template.category)}",
                    {icon_by_name(template.icon, "h-3.5 w-3.5".to_string())}
                }
                div { class: "flex min-w-0 flex-1 flex-col",
                    span { class: "truncate text-[12px] font-medium leading-tight text-slate-100", "{template.label}" }
                    span { class: "truncate text-[10px] leading-tight text-slate-500", "{template.friendly}" }
                }
                div { class: "flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100",
                    if let Some(doc_link) = template.doc_link {
                        a {
                            href: "/docs/10_RESTATE_SDK.md{doc_link}",
                            target: "_blank",
                            class: "text-slate-500 hover:text-indigo-400",
                            onclick: move |evt| evt.stop_propagation(),
                            HelpCircleIcon { class: "h-3.5 w-3.5" }
                        }
                    }
                }
            }

            if *show_tooltip.read() {
                div {
                    class: "absolute left-full top-0 z-50 ml-2 w-64 rounded-lg border border-slate-700 bg-slate-800 p-3 shadow-xl",

                    div { class: "flex items-start gap-2",
                        div { class: "mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-md {category_icon_colors(template.category)}",
                            {icon_by_name(template.icon, "h-3 w-3".to_string())}
                        }
                        div { class: "flex-1",
                            h4 { class: "text-[12px] font-semibold text-slate-100", "{template.label}" }
                            p { class: "mt-1 text-[11px] leading-relaxed text-slate-300 whitespace-pre-line", "{template.tooltip}" }
                        }
                    }

                    if let Some(doc_link) = template.doc_link {
                        div { class: "mt-2 border-t border-slate-700 pt-2",
                            a {
                                href: "/docs/10_RESTATE_SDK.md{doc_link}",
                                target: "_blank",
                                class: "flex items-center gap-1 text-[11px] text-indigo-400 hover:underline",
                                "View documentation "
                                span { class: "text-indigo-300", "->" }
                            }
                        }
                    }
                }
            }
        }
    }
}
