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
        "entry" => "bg-blue-100 text-blue-700 border-blue-200",
        "durable" => "bg-green-100 text-green-700 border-green-200",
        "state" => "bg-cyan-100 text-cyan-700 border-cyan-200",
        "flow" => "bg-pink-100 text-pink-700 border-pink-200",
        "timing" => "bg-purple-100 text-purple-700 border-purple-200",
        "signal" => "bg-amber-100 text-amber-700 border-amber-200",
        _ => "bg-slate-100 text-slate-700 border-slate-200",
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
        aside { class: "flex h-full w-[310px] shrink-0 flex-col border-r border-slate-200 bg-slate-50/95 backdrop-blur",
            div { class: "flex items-center gap-2 border-b border-slate-200 px-4 py-3",
                div { class: "flex h-6 w-6 items-center justify-center rounded-md bg-blue-600/10",
                    BoxIcon { class: "h-3.5 w-3.5 text-blue-600" }
                }
                span { class: "text-[13px] font-semibold text-slate-900", "Application Composer" }
            }

            div { class: "px-3 py-2.5",
                div { class: "relative",
                    SearchIcon { class: "pointer-events-none absolute left-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" }
                    input {
                        r#type: "text",
                        placeholder: "Search nodes...",
                        value: "{search.read()}",
                        class: "h-9 w-full rounded-xl border border-slate-300 bg-white pl-8 pr-3 text-[12px] text-slate-800 placeholder:text-slate-400 outline-none transition-colors focus:border-blue-500/60 focus:ring-2 focus:ring-blue-500/20",
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
                                        span { class: "text-[11px] font-semibold uppercase tracking-wider text-slate-500", "{category_label(category)}" }
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
                        SearchIcon { class: "mb-3 h-8 w-8 text-slate-300" }
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
                class: "group flex w-full cursor-grab items-center gap-2.5 rounded-xl border border-transparent bg-white px-2.5 py-2.5 text-left transition-all duration-100 hover:border-slate-200 hover:bg-slate-100 active:scale-[0.98] active:cursor-grabbing",
                onmousedown: move |_| on_pickup_node.call(template.node_type),
                onclick: move |_| on_add_node.call(template.node_type),
                onmouseenter: move |_| show_tooltip.set(true),
                onmouseleave: move |_| show_tooltip.set(false),
                div { class: "flex h-7 w-7 shrink-0 items-center justify-center rounded-md border transition-colors {category_icon_colors(template.category)}",
                    {icon_by_name(template.icon, "h-3.5 w-3.5".to_string())}
                }
                div { class: "flex min-w-0 flex-1 flex-col",
                    span { class: "truncate text-[12px] font-semibold leading-tight text-slate-900", "{template.label}" }
                    span { class: "truncate text-[10px] leading-tight text-slate-500", "{template.friendly}" }
                }
                div { class: "flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100",
                    if let Some(doc_link) = template.doc_link {
                        a {
                            href: "/docs/10_RESTATE_SDK.md{doc_link}",
                            target: "_blank",
                            class: "text-slate-400 hover:text-blue-600",
                            onclick: move |evt| evt.stop_propagation(),
                            HelpCircleIcon { class: "h-3.5 w-3.5" }
                        }
                    }
                }
            }

            if *show_tooltip.read() {
                div {
                    class: "absolute left-full top-0 z-50 ml-2 w-64 rounded-lg border border-slate-200 bg-white p-3 shadow-xl",

                    div { class: "flex items-start gap-2",
                        div { class: "mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-md {category_icon_colors(template.category)}",
                            {icon_by_name(template.icon, "h-3 w-3".to_string())}
                        }
                        div { class: "flex-1",
                            h4 { class: "text-[12px] font-semibold text-slate-900", "{template.label}" }
                            p { class: "mt-1 text-[11px] leading-relaxed text-slate-600 whitespace-pre-line", "{template.tooltip}" }
                        }
                    }

                    if let Some(doc_link) = template.doc_link {
                        div { class: "mt-2 border-t border-slate-200 pt-2",
                            a {
                                href: "/docs/10_RESTATE_SDK.md{doc_link}",
                                target: "_blank",
                                class: "flex items-center gap-1 text-[11px] text-blue-600 hover:underline",
                                "View documentation "
                                span { class: "text-blue-500", "->" }
                            }
                        }
                    }
                }
            }
        }
    }
}
