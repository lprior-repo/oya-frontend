use crate::ui::workflow_nodes::schema::{SendMessageConfig, TargetType};
use crate::ui::workflow_nodes::shared::{
    FormField, FormHint, NodeCard, input_classes, textarea_classes, json_to_display,
    parse_json_draft, CARD_CLASSES, LABEL_CLASSES,
};
use dioxus::prelude::*;

const FOCUS_RING: &str = "yellow";

fn normalize_optional_key(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn target_type_from_input(input: &str) -> TargetType {
    match input {
        "Service" => TargetType::Service,
        "Virtual Object" => TargetType::VirtualObject,
        "Workflow" => TargetType::Workflow,
        _ => TargetType::Service,
    }
}

#[derive(Clone)]
pub struct SendMessageNode {
    pub config: Signal<SendMessageConfig>,
}

impl SendMessageNode {
    pub fn new() -> Self {
        Self {
            config: use_signal(|| SendMessageConfig {
                target_type: TargetType::Service,
                service_name: String::new(),
                key: None,
                handler_name: String::new(),
                input: serde_json::Value::Null,
            }),
        }
    }

    pub fn from_config(config: SendMessageConfig) -> Self {
        Self {
            config: use_signal(|| config),
        }
    }
}

impl Default for SendMessageNode {
    fn default() -> Self {
        Self::new()
    }
}

#[component]
pub fn SendMessageForm(config: Signal<SendMessageConfig>) -> Element {
    let input_cls = input_classes(FOCUS_RING);
    let textarea_cls = textarea_classes(FOCUS_RING);

    let initial_draft = json_to_display(&config.read().input);
    let draft = use_signal(move || initial_draft);
    let parse_error = use_signal(|| None::<String>);

    let key_value = match config.read().key.clone() {
        Some(value) => value,
        None => String::new(),
    };

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
            FormField {
                label: "Send to",
                select {
                    class: "{input_cls}",
                    value: match &*config.read() {
                        SendMessageConfig { target_type: TargetType::Service, .. } => "Service",
                        SendMessageConfig { target_type: TargetType::VirtualObject, .. } => "Virtual Object",
                        SendMessageConfig { target_type: TargetType::Workflow, .. } => "Workflow",
                    },
                    onchange: move |e| {
                        let next_target_type = target_type_from_input(e.value().as_str());
                        let should_clear_key = matches!(next_target_type, TargetType::Service);
                        config.write().target_type = next_target_type;
                        if should_clear_key {
                            config.write().key = None;
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
                    placeholder: "e.g., notification_service",
                    value: "{config.service_name}",
                    oninput: move |e| config.write().service_name = e.value().clone(),
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
                        oninput: move |e| config.write().key = normalize_optional_key(e.value().as_str()),
                    }
                }
            }
            FormField {
                label: "Handler",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., send_email",
                    value: "{config.handler_name}",
                    oninput: move |e| config.write().handler_name = e.value().clone(),
                }
            }
            FormField {
                label: "Message (JSON)",
                textarea {
                    class: "{textarea_cls}",
                    rows: 3,
                    value: "{draft}",
                    oninput: move |e| {
                        let next_value = e.value().clone();
                        draft.set(next_value.clone());
                        match parse_json_draft(next_value.as_str()) {
                            Ok(value) => {
                                parse_error.set(None);
                                config.write().input = value;
                            }
                            Err(error_text) => parse_error.set(Some(error_text)),
                        }
                    },
                }
                if let Some(error_text) = parse_error.read().as_ref() {
                    p { class: "text-xs text-red-600 mt-1", "{error_text}" }
                }
            }
            div {
                class: "bg-gray-50 p-3 rounded-lg",
                p { class: "text-sm text-gray-600", "💡 Use this when you don't need to wait for a response, like sending notifications." }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_optional_key;
    use crate::ui::workflow_nodes::shared::parse_json_draft;

    #[test]
    fn normalize_optional_key_none_for_blank() {
        assert_eq!(normalize_optional_key("   "), None);
    }

    #[test]
    fn normalize_optional_key_trims_whitespace() {
        assert_eq!(normalize_optional_key("  order-1  "), Some("order-1".to_string()));
    }

    #[test]
    fn parse_json_draft_rejects_invalid_json() {
        assert!(parse_json_draft("{not-json}").is_err());
    }
}

#[component]
pub fn SendMessageNodeCard() -> Element {
    rsx! {
        NodeCard {
            icon_bg: "bg-yellow-100",
            icon: "📤",
            title: "Send Message",
            subtitle: "Send without waiting for response",
        }
    }
}
