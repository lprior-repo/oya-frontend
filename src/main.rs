use dioxus::prelude::dioxus_elements::input_data::MouseButton;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

// --- Models ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub id: Uuid,
    pub node_type: String,
    pub x: f32,
    pub y: f32,
    pub config: serde_json::Value,
    pub last_output: Option<serde_json::Value>,
    pub selected: bool,
    pub executing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub id: Uuid,
    pub source: Uuid,
    pub target: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Workflow {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub viewport: Viewport,
    pub execution_queue: Vec<Uuid>,
    pub current_step: usize,
}

impl Workflow {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            viewport: Viewport {
                x: 0.0,
                y: 0.0,
                zoom: 1.0,
            },
            execution_queue: Vec::new(),
            current_step: 0,
        }
    }

    pub fn add_node(&mut self, node_type: &str, x: f32, y: f32) -> Uuid {
        let id = Uuid::new_v4();
        self.nodes.push(Node {
            id,
            node_type: node_type.to_string(),
            x,
            y,
            config: serde_json::json!({}),
            last_output: None,
            selected: false,
            executing: false,
        });
        id
    }

    pub fn add_connection(&mut self, source: Uuid, target: Uuid) {
        if source != target
            && !self
                .connections
                .iter()
                .any(|c| c.source == source && c.target == target)
        {
            self.connections.push(Connection {
                id: Uuid::new_v4(),
                source,
                target,
            });
        }
    }

    pub fn update_node_position(&mut self, id: Uuid, dx: f32, dy: f32) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
            node.x += dx;
            node.y += dy;
        }
    }

    pub fn select_node(&mut self, id: Uuid, multi: bool) {
        if !multi {
            for n in self.nodes.iter_mut() {
                n.selected = false;
            }
        }
        if let Some(n) = self.nodes.iter_mut().find(|n| n.id == id) {
            n.selected = true;
        }
    }

    pub fn deselect_all(&mut self) {
        for n in self.nodes.iter_mut() {
            n.selected = false;
        }
    }

    pub fn remove_node(&mut self, id: Uuid) {
        self.nodes.retain(|n| n.id != id);
        self.connections
            .retain(|c| c.source != id && c.target != id);
    }

    pub fn zoom(&mut self, delta: f32, cx: f32, cy: f32) {
        let old_zoom = self.viewport.zoom;
        let new_zoom = (old_zoom * (1.0 + delta)).clamp(0.1, 5.0);
        let factor = new_zoom / old_zoom;
        self.viewport.x = cx - (cx - self.viewport.x) * factor;
        self.viewport.y = cy - (cy - self.viewport.y) * factor;
        self.viewport.zoom = new_zoom;
    }

    pub fn prepare_run(&mut self) {
        let mut queue = Vec::new();
        let mut visited = HashSet::new();

        let mut processing: Vec<Uuid> = self
            .nodes
            .iter()
            .filter(|n| !self.connections.iter().any(|c| c.target == n.id))
            .map(|n| n.id)
            .collect();

        if processing.is_empty() && !self.nodes.is_empty() {
            processing.push(self.nodes[0].id);
        }

        while let Some(id) = processing.pop() {
            if visited.insert(id) {
                queue.push(id);
                let targets: Vec<Uuid> = self
                    .connections
                    .iter()
                    .filter(|c| c.source == id)
                    .map(|c| c.target)
                    .collect();
                processing.extend(targets);
            }
        }

        for node in &self.nodes {
            if !visited.contains(&node.id) {
                queue.push(node.id);
            }
        }

        self.execution_queue = queue;
        self.current_step = 0;
        for node in self.nodes.iter_mut() {
            node.executing = false;
            node.last_output = None;
        }
    }

    pub fn step(&mut self) -> bool {
        if self.current_step >= self.execution_queue.len() {
            for node in self.nodes.iter_mut() {
                node.executing = false;
            }
            return false;
        }

        for node in self.nodes.iter_mut() {
            node.executing = false;
        }

        let node_id = self.execution_queue[self.current_step];

        let parent_outputs: Vec<serde_json::Value> = self
            .connections
            .iter()
            .filter(|c| c.target == node_id)
            .filter_map(|c| {
                self.nodes
                    .iter()
                    .find(|n| n.id == c.source)?
                    .last_output
                    .clone()
            })
            .collect();

        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.executing = true;

            let output = match node.node_type.as_str() {
                "Trigger" => serde_json::json!({
                    "timestamp": "2026-02-19T12:00:00Z",
                    "source": "manual"
                }),
                "JSON Transform" => {
                    serde_json::json!({
                        "transformed": true,
                        "data": parent_outputs
                    })
                }
                "HTTP Request" => {
                    serde_json::json!({
                        "status": 200,
                        "url": node.config.get("url").and_then(|v| v.as_str()).unwrap_or("https://api.mock"),
                        "body": parent_outputs.first().cloned().unwrap_or(serde_json::json!({}))
                    })
                }
                _ => serde_json::json!({
                    "executed": true,
                    "step": self.current_step,
                    "input_count": parent_outputs.len()
                }),
            };
            node.last_output = Some(output);
        }

        self.current_step += 1;
        true
    }

    pub fn run(&mut self) {
        self.prepare_run();
        while self.step() {}
    }
}

// --- Components ---

#[component]
fn NodeCard(
    node: Node,
    on_drag: EventHandler<()>,
    on_select: EventHandler<bool>,
    on_delete: EventHandler<()>,
    on_pin_down: EventHandler<()>,
    on_pin_up: EventHandler<()>,
) -> Element {
    let (color, icon) = match node.node_type.as_str() {
        "Trigger" => (
            "amber",
            rsx! {
                svg { fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", class: "w-4 h-4",
                    path { d: "M13 10V3L4 14h7v7l9-11h-7z", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            },
        ),
        "HTTP Request" => (
            "indigo",
            rsx! {
                svg { fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", class: "w-4 h-4",
                    path { d: "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            },
        ),
        "JSON Transform" => (
            "violet",
            rsx! {
                svg { fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", class: "w-4 h-4",
                    path { d: "M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            },
        ),
        _ => (
            "slate",
            rsx! {
                svg { fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", class: "w-4 h-4",
                    path { d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            },
        ),
    };

    let border = if node.executing {
        format!("border-emerald-500 ring-4 ring-emerald-500/20")
    } else if node.selected {
        format!("border-{color}-500 ring-4 ring-{color}-500/10")
    } else {
        format!("border-slate-700 shadow-xl")
    };

    rsx! {
        div {
            class: "absolute bg-slate-800 border-2 {border} rounded-2xl w-52 transition-all hover:border-{color}-400 group z-10",
            style: "left: {node.x}px; top: {node.y}px; cursor: grab active:cursor-grabbing",
            onmousedown: move |evt| {
                evt.stop_propagation();
                on_drag.call(());
                on_select.call(evt.modifiers().shift());
            },
            div { class: "p-3 border-b border-slate-700 flex justify-between items-center bg-slate-900/50 rounded-t-2xl",
                div { class: "flex items-center gap-2",
                    div { class: "text-{color}-400", {icon} }
                    span { class: "text-[10px] uppercase tracking-widest font-bold text-slate-400", "{node.node_type}" }
                }
                button { class: "text-slate-600 hover:text-red-400", onclick: move |_| on_delete.call(()), "×" }
            }
            div { class: "p-4 min-h-[64px] flex flex-col justify-center",
                if let Some(out) = &node.last_output {
                    div { class: "text-[10px] font-mono text-emerald-400 truncate", "{out}" }
                } else {
                    div { class: "text-[10px] text-slate-500 italic", "Ready" }
                }
            }
            div {
                class: "absolute -left-2 top-1/2 -translate-y-1/2 w-4 h-4 bg-slate-700 border-2 border-slate-900 rounded-full hover:bg-{color}-500 cursor-pointer z-20",
                onmouseup: move |_| on_pin_up.call(())
            }
            div {
                class: "absolute -right-2 top-1/2 -translate-y-1/2 w-4 h-4 bg-slate-700 border-2 border-slate-900 rounded-full hover:bg-{color}-500 cursor-pointer z-20",
                onmousedown: move |evt| { evt.stop_propagation(); on_pin_down.call(()); }
            }
        }
    }
}

#[component]
fn CommandPalette(on_select: EventHandler<String>, on_close: EventHandler<()>) -> Element {
    let mut search = use_signal(|| "".to_string());
    let node_types = vec![
        "HTTP Request",
        "Trigger",
        "JSON Transform",
        "Webhook",
        "Schedule",
    ];

    let filtered: Vec<String> = node_types
        .into_iter()
        .filter(|t| t.to_lowercase().contains(&search.read().to_lowercase()))
        .map(|t| t.to_string())
        .collect();

    rsx! {
        div {
            class: "fixed inset-0 z-[100] flex items-start justify-center pt-32 bg-slate-950/80 backdrop-blur-sm px-4",
            onmousedown: move |_| on_close.call(()),
            div {
                class: "w-full max-w-[500px] bg-slate-900 border border-slate-700 rounded-2xl shadow-2xl overflow-hidden",
                onmousedown: move |evt| evt.stop_propagation(),
                div { class: "p-4 border-b border-slate-800",
                    input {
                        class: "w-full bg-transparent text-xl outline-none placeholder-slate-600 text-white",
                        placeholder: "Type a node name...",
                        autofocus: true,
                        value: "{search}",
                        oninput: move |evt| search.set(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter && !filtered.is_empty() {
                                on_select.call(filtered[0].clone());
                            }
                            if evt.key() == Key::Escape {
                                on_close.call(());
                            }
                        }
                    }
                }
                div { class: "max-h-96 overflow-y-auto p-2 flex flex-col gap-1",
                    for t in filtered.clone() {
                        div {
                            class: "p-3 hover:bg-indigo-600/30 rounded-xl cursor-pointer flex justify-between items-center group transition-colors",
                            onclick: move |_| on_select.call(t.clone()),
                            span { class: "font-medium", "{t}" }
                            span { class: "text-[10px] bg-slate-800 text-slate-500 group-hover:bg-indigo-500 group-hover:text-white px-2 py-1 rounded uppercase tracking-tighter", "Node" }
                        }
                    }
                    if filtered.is_empty() {
                        div { class: "p-8 text-center text-slate-500 italic text-sm", "No results found" }
                    }
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    let mut workflow = use_signal(Workflow::new);
    let mut dragging_node = use_signal(|| None::<Uuid>);
    let mut is_panning = use_signal(|| false);
    let mut last_mouse_pos = use_signal(|| (0.0, 0.0));
    let mut connecting_from = use_signal(|| None::<Uuid>);
    let mut show_palette = use_signal(|| false);

    let wf = workflow.read();
    let vp = wf.viewport.clone();
    let selected_node = wf.nodes.iter().find(|n| n.selected).cloned();

    rsx! {
        script { src: "https://cdn.tailwindcss.com" }
        style {
            "@keyframes dash {{ to {{ stroke-dashoffset: -16; }} }}"
        }
        div {
            class: "flex h-screen w-screen bg-slate-950 text-slate-200 font-sans select-none overflow-hidden",
            onkeydown: move |evt| {
                if (evt.modifiers().meta() || evt.modifiers().ctrl()) && evt.key() == Key::Character("k".into()) {
                    show_palette.toggle();
                }
                if evt.key() == Key::Escape {
                    show_palette.set(false);
                }
            },
            if *show_palette.read() {
                CommandPalette {
                    on_select: move |node_type: String| {
                        workflow.write().add_node(&node_type, 200.0, 200.0);
                        show_palette.set(false);
                    },
                    on_close: move |_| show_palette.set(false)
                }
            }
            aside { class: "w-64 bg-slate-900 border-r border-slate-800 p-6 flex flex-col gap-6 shadow-2xl z-20",
                div { class: "flex flex-col gap-2",
                    button {
                        class: "w-full py-3 bg-indigo-600 hover:bg-indigo-500 rounded-xl font-bold transition-all transform active:scale-95",
                        onclick: move |_| { workflow.write().add_node("Trigger", 100.0, 100.0); },
                        "+ Add Node"
                    }
                    p { class: "text-[9px] text-center text-slate-500 uppercase tracking-widest", "Tip: Press CMD+K" }
                }

                div { class: "flex flex-col gap-2 mt-auto pt-6 border-t border-slate-800",
                    button {
                        class: "w-full py-3 bg-slate-800 hover:bg-slate-700 rounded-xl font-bold flex items-center justify-center gap-2",
                        onclick: move |_| {
                            let json = serde_json::to_string_pretty(&*workflow.read()).unwrap();
                            #[cfg(target_arch = "wasm32")]
                            {
                                use wasm_bindgen::JsCast;
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        let blob = web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(&json.into())).unwrap();
                                        let url = window.url().create_object_url_with_blob(&blob).unwrap();
                                        let a = document.create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                                        a.set_href(&url);
                                        a.set_download("workflow.json");
                                        a.click();
                                        let _ = window.url().revoke_object_url(&url);
                                    }
                                }
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            println!("Exported JSON: {}", json);
                        },
                        "📤 Export"
                    }
                    label {
                        class: "w-full py-3 bg-slate-800 hover:bg-slate-700 rounded-xl font-bold flex items-center justify-center gap-2 cursor-pointer",
                        "📥 Import"
                        input {
                            r#type: "file",
                            class: "hidden",
                            accept: ".json",
                            onchange: move |evt| {
                                async move {
                                    if let Some(file) = evt.files().get(0) {
                                        if let Ok(content) = file.read().await {
                                            if let Ok(content_str) = String::from_utf8(content) {
                                                if let Ok(parsed) = serde_json::from_str::<Workflow>(&content_str) {
                                                    workflow.set(parsed);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "flex flex-col gap-2",
                    button {
                        class: "w-full py-3 bg-emerald-600 hover:bg-emerald-500 rounded-xl font-bold transition-all",
                        onclick: move |_| {
                            let mut wf = workflow.write();
                            if wf.current_step >= wf.execution_queue.len() || wf.execution_queue.is_empty() {
                                wf.prepare_run();
                            }
                            wf.step();
                        },
                        "⏭ Step"
                    }
                    button {
                        class: "w-full py-3 bg-slate-800 hover:bg-slate-700 rounded-xl font-bold",
                        onclick: move |_| { workflow.write().run(); },
                        "▶ Run"
                    }
                }
            }

            main {
                class: "relative flex-1 overflow-hidden bg-slate-950",
                onmousedown: move |evt| {
                    if evt.trigger_button() == Some(MouseButton::Auxiliary) || evt.modifiers().shift() {
                        is_panning.set(true);
                    } else {
                        workflow.write().deselect_all();
                    }
                    let coord = evt.page_coordinates();
                    last_mouse_pos.set((coord.x as f32, coord.y as f32));
                },
                onmousemove: move |evt| {
                    let coord = evt.page_coordinates();
                    let (mx, my) = (coord.x as f32, coord.y as f32);
                    let (lx, ly) = last_mouse_pos.read().clone();
                    let dx = mx - lx;
                    let dy = my - ly;
                    if *is_panning.read() {
                        let mut wf = workflow.write();
                        wf.viewport.x += dx;
                        wf.viewport.y += dy;
                    } else if let Some(id) = dragging_node.read().clone() {
                        let zoom = workflow.read().viewport.zoom;
                        workflow.write().update_node_position(id, dx / zoom, dy / zoom);
                    }
                    last_mouse_pos.set((mx, my));
                },
                onmouseup: move |_| {
                    is_panning.set(false);
                    dragging_node.set(None);
                },
                onwheel: move |evt| {
                    let delta = -evt.delta().strip_units().y as f32 * 0.001;
                    workflow.write().zoom(delta, 640.0, 400.0);
                },

                div {
                    class: "absolute inset-0 origin-top-left",
                    style: "transform: translate({vp.x}px, {vp.y}px) scale({vp.zoom});",

                    div {
                        class: "absolute inset-[-10000px] pointer-events-none opacity-20",
                        style: "background-image: radial-gradient(circle, #4f46e5 1px, transparent 1px); background-size: 40px 40px;"
                    }

                    svg { class: "absolute inset-0 pointer-events-none w-[10000px] h-[10000px]",
                        for conn in workflow.read().connections.iter() {
                            if let (Some(s), Some(t)) = (
                                workflow.read().nodes.iter().find(|n| n.id == conn.source),
                                workflow.read().nodes.iter().find(|n| n.id == conn.target)
                            ) {
                                path {
                                    key: "{conn.id}",
                                    d: "M {s.x + 208.0} {s.y + 40.0} C {s.x + 268.0} {s.y + 40.0}, {t.x - 60.0} {t.y + 40.0}, {t.x} {t.y + 40.0}",
                                    stroke: if s.executing { "#10b981" } else { "#6366f1" },
                                    stroke_width: "3",
                                    fill: "none",
                                    class: if s.executing { "opacity-100" } else { "opacity-40" },
                                    style: if s.executing { "stroke-dasharray: 8; animation: dash 1s linear infinite;" } else { "" }
                                }
                                if let Some(out) = &s.last_output {
                                    foreignObject {
                                        x: "{(s.x + 208.0 + t.x) / 2.0 - 40.0}",
                                        y: "{(s.y + 40.0 + t.y + 40.0) / 2.0 - 10.0}",
                                        width: "80",
                                        height: "20",
                                        div {
                                            class: "bg-slate-900/90 border border-indigo-500/50 rounded-full text-[7px] font-mono text-indigo-300 px-2 py-0.5 truncate shadow-lg backdrop-blur-sm",
                                            "{out}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    for node in workflow.read().nodes.iter().cloned() {
                        NodeCard {
                            node: node.clone(),
                            on_drag: move |_| { dragging_node.set(Some(node.id)); },
                            on_select: move |multi| { workflow.write().select_node(node.id, multi); },
                            on_delete: move |_| { workflow.write().remove_node(node.id); },
                            on_pin_down: move |_| { connecting_from.set(Some(node.id)); },
                            on_pin_up: move |_| {
                                let src_opt = *connecting_from.read();
                                if let Some(src) = src_opt {
                                    workflow.write().add_connection(src, node.id);
                                    connecting_from.set(None);
                                }
                            }
                        }
                    }
                }
            }

            if let Some(node) = selected_node {
                aside { class: "w-96 bg-slate-900 border-l border-slate-800 p-6 flex flex-col z-30 shadow-2xl",
                    h3 { class: "text-sm font-bold mb-6", "Node Settings" }
                    div { class: "flex flex-col gap-2",
                        label { class: "text-[10px] uppercase font-bold text-slate-500", "Parameter" }
                        textarea { class: "w-full h-48 bg-slate-950 border border-slate-700 p-4 rounded-xl text-[11px] font-mono", value: "{node.config}" }
                    }
                    button { class: "mt-auto w-full py-3 bg-indigo-600 rounded-xl font-bold", onclick: move |_| { workflow.write().deselect_all(); }, "Close" }
                }
            }
        }
    }
}

fn main() {
    launch(App);
}
