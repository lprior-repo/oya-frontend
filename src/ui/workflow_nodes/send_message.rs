use crate::ui::workflow_nodes::schema::{DelayedMessageConfig, SendMessageConfig, TargetType};
use dioxus::prelude::*;

#[component]
pub fn SendMessageForm(config: Signal<SendMessageConfig>) -> Element {
    rsx! {
        div {
            class: "space-y-4",

            div {
                class: "bg-yellow-50 border border-yellow-200 rounded-lg p-3 mb-4",
                p {
                    class: "text-sm text-yellow-800",
                    "📤 ",
                    strong { "Fire & Forget" },
                    " - Send a message and continue without waiting."
                }
            }

            div {
                class: "form-field",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "Send to"
                }
                select {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-yellow-500",
                    value: match &*config.read() {
                        SendMessageConfig { target_type: TargetType::Service, .. } => "Service",
                        SendMessageConfig { target_type: TargetType::VirtualObject, .. } => "Virtual Object",
                        SendMessageConfig { target_type: TargetType::Workflow, .. } => "Workflow",
                    },
                    onchange: move |e| {
                        config.write().target_type = match e.value().as_str() {
                            "Service" => TargetType::Service,
                            "Virtual Object" => TargetType::VirtualObject,
                            "Workflow" => TargetType::Workflow,
                            _ => TargetType::Service,
                        };
                    },
                    option { value: "Service", "Service" }
                    option { value: "Virtual Object", "Virtual Object" }
                    option { value: "Workflow", "Workflow" }
                }
            }

            div {
                class: "form-field",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "Service Name"
                }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-yellow-500",
                    placeholder: "e.g., notification_service",
                    value: "{config.read().service_name}",
                    oninput: move |e| {
                        config.write().service_name = e.value().clone();
                    }
                }
            }

            div {
                class: "form-field",
                visible: matches!(config.read().target_type, TargetType::VirtualObject | TargetType::Workflow),
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "Key (for objects/workflows)"
                }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-yellow-500",
                    placeholder: "e.g., user-456",
                    value: "{config.read().key.as_deref().unwrap_or(\"\")}",
                    oninput: move |e| {
                        config.write().key = Some(e.value().clone());
                    }
                }
            }

            div {
                class: "form-field",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "Handler"
                }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-yellow-500",
                    placeholder: "e.g., send_email",
                    value: "{config.read().handler_name}",
                    oninput: move |e| {
                        config.write().handler_name = e.value().clone();
                    }
                }
            }

            div {
                class: "form-field",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "Message (JSON)"
                }
                textarea {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-yellow-500 font-mono text-sm",
                    rows: 3,
                    oninput: move |e| {
                        if let Ok(v) = serde_json::from_str(&e.value()) {
                            config.write().input = v;
                        }
                    },
                    "{serde_json::to_string_pretty(&*config.read().input).unwrap_or_default()}"
                }
            }

            div {
                class: "bg-gray-50 p-3 rounded-lg",
                p {
                    class: "text-sm text-gray-600",
                    "💡 Use this when you don't need to wait for a response, like sending notifications."
                }
            }
        }
    }
}

#[component]
pub fn SendMessageNodeCard() -> Element {
    rsx! {
        div {
            class: "flex items-center gap-3 p-3 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow",

            div {
                class: "w-10 h-10 bg-yellow-100 rounded-full flex items-center justify-center",
                span {
                    class: "text-xl",
                    "📤"
                }
            },

            div {
                class: "flex-1",
                h3 {
                    class: "font-medium text-gray-900",
                    "Send Message"
                }
                p {
                    class: "text-sm text-gray-500",
                    "Send without waiting for response"
                }
            }
        }
    }
}

#[component]
pub fn DelayedMessageForm(config: Signal<DelayedMessageConfig>) -> Element {
    rsx! {
        div {
            class: "space-y-4",

            div {
                class: "bg-orange-50 border border-orange-200 rounded-lg p-3 mb-4",
                p {
                    class: "text-sm text-orange-800",
                    "⏰ ",
                    strong { "Schedule for Later" },
                    " - Send a message at a specific time."
                }
            }

            div {
                class: "form-field",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "Send to"
                }
                select {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-500",
                    value: match &*config.read() {
                        DelayedMessageConfig { target_type: TargetType::Service, .. } => "Service",
                        DelayedMessageConfig { target_type: TargetType::VirtualObject, .. } => "Virtual Object",
                        DelayedMessageConfig { target_type: TargetType::Workflow, .. } => "Workflow",
                    },
                    onchange: move |e| {
                        config.write().target_type = match e.value().as_str() {
                            "Service" => TargetType::Service,
                            "Virtual Object" => TargetType::VirtualObject,
                            "Workflow" => TargetType::Workflow,
                            _ => TargetType::Service,
                        };
                    },
                    option { value: "Service", "Service" }
                    option { value: "Virtual Object", "Virtual Object" }
                    option { value: "Workflow", "Workflow" }
                }
            }

            div {
                class: "form-field",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "Service Name"
                }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-500",
                    placeholder: "e.g., reminder_service",
                    value: "{config.read().service_name}",
                    oninput: move |e| {
                        config.write().service_name = e.value().clone();
                    }
                }
            }

            div {
                class: "form-field",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "How long to wait?"
                }
                div {
                    class: "grid grid-cols-4 gap-2",
                    button {
                        class: "px-2 py-2 text-sm border rounded-md hover:bg-gray-50",
                        onclick: move |_| {
                            config.write().delay_ms = 60_000;
                        },
                        "1 min"
                    }
                    button {
                        class: "px-2 py-2 text-sm border rounded-md hover:bg-gray-50",
                        onclick: move |_| {
                            config.write().delay_ms = 3600_000;
                        },
                        "1 hour"
                    }
                    button {
                        class: "px-2 py-2 text-sm border rounded-md hover:bg-gray-50",
                        onclick: move |_| {
                            config.write().delay_ms = 86400_000;
                        },
                        "1 day"
                    }
                    button {
                        class: "px-2 py-2 text-sm border rounded-md hover:bg-gray-50",
                        onclick: move |_| {
                            config.write().delay_ms = 604800_000;
                        },
                        "1 week"
                    }
                }
                input {
                    r#type: "number",
                    class: "w-full mt-2 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-500",
                    placeholder: "Or enter milliseconds",
                    value: "{config.read().delay_ms}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u64>() {
                            config.write().delay_ms = v;
                        }
                    }
                }
            }

            div {
                class: "form-field",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    "Message (JSON)"
                }
                textarea {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-500 font-mono text-sm",
                    rows: 3,
                    oninput: move |e| {
                        if let Ok(v) = serde_json::from_str(&e.value()) {
                            config.write().input = v;
                        }
                    },
                    "{serde_json::to_string_pretty(&*config.read().input).unwrap_or_default()}"
                }
            }
        }
    }
}

#[component]
pub fn DelayedMessageNodeCard() -> Element {
    rsx! {
        div {
            class: "flex items-center gap-3 p-3 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow",

            div {
                class: "w-10 h-10 bg-orange-100 rounded-full flex items-center justify-center",
                span {
                    class: "text-xl",
                    "⏰"
                }
            },

            div {
                class: "flex-1",
                h3 {
                    class: "font-medium text-gray-900",
                    "Delayed Message"
                }
                p {
                    class: "text-sm text-gray-500",
                    "Send after a delay"
                }
            }
        }
    }
}
