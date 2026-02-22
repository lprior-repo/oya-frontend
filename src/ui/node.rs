use crate::ui::icons::icon_by_name;
use crate::ui::InlineConfigPanel;
use dioxus::prelude::*;
use oya_frontend::graph::{ExecutionState, Node, NodeCategory};
use serde_json::Value;

#[component]
pub fn FlowNodeComponent(
    node: Node,
    selected: bool,
    inline_open: bool,
    on_mouse_down: EventHandler<MouseEvent>,
    on_click: EventHandler<MouseEvent>,
    on_double_click: EventHandler<MouseEvent>,
    on_handle_mouse_down: EventHandler<(MouseEvent, String)>,
    on_handle_mouse_enter: EventHandler<String>,
    on_handle_mouse_leave: EventHandler<()>,
    on_inline_change: EventHandler<Value>,
    on_inline_close: EventHandler<()>,
) -> Element {
    let category = node.category;
    let icon = node.icon.clone();

    let category_border = match category {
        NodeCategory::Entry => "border-emerald-500/40",
        NodeCategory::Durable => "border-indigo-500/40",
        NodeCategory::State => "border-orange-500/40",
        NodeCategory::Flow => "border-amber-500/40",
        NodeCategory::Timing => "border-pink-500/40",
        NodeCategory::Signal => "border-blue-500/40",
    };

    let category_icon_bg = match category {
        NodeCategory::Entry => "bg-emerald-500/15 text-emerald-400",
        NodeCategory::Durable => "bg-indigo-500/15 text-indigo-500",
        NodeCategory::State => "bg-orange-500/15 text-orange-400",
        NodeCategory::Flow => "bg-amber-500/15 text-amber-400",
        NodeCategory::Timing => "bg-pink-500/15 text-pink-400",
        NodeCategory::Signal => "bg-blue-500/15 text-blue-400",
    };

    let category_accent_bar = match category {
        NodeCategory::Entry => "bg-emerald-500/40",
        NodeCategory::Durable => "bg-indigo-500/40",
        NodeCategory::State => "bg-orange-500/40",
        NodeCategory::Flow => "bg-amber-500/40",
        NodeCategory::Timing => "bg-pink-500/40",
        NodeCategory::Signal => "bg-blue-500/40",
    };

    let is_running = matches!(node.execution_state, ExecutionState::Running);

    let selected_classes = if selected {
        "ring-2 ring-blue-500/60 border-blue-500/40 shadow-xl shadow-blue-500/20"
    } else {
        "hover:border-slate-300"
    };
    let running_classes = if is_running {
        "shadow-[0_0_0_2px_rgba(59,130,246,0.18)]"
    } else {
        ""
    };

    let z_index = if selected || inline_open { 10 } else { 1 };

    rsx! {
        div {
            "data-node-id": "{node.id}",
            class: "absolute select-none",
            style: "left: {node.x}px; top: {node.y}px; z-index: {z_index};",

            div {
                class: "group rounded-xl border bg-white transition-shadow duration-150 cursor-grab active:cursor-grabbing {category_border} {selected_classes} {running_classes}",
                style: "width: 220px;",
                onmousedown: move |e| {
                    on_mouse_down.call(e);
                },
                onclick: move |e| {
                    on_click.call(e);
                },
                ondoubleclick: move |e| {
                    e.stop_propagation();
                    on_double_click.call(e);
                },

                div {
                    class: "absolute -top-[5px] left-1/2 -translate-x-1/2 h-[10px] w-[10px] rounded-full border-2 border-slate-300 bg-white hover:bg-blue-500 hover:border-blue-500 hover:scale-125 transition-all duration-150 cursor-crosshair z-10",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        on_handle_mouse_down.call((e, "target".to_string()));
                    },
                    onmouseenter: move |_| on_handle_mouse_enter.call("target".to_string()),
                    onmouseleave: move |_| on_handle_mouse_leave.call(())
                }

                div { class: "flex items-center gap-3 px-3.5 py-3",
                    div { class: "flex h-8 w-8 shrink-0 items-center justify-center rounded-md {category_icon_bg}",
                        {icon_by_name(&icon, "h-4 w-4".to_string())}
                    }

                    div { class: "flex flex-col gap-0.5 min-w-0 flex-1",
                        span { class: "text-[13px] font-semibold leading-tight text-slate-900 truncate", "{node.name}" }
                        span { class: "text-[11px] leading-tight text-slate-500 truncate", "{node.description}" }
                    }

                    div { class: "ml-auto shrink-0",
                        {
                            match node.execution_state {
                                ExecutionState::Idle => rsx! { div {} },
                                ExecutionState::Waiting => rsx! {
                                    span {
                                        class: "inline-flex items-center gap-1 rounded-full border px-1.5 py-px text-[9px] font-medium leading-none bg-amber-500/15 text-amber-400 border-amber-500/30",
                                        {icon_by_name("clock", "h-2.5 w-2.5".to_string())}
                                        "Waiting"
                                    }
                                },
                                ExecutionState::Running => rsx! {
                                    span {
                                        class: "inline-flex items-center gap-1 rounded-full border px-1.5 py-px text-[9px] font-medium leading-none bg-blue-500/15 text-blue-400 border-blue-500/30",
                                        {icon_by_name("loader", "h-2.5 w-2.5 animate-spin".to_string())}
                                        "Running"
                                    }
                                },
                                ExecutionState::Succeeded => rsx! {
                                    span {
                                        class: "inline-flex items-center gap-1 rounded-full border px-1.5 py-px text-[9px] font-medium leading-none bg-emerald-500/15 text-emerald-400 border-emerald-500/30",
                                        {icon_by_name("check-circle", "h-2.5 w-2.5".to_string())}
                                        "Done"
                                    }
                                },
                                ExecutionState::Failed => rsx! {
                                    span {
                                        class: "inline-flex items-center gap-1 rounded-full border px-1.5 py-px text-[9px] font-medium leading-none bg-red-500/15 text-red-400 border-red-500/30",
                                        title: "Execution failed",
                                        {icon_by_name("x", "h-2.5 w-2.5".to_string())}
                                        "Failed"
                                    }
                                },
                                ExecutionState::Skipped => rsx! {
                                    span {
                                        class: "inline-flex items-center gap-1 rounded-full border px-1.5 py-px text-[9px] font-medium leading-none bg-slate-500/15 text-slate-400 border-slate-500/30 opacity-60",
                                        {icon_by_name("x", "h-2.5 w-2.5".to_string())}
                                        "Skipped"
                                    }
                                },
                            }
                        }
                    }
                }

                if node.config.get("journalIndex").is_some() || node.config.get("retryCount").and_then(serde_json::Value::as_u64) > Some(0) {
                    div { class: "flex items-center gap-2 px-3 pb-2 text-[9px] font-mono text-slate-500",
                        if let Some(idx) = node.config.get("journalIndex").and_then(serde_json::Value::as_u64) {
                            span { class: "rounded bg-slate-100 px-1 py-px", "journal #{idx}" }
                        }
                        if let Some(retries) = node.config.get("retryCount").and_then(serde_json::Value::as_u64) {
                            if retries > 0 {
                                span { class: "rounded bg-red-500/10 text-red-400/70 px-1 py-px", "{retries} retries" }
                            }
                        }
                        if let Some(key) = node.config.get("idempotencyKey").and_then(|i| i.as_str()) {
                            span { class: "rounded bg-slate-100 px-1 py-px truncate max-w-[80px]", title: "{key}", "key: {key}" }
                        }
                    }
                }

                div { class: "h-[2px] rounded-b-lg {category_accent_bar}" }

                div {
                    class: "absolute -bottom-[5px] left-1/2 -translate-x-1/2 h-[10px] w-[10px] rounded-full border-2 border-slate-300 bg-white hover:bg-blue-500 hover:border-blue-500 hover:scale-125 transition-all duration-150 cursor-crosshair z-10",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        on_handle_mouse_down.call((e, "source".to_string()));
                    },
                    onmouseenter: move |_| on_handle_mouse_enter.call("source".to_string()),
                    onmouseleave: move |_| on_handle_mouse_leave.call(())
                }
            }

            if inline_open {
                div {
                    class: "mt-1",
                    onclick: move |e| e.stop_propagation(),
                    InlineConfigPanel {
                        node: node.clone(),
                        on_change: on_inline_change,
                        on_close: on_inline_close,
                    }
                }
            }
        }
    }
}
