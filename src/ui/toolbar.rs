use crate::ui::icons::{
    LayersIcon, MaximizeIcon, PlayIcon, RedoIcon, SaveIcon, SettingsIcon, UndoIcon, ZoomInIcon,
    ZoomOutIcon,
};
use dioxus::prelude::*;

#[component]
fn ToolbarButton(
    label: &'static str,
    disabled: bool,
    on_click: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let disabled_classes = if disabled {
        "opacity-40 pointer-events-none"
    } else {
        ""
    };

    rsx! {
        button {
            class: "flex h-8 w-8 items-center justify-center rounded-md text-slate-500 transition-all duration-100 hover:bg-slate-800 hover:text-slate-100 {disabled_classes}",
            title: "{label}",
            disabled,
            onclick: move |evt| on_click.call(evt),
            {children}
        }
    }
}

#[component]
pub fn FlowToolbar(
    workflow_name: ReadSignal<String>,
    on_workflow_name_change: EventHandler<String>,
    node_count: ReadSignal<usize>,
    edge_count: ReadSignal<usize>,
    zoom_label: ReadSignal<String>,
    on_zoom_in: EventHandler<MouseEvent>,
    on_zoom_out: EventHandler<MouseEvent>,
    on_fit_view: EventHandler<MouseEvent>,
    on_layout: EventHandler<MouseEvent>,
    on_execute: EventHandler<MouseEvent>,
    on_undo: EventHandler<MouseEvent>,
    on_redo: EventHandler<MouseEvent>,
    on_save: EventHandler<MouseEvent>,
    on_settings: EventHandler<MouseEvent>,
    can_undo: ReadSignal<bool>,
    can_redo: ReadSignal<bool>,
) -> Element {
    rsx! {
        header { class: "flex h-12 items-center justify-between border-b border-slate-800 bg-slate-900/95 px-3 backdrop-blur",
            div { class: "flex items-center gap-3",
                div { class: "flex items-center gap-2",
                    div { class: "flex h-7 w-7 items-center justify-center rounded-md bg-indigo-500/10",
                        PlayIcon { class: "ml-0.5 h-3 w-3 text-indigo-300" }
                    }
                    input {
                        r#type: "text",
                        value: "{workflow_name.read()}",
                        class: "h-7 w-auto min-w-[120px] max-w-[240px] border-none bg-transparent text-[14px] font-semibold text-slate-100 outline-none",
                        spellcheck: false,
                        oninput: move |evt| on_workflow_name_change.call(evt.value())
                    }
                }
                div { class: "flex items-center gap-2 text-[11px] text-slate-500",
                    span { class: "rounded border border-indigo-500/30 bg-indigo-500/10 px-1.5 py-0.5 text-indigo-300", "Builder" }
                    span { class: "rounded bg-slate-800 px-1.5 py-0.5 font-mono", "{node_count.read()} nodes" }
                    span { class: "rounded bg-slate-800 px-1.5 py-0.5 font-mono", "{edge_count.read()} edges" }
                    span { class: "hidden rounded bg-slate-800 px-1.5 py-0.5 font-mono lg:inline-flex", "Drag canvas to pan" }
                }
            }

            div { class: "flex items-center gap-0.5 rounded-lg border border-slate-800 bg-slate-950/70 px-1 py-0.5",
                ToolbarButton {
                    label: "Zoom Out",
                    disabled: false,
                    on_click: move |evt| on_zoom_out.call(evt),
                    ZoomOutIcon { class: "h-4 w-4" }
                }
                span { class: "min-w-[3rem] text-center font-mono text-[11px] text-slate-500", "{zoom_label.read()}" }
                ToolbarButton {
                    label: "Zoom In",
                    disabled: false,
                    on_click: move |evt| on_zoom_in.call(evt),
                    ZoomInIcon { class: "h-4 w-4" }
                }
                div { class: "mx-1 h-4 w-px bg-slate-800" }
                ToolbarButton {
                    label: "Fit View",
                    disabled: false,
                    on_click: move |evt| on_fit_view.call(evt),
                    MaximizeIcon { class: "h-4 w-4" }
                }
                div { class: "mx-1 h-4 w-px bg-slate-800" }
                ToolbarButton {
                    label: "Auto Layout",
                    disabled: false,
                    on_click: move |evt| on_layout.call(evt),
                    LayersIcon { class: "h-4 w-4" }
                }
            }

            div { class: "flex items-center gap-1",
                ToolbarButton {
                    label: "Undo",
                    disabled: !*can_undo.read(),
                    on_click: move |evt| on_undo.call(evt),
                    UndoIcon { class: "h-4 w-4" }
                }
                ToolbarButton {
                    label: "Redo",
                    disabled: !*can_redo.read(),
                    on_click: move |evt| on_redo.call(evt),
                    RedoIcon { class: "h-4 w-4" }
                }
                div { class: "mx-1 h-5 w-px bg-slate-800" }
                ToolbarButton {
                    label: "Save Workflow",
                    disabled: false,
                    on_click: move |evt| on_save.call(evt),
                    SaveIcon { class: "h-4 w-4" }
                }
                ToolbarButton {
                    label: "Settings",
                    disabled: false,
                    on_click: move |evt| on_settings.call(evt),
                    SettingsIcon { class: "h-4 w-4" }
                }
                button {
                    class: "ml-1 flex h-8 items-center gap-1.5 rounded-md bg-indigo-500 px-3 text-[12px] font-medium text-white transition-colors hover:bg-indigo-500/90",
                    title: "Run this workflow",
                    onclick: move |evt| on_execute.call(evt),
                    PlayIcon { class: "h-3.5 w-3.5" }
                    "Execute"
                }
            }
        }
    }
}
