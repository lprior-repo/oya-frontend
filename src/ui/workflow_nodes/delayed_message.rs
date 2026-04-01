use crate::ui::workflow_nodes::schema::{DelayedMessageConfig, TargetType};
use crate::ui::workflow_nodes::shared::{
    FormField, FormHint, NodeCard, input_classes, textarea_classes, json_to_display,
    parse_json_draft, CARD_CLASSES, LABEL_CLASSES, PRESET_BTN_CLASSES,
};
use dioxus::prelude::*;

const FOCUS_RING: &str = "orange";

#[component]
pub fn DelayedMessageForm(config: Signal<DelayedMessageConfig>) -> Element {
    let input_cls = input_classes(FOCUS_RING);
    let textarea_cls = textarea_classes(FOCUS_RING);

    let pretty_input = json_to_display(&config.read().input);
    let json_draft = use_signal(|| pretty_input.clone());
    let json_error = use_signal(|| Option::<String>::None);
    let delay_error = use_signal(|| Option::<String>::None);
    let last_synced_input = use_signal(|| pretty_input.clone());

    use_effect(move || {
        let latest = json_to_display(&config.read().input);
        let synced = last_synced_input.read().clone();
        if latest != synced {
            last_synced_input.set(latest.clone());
            json_draft.set(latest);
            json_error.set(None);
        }
    });

    let key_value = match config.read().key.clone() {
        Some(value) => value,
        None => String::new(),
    };

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

            FormField {
                label: "Send to",
                select {
                    class: "{input_cls}",
                    value: match config.read().target_type {
                        TargetType::Service => "Service",
                        TargetType::VirtualObject => "Virtual Object",
                        TargetType::Workflow => "Workflow",
                    },
                    onchange: move |e| {
                        let next_target_type = match e.value().as_str() {
                            "Service" => TargetType::Service,
                            "Virtual Object" => TargetType::VirtualObject,
                            "Workflow" => TargetType::Workflow,
                            _ => TargetType::Service,
                        };

                        let mut cfg = config.write();
                        cfg.target_type = next_target_type;
                        if matches!(next_target_type, TargetType::Service) {
                            cfg.key = None;
                        }
                    },
                    option { value: "Service", "Service" }
                    option { value: "Virtual Object", "Virtual Object" }
                    option { value: "Workflow", "Workflow" }
                }
            }

            FormField {
                label: "Service Name",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., reminder_service",
                    value: "{config.read().service_name}",
                    oninput: move |e| {
                        config.write().service_name = e.value().clone();
                    }
                }
            }

            FormField {
                label: "Key (for objects/workflows)",
                div {
                    class: if matches!(config.read().target_type, TargetType::VirtualObject | TargetType::Workflow) { "" } else { "hidden" },
                    input {
                        r#type: "text",
                        class: "{input_cls}",
                        placeholder: "e.g., user-456",
                        value: "{key_value}",
                        oninput: move |e| {
                            let value = e.value().clone();
                            config.write().key = if value.trim().is_empty() {
                                None
                            } else {
                                Some(value)
                            };
                        }
                    }
                }
            }

            FormField {
                label: "Handler",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., send_reminder",
                    value: "{config.read().handler_name}",
                    oninput: move |e| {
                        config.write().handler_name = e.value().clone();
                    }
                }
            }

            FormField {
                label: "How long to wait?",
                div {
                    class: "grid grid-cols-4 gap-2 mb-2",
                    role: "group",
                    aria_label: "Delay presets",
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            config.write().delay_ms = 60_000;
                            delay_error.set(None);
                        },
                        "1 min"
                    }
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            config.write().delay_ms = 3600_000;
                            delay_error.set(None);
                        },
                        "1 hour"
                    }
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            config.write().delay_ms = 86400_000;
                            delay_error.set(None);
                        },
                        "1 day"
                    }
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            config.write().delay_ms = 604800_000;
                            delay_error.set(None);
                        },
                        "1 week"
                    }
                }
                input {
                    r#type: "number",
                    min: "1",
                    class: "{input_cls}",
                    placeholder: "Or enter milliseconds",
                    value: "{config.delay_ms}",
                    oninput: move |e| {
                        let value = e.value();
                        if value.trim().is_empty() {
                            delay_error.set(None);
                        } else if let Ok(parsed) = value.parse::<u64>() {
                            if parsed > 0 {
                                config.write().delay_ms = parsed;
                                delay_error.set(None);
                            } else {
                                delay_error
                                    .set(Some("Delay must be greater than 0 ms".to_string()));
                            }
                        }
                    }
                }
                if let Some(error) = delay_error() {
                    p {
                        class: "text-xs text-red-600 mt-1",
                        "{error}"
                    }
                } else {
                    FormHint { text: "How long to wait before sending (must be greater than 0 ms)" }
                }
            }

            FormField {
                label: "Message (JSON)",
                textarea {
                    class: "{textarea_cls}",
                    rows: 3,
                    value: "{json_draft}",
                    oninput: move |e| {
                        let draft = e.value().clone();
                        json_draft.set(draft.clone());

                        match parse_json_draft(&draft) {
                            Ok(value) => {
                                config.write().input = value;
                                last_synced_input.set(draft);
                                json_error.set(None);
                            }
                            Err(error) => {
                                json_error.set(Some(error));
                            }
                        }
                    },
                }
                if let Some(error) = json_error() {
                    p {
                        class: "text-xs text-red-600 mt-1",
                        "{error}"
                    }
                } else {
                    FormHint { text: "Enter valid JSON to update this field" }
                }
            }
        }
    }
}

#[component]
pub fn DelayedMessageNodeCard() -> Element {
    rsx! {
        NodeCard {
            icon_bg: "bg-orange-100",
            icon: "⏰",
            title: "Delayed Message",
            subtitle: "Send after a delay",
        }
    }
}
