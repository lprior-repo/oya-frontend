use dioxus::prelude::*;
use oya_frontend::graph::{Node, NodeCategory, NodeId, Workflow};
use std::collections::HashMap;

use crate::ui::NodeConfigEditor;

#[component]
pub fn SelectedNodePanel(
    selected_node_id: Signal<Option<NodeId>>,
    selected_node_ids: Signal<Vec<NodeId>>,
    nodes_by_id: ReadSignal<HashMap<NodeId, Node>>,
    workflow: Signal<Workflow>,
    undo_stack: Signal<Vec<Workflow>>,
    redo_stack: Signal<Vec<Workflow>>,
) -> Element {
    if let Some(node_id) = *selected_node_id.read() {
        if let Some(selected_node) = nodes_by_id.read().get(&node_id).cloned() {
            let badge_classes = match selected_node.category {
                NodeCategory::Entry => "bg-emerald-500/15 text-emerald-300 border-emerald-500/25",
                NodeCategory::Durable => "bg-indigo-500/15 text-indigo-300 border-indigo-500/25",
                NodeCategory::State => "bg-orange-500/15 text-orange-300 border-orange-500/25",
                NodeCategory::Flow => "bg-amber-500/15 text-amber-300 border-amber-500/25",
                NodeCategory::Timing => "bg-pink-500/15 text-pink-300 border-pink-500/25",
                NodeCategory::Signal => "bg-blue-500/15 text-blue-300 border-blue-500/25",
            };

            return rsx! {
                aside { class: "animate-slide-in-right z-30 flex w-[300px] shrink-0 flex-col border-l border-slate-800 bg-slate-900/95",
                    div { class: "flex items-center justify-between border-b border-slate-800 px-4 py-3",
                        div { class: "flex items-center gap-2.5",
                            div { class: "flex h-7 w-7 items-center justify-center rounded-md border {badge_classes}",
                                {crate::ui::icons::icon_by_name(&selected_node.icon, "h-3.5 w-3.5".to_string())}
                            }
                            div {
                                h3 { class: "text-[13px] font-semibold text-slate-100", "{selected_node.name}" }
                                p { class: "text-[10px] text-slate-500", "{selected_node.description}" }
                            }
                        }
                        button {
                            class: "flex h-6 w-6 items-center justify-center rounded-md text-slate-500 transition-colors hover:bg-slate-800 hover:text-slate-100",
                            onclick: move |_| {
                                selected_node_id.set(None);
                                selected_node_ids.set(Vec::new());
                            },
                            crate::ui::icons::XIcon { class: "h-3.5 w-3.5" }
                        }
                    }

                    div { class: "flex-1 overflow-y-auto p-4",
                        div { class: "mb-4 flex items-center gap-2",
                            span { class: "inline-flex items-center rounded-md border px-2 py-0.5 text-[10px] font-medium capitalize {badge_classes}", "{selected_node.category}" }
                            span { class: "text-[10px] font-mono text-slate-500", "ID: {selected_node.id}" }
                        }
                        div { class: "mb-4 flex flex-col gap-1.5",
                            label { class: "text-[11px] font-medium uppercase tracking-wide text-slate-500", "Node Name" }
                            input {
                                class: "h-8 rounded-md border border-slate-700 bg-slate-950 px-3 text-[12px] text-slate-100 outline-none transition-colors focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/30",
                                value: "{selected_node.name}",
                                oninput: move |evt| {
                                    let mut wf = workflow.write();
                                    if let Some(node) = wf.nodes.iter_mut().find(|node| node.id == node_id) {
                                        node.name = evt.value();
                                    }
                                }
                            }
                        }

                        div { class: "mb-4 flex flex-col gap-1.5",
                            label { class: "text-[11px] font-medium uppercase tracking-wide text-slate-500", "Notes" }
                            textarea {
                                rows: "3",
                                placeholder: "Add notes about this node...",
                                class: "rounded-md border border-slate-700 bg-slate-950 px-3 py-2 text-[12px] text-slate-100 placeholder:text-slate-500/70 outline-none transition-colors focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/30 resize-none",
                                value: "{selected_node.description}",
                                oninput: move |evt| {
                                    let mut wf = workflow.write();
                                    if let Some(node) = wf.nodes.iter_mut().find(|node| node.id == node_id) {
                                        node.description = evt.value();
                                    }
                                }
                            }
                        }

                        div { class: "h-px bg-slate-800" }
                        div { class: "pt-4",
                            NodeConfigEditor {
                                node: selected_node.clone(),
                                on_change: move |new_config| {
                                    let mut wf = workflow.write();
                                    if let Some(node) = wf.nodes.iter_mut().find(|node| node.id == node_id) {
                                        node.config = new_config;
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex items-center gap-2 border-t border-slate-800 px-4 py-3",
                        button {
                            class: "flex h-8 flex-1 items-center justify-center gap-1.5 rounded-md border border-slate-700 text-[12px] text-slate-300 transition-colors hover:bg-slate-800 hover:text-slate-100",
                            onclick: move |_| {
                                let snapshot = workflow.read().clone();
                                undo_stack.write().push(snapshot);
                                if undo_stack.read().len() > 60 {
                                    let _ = undo_stack.write().remove(0);
                                }
                                redo_stack.write().clear();

                                let maybe_clone = workflow
                                    .read()
                                    .nodes
                                    .iter()
                                    .find(|node| node.id == node_id)
                                    .cloned();
                                if let Some(mut clone) = maybe_clone {
                                    clone.id = NodeId::new();
                                    clone.x += 40.0;
                                    clone.y += 40.0;
                                    let cloned_id = clone.id;
                                    workflow.write().nodes.push(clone);
                                    selected_node_id.set(Some(cloned_id));
                                    selected_node_ids.set(vec![cloned_id]);
                                }
                            },
                            crate::ui::icons::CopyIcon { class: "h-3.5 w-3.5" }
                            "Duplicate"
                        }
                        button {
                            class: "flex h-8 flex-1 items-center justify-center gap-1.5 rounded-md border border-red-500/30 text-[12px] text-red-400 transition-colors hover:bg-red-500/10",
                            onclick: move |_| {
                                let snapshot = workflow.read().clone();
                                undo_stack.write().push(snapshot);
                                if undo_stack.read().len() > 60 {
                                    let _ = undo_stack.write().remove(0);
                                }
                                redo_stack.write().clear();
                                workflow.write().remove_node(node_id);
                                selected_node_id.set(None);
                                selected_node_ids.set(Vec::new());
                            },
                            crate::ui::icons::TrashIcon { class: "h-3.5 w-3.5" }
                            "Delete"
                        }
                    }
                }
            };
        }
    }

    rsx! {}
}
