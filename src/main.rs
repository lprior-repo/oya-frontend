#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use dioxus::prelude::dioxus_elements::input_data::MouseButton;
use dioxus::prelude::*;
use new_app::graph::{Node, NodeId, Workflow};
use uuid::Uuid;

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
        "Function" => (
            "fuchsia",
            rsx! {
                svg { fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", class: "w-4 h-4",
                    path { d: "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            },
        ),
        "If" => (
            "orange",
            rsx! {
                svg { fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", class: "w-4 h-4",
                    path { d: "M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            },
        ),
        "Schedule" => (
            "cyan",
            rsx! {
                svg { fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", class: "w-4 h-4",
                    path { d: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            },
        ),
        _ => (
            "slate",
            rsx! {
                svg { fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", class: "w-4 h-4",
                    path { d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 v2M7 7h10", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            },
        ),
    };

    let border = if node.error.is_some() {
        "border-red-500 ring-4 ring-red-500/20".to_string()
    } else if node.executing {
        "border-emerald-500 ring-4 ring-emerald-500/20".to_string()
    } else if node.selected {
        format!("border-{color}-500 ring-4 ring-{color}-500/10")
    } else if node.skipped {
        "border-slate-800 opacity-40".to_string()
    } else {
        "border-slate-700 shadow-xl".to_string()
    };

    rsx! {
        div {
            class: "absolute bg-slate-800 border-2 {border} rounded-2xl w-52 transition-all hover:border-{color}-400 group z-10 node-card",
            style: "left: {node.x}px; top: {node.y}px; cursor: grab active:cursor-grabbing",
            onmousedown: move |evt| {
                evt.stop_propagation();
                on_drag.call(());
                on_select.call(evt.modifiers().shift());
            },
            div { class: "p-3 border-b border-slate-700 flex justify-between items-center bg-slate-900/50 rounded-t-2xl",
                div { class: "flex items-center gap-2",
                    div { class: "text-{color}-400", {icon} }
                    span { class: "text-[10px] uppercase tracking-widest font-bold text-slate-400", "{node.name}" }
                }
                button { class: "text-slate-600 hover:text-red-400", onclick: move |_| on_delete.call(()), "×" }
            }
            div { class: "p-4 min-h-[64px] flex flex-col justify-center",
                if let Some(err) = &node.error {
                    div { class: "text-[10px] font-bold text-red-400", "{err}" }
                } else if let Some(out) = &node.last_output {
                    div { class: "text-[10px] font-mono text-emerald-400 truncate wire-data", "{out}" }
                } else {
                    div { class: "text-[10px] text-slate-500 italic", if node.skipped { "Skipped" } else { "Ready" } }
                }
            }
            // Input Pin
            div {
                class: "absolute -left-2 top-1/2 -translate-y-1/2 w-4 h-4 bg-slate-700 border-2 border-slate-900 rounded-full hover:bg-{color}-500 cursor-pointer z-20",
                onmouseup: move |_| on_pin_up.call(())
            }
            // Output Pin(s)
            if node.node_type == "If" {
                div { class: "absolute -right-2 top-1/4 -translate-y-1/2 flex flex-col gap-4",
                    div {
                        class: "w-4 h-4 bg-emerald-600 border-2 border-slate-900 rounded-full hover:bg-emerald-400 cursor-pointer z-20",
                        onmousedown: move |evt| { evt.stop_propagation(); on_pin_down.call(()); }
                    }
                    div {
                        class: "w-4 h-4 bg-red-600 border-2 border-slate-900 rounded-full hover:bg-red-400 cursor-pointer z-20",
                        onmousedown: move |evt| { evt.stop_propagation(); on_pin_down.call(()); }
                    }
                }
            } else {
                div {
                    class: "absolute -right-2 top-1/2 -translate-y-1/2 w-4 h-4 bg-slate-700 border-2 border-slate-900 rounded-full hover:bg-{color}-500 cursor-pointer z-20",
                    onmousedown: move |evt| { evt.stop_propagation(); on_pin_down.call(()); }
                }
            }
        }
    }
}

#[component]
fn NodeConfigEditor(node: Node, on_change: EventHandler<serde_json::Value>) -> Element {
    let config = node.config.clone();
    
    rsx! {
        div { class: "flex flex-col gap-4",
            match node.node_type.as_str() {
                "HTTP Request" => {
                    let method = config.get("method").and_then(serde_json::Value::as_str).unwrap_or("GET").to_string();
                    let url = config.get("url").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
                    let cfg_clone1 = config.clone();
                    let cfg_clone2 = config.clone();
                    rsx! {
                        div { class: "flex flex-col gap-2",
                            label { class: "text-[10px] uppercase font-bold text-slate-500", "Method" }
                            select {
                                class: "bg-slate-950 border border-slate-700 p-2 rounded-xl text-[11px] text-white",
                                value: "{method}",
                                onchange: move |evt| {
                                    let mut new_config = cfg_clone1.clone();
                                    if let Some(obj) = new_config.as_object_mut() {
                                        obj.insert("method".to_string(), serde_json::Value::String(evt.value()));
                                        on_change.call(new_config);
                                    }
                                },
                                option { value: "GET", "GET" }
                                option { value: "POST", "POST" }
                                option { value: "PUT", "PUT" }
                                option { value: "DELETE", "DELETE" }
                            }
                        }
                        div { class: "flex flex-col gap-2",
                            label { class: "text-[10px] uppercase font-bold text-slate-500", "URL" }
                            input {
                                class: "bg-slate-950 border border-slate-700 p-2 rounded-xl text-[11px] text-white",
                                value: "{url}",
                                oninput: move |evt| {
                                    let mut new_config = cfg_clone2.clone();
                                    if let Some(obj) = new_config.as_object_mut() {
                                        obj.insert("url".to_string(), serde_json::Value::String(evt.value()));
                                        on_change.call(new_config);
                                    }
                                }
                            }
                        }
                    }
                },
                "Function" => {
                    let mapping = config.get("mapping").cloned().unwrap_or_else(|| serde_json::json!({}));
                    let cfg_clone = config.clone();
                    rsx! {
                        div { class: "flex flex-col gap-2",
                            label { class: "text-[10px] uppercase font-bold text-slate-500", "Result Mapping" }
                            textarea {
                                class: "w-full h-48 bg-slate-950 border border-slate-700 p-4 rounded-xl text-[11px] font-mono text-white",
                                value: "{mapping}",
                                oninput: move |evt| {
                                    if let Ok(parsed) = serde_json::from_str(&evt.value()) {
                                        let mut new_config = cfg_clone.clone();
                                        if let Some(obj) = new_config.as_object_mut() {
                                            obj.insert("mapping".to_string(), parsed);
                                            on_change.call(new_config);
                                        }
                                    }
                                }
                            }
                            span { class: "text-[9px] text-slate-600", "Format: {{ \"key\": \"{{{{ expression }}}}\" }}" }
                        }
                    }
                },
                "If" => {
                    let condition = config.get("condition").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
                    let cfg_clone = config.clone();
                    rsx! {
                        div { class: "flex flex-col gap-2",
                            label { class: "text-[10px] uppercase font-bold text-slate-500", "Condition" }
                            input {
                                class: "bg-slate-950 border border-slate-700 p-2 rounded-xl text-[11px] text-white font-mono",
                                placeholder: "{{ $node[\"Trigger\"].json.data }}",
                                value: "{condition}",
                                oninput: move |evt| {
                                    let mut new_config = cfg_clone.clone();
                                    if let Some(obj) = new_config.as_object_mut() {
                                        obj.insert("condition".to_string(), serde_json::Value::String(evt.value()));
                                        on_change.call(new_config);
                                    }
                                }
                            }
                        }
                    }
                },
                "Schedule" => {
                    let interval = config.get("interval_sec").and_then(serde_json::Value::as_u64).unwrap_or(60);
                    let cfg_clone = config.clone();
                    rsx! {
                        div { class: "flex flex-col gap-2",
                            label { class: "text-[10px] uppercase font-bold text-slate-500", "Interval (Seconds)" }
                            input {
                                r#type: "number",
                                class: "bg-slate-950 border border-slate-700 p-2 rounded-xl text-[11px] text-white",
                                value: "{interval}",
                                oninput: move |evt| {
                                    let mut new_config = cfg_clone.clone();
                                    if let Some(obj) = new_config.as_object_mut() {
                                        if let Ok(n) = evt.value().parse::<u64>() {
                                            obj.insert("interval_sec".to_string(), serde_json::Value::Number(n.into()));
                                            on_change.call(new_config);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    let cfg_str = config.to_string();
                    rsx! {
                        div { class: "flex flex-col gap-2",
                            label { class: "text-[10px] uppercase font-bold text-slate-500", "JSON Config" }
                            textarea {
                                class: "w-full h-48 bg-slate-950 border border-slate-700 p-4 rounded-xl text-[11px] font-mono text-white",
                                value: "{cfg_str}",
                                oninput: move |evt| {
                                    if let Ok(parsed) = serde_json::from_str(&evt.value()) {
                                        on_change.call(parsed);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if config.to_string().contains("{{") {
                div { class: "flex items-center gap-2 mt-2",
                    div { class: "w-2 h-2 rounded-full bg-indigo-500 animate-pulse" }
                    span { class: "text-[10px] text-indigo-400 font-bold", "Expression Active" }
                }
            }
        }
    }
}

#[component]
fn CommandPalette(on_select: EventHandler<String>, on_close: EventHandler<()>) -> Element {
    let mut search = use_signal(String::new);
    let node_types = [
        "HTTP Request",
        "Trigger",
        "JSON Transform",
        "Function",
        "If",
        "Webhook",
        "Schedule",
    ];

    let filtered: Vec<String> = node_types
        .iter()
        .filter(|t| t.to_lowercase().contains(&search.read().to_lowercase()))
        .map(std::string::ToString::to_string)
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
                            span { class: "font-medium text-white", "{t}" }
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
    let mut workflow = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(json)) = storage.get_item("flow_wasm_workflow") {
                        if let Ok(parsed) = serde_json::from_str::<Workflow>(&json) {
                            return parsed;
                        }
                    }
                }
            }
        }
        Workflow::new()
    });

    let mut dragging_node = use_signal(|| None::<NodeId>);
    let mut is_panning = use_signal(|| false);
    let mut last_mouse_pos = use_signal(|| (0.0, 0.0));
    let mut connecting_from = use_signal(|| None::<NodeId>);
    let mut show_palette = use_signal(|| false);
    let mut phantom_wire = use_signal(|| None::<((f32, f32), (f32, f32))>);
    let mut selection_box = use_signal(|| None::<((f32, f32), (f32, f32))>);
    let mut active_tab = use_signal(|| "library".to_string());
    let mut toasts = use_signal(Vec::<(Uuid, String, String)>::new);

    // Toast Helper
    let add_toast = move |title: &str, kind: &str| {
        let id = Uuid::new_v4();
        toasts.write().push((id, title.to_string(), kind.to_string()));
        spawn(async move {
            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::sleep(std::time::Duration::from_secs(3)).await;
            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            
            toasts.write().retain(|(tid, _, _)| *tid != id);
        });
    };

    // Schedule Loop
    use_future(move || async move {
        loop {
            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            let mut wf_write = workflow.write();
            let mut should_run = false;
            
            for node in &wf_write.nodes {
                if node.node_type == "Schedule" {
                    let interval = node.config.get("interval_sec").and_then(serde_json::Value::as_u64).unwrap_or(60);
                    let last_run = node.last_output.as_ref()
                        .and_then(|o| o.get("timestamp"))
                        .and_then(serde_json::Value::as_str)
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok());
                    
                    let now = chrono::Utc::now();
                    let elapsed = last_run.map_or(interval, |lr| (now - lr.with_timezone(&chrono::Utc)).num_seconds().max(0).unsigned_abs());
                    
                    if elapsed >= interval {
                        should_run = true;
                        break;
                    }
                }
            }
            
            if should_run {
                wf_write.run().await;
            }
        }
    });

    use_effect(move || {
        let wf = workflow.read();
        let _json = serde_json::to_string(&*wf).unwrap_or_default();
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("flow_wasm_workflow", &_json);
            }
        }
    });

    let wf = workflow.read();
    let vp = wf.viewport.clone();
    let selected_node = wf.nodes.iter().find(|n| n.selected).cloned();

    rsx! {
        script { src: "https://cdn.tailwindcss.com" }
        style {
            "@keyframes dash {{ to {{ stroke-dashoffset: -16; }} }}"
            "@keyframes slide-in {{ from {{ transform: translateX(100%); opacity: 0; }} to {{ transform: translateX(0); opacity: 1; }} }}"
            ".animate-slide-in {{ animation: slide-in 0.3s ease-out; }}"
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
                    on_close: move |()| show_palette.set(false)
                }
            }
            aside { class: "w-64 bg-slate-900 border-r border-slate-800 p-6 flex flex-col gap-6 shadow-2xl z-20",
                div { class: "flex border-b border-slate-800 mb-2",
                    {
                        let base = "flex-1 py-2 text-[10px] uppercase font-bold";
                        let is_lib = *active_tab.read() == "library";
                        let is_hist = *active_tab.read() == "history";
                        let lib_class = if is_lib { format!("{base} text-indigo-400 border-b-2 border-indigo-400") }
                                        else { format!("{base} text-slate-500") };
                        let hist_class = if is_hist { format!("{base} text-indigo-400 border-b-2 border-indigo-400") }
                                         else { format!("{base} text-slate-500") };
                        rsx! {
                            button { 
                                class: "{lib_class}",
                                onclick: move |_| active_tab.set("library".to_string()),
                                "Library"
                            }
                            button { 
                                class: "{hist_class}",
                                onclick: move |_| active_tab.set("history".to_string()),
                                "History"
                            }
                        }
                    }
                }
                if *active_tab.read() == "library" {
                    div { class: "flex flex-col gap-2",
                        button {
                            class: "w-full py-3 bg-indigo-600 hover:bg-indigo-500 rounded-xl font-bold transition-all transform active:scale-95 text-white",
                            onclick: move |_| { workflow.write().add_node("Trigger", 100.0, 100.0); },
                            "+ Add Node"
                        }
                        p { class: "text-[9px] text-center text-slate-500 uppercase tracking-widest", "Tip: Press CMD+K" }
                    }
                } else {
                    div { class: "flex flex-col gap-2 overflow-y-auto",
                        for record in workflow.read().history.iter().rev() {
                            {
                                let time_str = record.timestamp.format("%H:%M:%S").to_string();
                                let status_color = if record.success { "bg-emerald-500" } else { "bg-red-500" };
                                let count = record.results.len();
                                rsx! {
                                    div { class: "p-3 bg-slate-950/50 border border-slate-800 rounded-xl flex flex-col gap-1",
                                        div { class: "flex justify-between items-center",
                                            span { class: "text-[9px] text-slate-400", "{time_str}" }
                                            div { class: "w-2 h-2 rounded-full {status_color}" }
                                        }
                                        span { class: "text-[10px] text-slate-500", "{count} nodes executed" }
                                    }
                                }
                            }
                        }
                        if workflow.read().history.is_empty() {
                            div { class: "text-center text-[10px] text-slate-600 italic py-8", "No execution history" }
                        }
                    }
                }

                div { class: "flex flex-col gap-2 mt-auto pt-6 border-t border-slate-800",
                    button {
                        class: "w-full py-3 bg-slate-800 hover:bg-slate-700 rounded-xl font-bold flex items-center justify-center gap-2 text-white",
                        onclick: move |_| {
                            let wf_data = workflow.read();
                            let _json_pretty = serde_json::to_string_pretty(&*wf_data).unwrap_or_default();
                            #[cfg(target_arch = "wasm32")]
                            {
                                use wasm_bindgen::JsCast;
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        let parts = js_sys::Array::of1(&_json_pretty.into());
                                        let blob = web_sys::Blob::new_with_str_sequence(&parts).unwrap();
                                        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                                        let a = document.create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                                        a.set_href(&url);
                                        a.set_download("workflow.json");
                                        a.click();
                                        let _ = web_sys::Url::revoke_object_url(&url);
                                    }
                                }
                            }
                        },
                        "📤 Export"
                    }
                }

                div { class: "flex flex-col gap-2",
                    button {
                        class: "w-full py-3 bg-emerald-600 hover:bg-emerald-500 rounded-xl font-bold transition-all text-white run-btn",
                        onclick: move |_| {
                            let mut t = add_toast;
                            spawn(async move {
                                let mut wf_write = workflow.write();
                                if wf_write.current_step >= wf_write.execution_queue.len() || wf_write.execution_queue.is_empty() {
                                    wf_write.prepare_run();
                                }
                                wf_write.step().await;
                                t("Step executed", "info");
                            });
                        },
                        "⏭ Step"
                    }
                    button {
                        class: "w-full py-3 bg-slate-800 hover:bg-slate-700 rounded-xl font-bold text-white",
                        onclick: move |_| {
                            let mut t = add_toast;
                            spawn(async move {
                                workflow.write().run().await;
                                t("Workflow run complete", "success");
                            });
                        },
                        "▶ Run"
                    }
                }
            }

            main {
                class: "relative flex-1 overflow-hidden bg-slate-950 canvas",
                onmousedown: move |evt| {
                    let coord = evt.page_coordinates();
                    #[allow(clippy::cast_possible_truncation)]
                    let (mx, my) = (coord.x as f32, coord.y as f32);

                    if evt.trigger_button() == Some(MouseButton::Auxiliary) || evt.modifiers().shift() {
                        is_panning.set(true);
                    } else {
                        workflow.write().deselect_all();
                        let current_vp = workflow.read().viewport.clone();
                        let canvas_x = (mx - current_vp.x) / current_vp.zoom;
                        let canvas_y = (my - current_vp.y) / current_vp.zoom;
                        selection_box.set(Some(((canvas_x, canvas_y), (canvas_x, canvas_y))));
                    }
                    last_mouse_pos.set((mx, my));
                },
                onmousemove: move |evt| {
                    let coord = evt.page_coordinates();
                    #[allow(clippy::cast_possible_truncation)]
                    let (mx, my) = (coord.x as f32, coord.y as f32);
                    let (lx, ly) = *last_mouse_pos.read();
                    let dx = mx - lx;
                    let dy = my - ly;
                    let current_vp = workflow.read().viewport.clone();
                    let zoom = current_vp.zoom;

                    if *is_panning.read() {
                        let mut wf_write = workflow.write();
                        wf_write.viewport.x += dx;
                        wf_write.viewport.y += dy;
                    } else if let Some(id) = *dragging_node.read() {
                        let mut wf_write = workflow.write();
                        let selected_ids: Vec<NodeId> = wf_write.nodes.iter()
                            .filter(|n| n.selected)
                            .map(|n| n.id)
                            .collect();

                        if selected_ids.contains(&id) {
                            for nid in selected_ids {
                                wf_write.update_node_position(nid, dx / zoom, dy / zoom);
                            }
                        } else {
                            wf_write.update_node_position(id, dx / zoom, dy / zoom);
                        }
                    } else if let Some(id) = *connecting_from.read() {
                        if let Some(node) = workflow.read().nodes.iter().find(|n| n.id == id) {
                            let start = (node.x + 208.0, node.y + 40.0);
                            let end = ((mx - current_vp.x) / zoom, (my - current_vp.y) / zoom);
                            phantom_wire.set(Some((start, end)));
                        }
                    } else {
                        let sb = *selection_box.read();
                        if let Some((start, _)) = sb {
                            let end = ((mx - current_vp.x) / zoom, (my - current_vp.y) / zoom);
                            selection_box.set(Some((start, end)));
                        }
                    }
                    last_mouse_pos.set((mx, my));
                },
                onmouseup: move |_| {
                    if let Some((start, end)) = *selection_box.read() {
                        let x1 = start.0.min(end.0);
                        let y1 = start.1.min(end.1);
                        let x2 = start.0.max(end.0);
                        let y2 = start.1.max(end.1);

                        let mut wf_write = workflow.write();
                        for node in &mut wf_write.nodes {
                            if node.x >= x1 && node.x <= x2 && node.y >= y1 && node.y <= y2 {
                                node.selected = true;
                            }
                        }
                    }
                    is_panning.set(false);
                    dragging_node.set(None);
                    connecting_from.set(None);
                    phantom_wire.set(None);
                    selection_box.set(None);
                },
                onwheel: move |evt| {
                    #[allow(clippy::cast_possible_truncation)]
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
                            }
                        }
                        if let Some(((sx, sy), (ex, ey))) = *phantom_wire.read() {
                            path {
                                d: "M {sx} {sy} C {sx + 60.0} {sy}, {ex - 60.0} {ey}, {ex} {ey}",
                                stroke: "#6366f1",
                                stroke_width: "2",
                                fill: "none",
                                class: "opacity-50",
                                style: "stroke-dasharray: 4;"
                            }
                        }
                        if let Some((start, end)) = *selection_box.read() {
                            {
                                let x = start.0.min(end.0);
                                let y = start.1.min(end.1);
                                let w = (start.0 - end.0).abs();
                                let h = (start.1 - end.1).abs();
                                rsx! {
                                    rect {
                                        x: "{x}",
                                        y: "{y}",
                                        width: "{w}",
                                        height: "{h}",
                                        fill: "rgba(99, 102, 241, 0.1)",
                                        stroke: "#6366f1",
                                        stroke_width: "1",
                                        style: "stroke-dasharray: 4;"
                                    }
                                }
                            }
                        }
                    }

                    for node in workflow.read().nodes.iter().cloned() {
                        NodeCard {
                            node: node.clone(),
                            on_drag: move |()| { dragging_node.set(Some(node.id)); },
                            on_select: move |multi| { workflow.write().select_node(node.id, multi); },
                            on_delete: move |()| { workflow.write().remove_node(node.id); },
                            on_pin_down: move |()| { connecting_from.set(Some(node.id)); },
                            on_pin_up: move |()| {
                                let src_opt = *connecting_from.read();
                                if let Some(src) = src_opt {
                                    workflow.write().add_connection(src, node.id, &"main".into(), &"main".into());
                                    connecting_from.set(None);
                                    phantom_wire.set(None);
                                }
                            }
                        }
                    }
                }
            }

            if let Some(node) = selected_node {
                aside { class: "w-96 bg-slate-900 border-l border-slate-800 p-6 flex flex-col z-30 shadow-2xl settings-panel",
                    h3 { class: "text-sm font-bold mb-6 text-white", "Node Settings" }
                    NodeConfigEditor {
                        node: node.clone(),
                        on_change: move |new_config| {
                            let mut wf_write = workflow.write();
                            if let Some(n) = wf_write.nodes.iter_mut().find(|n| n.id == node.id) {
                                n.config = new_config;
                            }
                        }
                    }
                    button { class: "mt-auto w-full py-3 bg-indigo-600 rounded-xl font-bold text-white", onclick: move |_| { workflow.write().deselect_all(); }, "Close" }
                }
            }

            // Toasts
            div { class: "fixed bottom-6 right-6 flex flex-col gap-2 z-[200]",
                for (id, msg, kind) in toasts.read().iter() {
                    {
                        let class = if kind == "success" { "bg-emerald-900/90 border-emerald-500 text-emerald-100" } 
                                    else { "bg-slate-900/90 border-slate-700 text-slate-100" };
                        rsx! {
                            div {
                                key: "{id}",
                                class: "px-4 py-3 rounded-xl shadow-2xl border flex items-center gap-3 animate-slide-in {class}",
                                span { class: "text-sm font-medium", "{msg}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    launch(App);
}
