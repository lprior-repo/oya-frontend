use crate::ui::workflow_nodes::schema::{ServiceCallConfig, TargetType};
use crate::ui::workflow_nodes::shared::{
    FormField, FormHint, NodeCard, input_classes, textarea_classes, json_to_display,
    parse_json_draft, CARD_CLASSES, LABEL_CLASSES,
};
use dioxus::prelude::*;

const FOCUS_RING: &str = "green";

#[component]
pub fn ServiceCallForm(config: Signal<ServiceCallConfig>) -> Element {
    let input_cls = input_classes(FOCUS_RING);
    let textarea_cls = textarea_classes(FOCUS_RING);

    let pretty_input = json_to_display(&config.read().input);
    let json_draft = use_signal(|| pretty_input.clone());
    let json_error = use_signal(|| Option::<String>::None);
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

    let condition_value = match config.read().condition.clone() {
        Some(value) => value,
        None => String::new(),
    };

    rsx! {
        div {
            class: "space-y-4",

            div {
                class: "bg-green-50 border border-green-200 rounded-lg p-3 mb-4",
                p {
                    class: "text-sm text-green-800",
                    "🔗 ",
                    strong { "Action" },
                    " - Call another service and wait for the response."
                }
            }

            FormField {
                label: "What type of thing are you calling?",
                select {
                    class: "{input_cls}",
                    value: match &*config.read() {
                        ServiceCallConfig { target_type: TargetType::Service, .. } => "Service",
                        ServiceCallConfig { target_type: TargetType::VirtualObject, .. } => "Virtual Object",
                        ServiceCallConfig { target_type: TargetType::Workflow, .. } => "Workflow",
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
                    option { value: "Service", "Service (stateless)" }
                    option { value: "Virtual Object", "Virtual Object (stateful)" }
                    option { value: "Workflow", "Workflow (long-running)" }
                }
            }

            FormField {
                label: "Service Name",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., payment_service",
                    value: "{config.service_name}",
                    oninput: move |e| {
                        config.write().service_name = e.value().clone();
                    }
                }
            }

            FormField {
                label: "Object/Workflow Key",
                div {
                    class: if matches!(config.read().target_type, TargetType::VirtualObject | TargetType::Workflow) { "" } else { "hidden" },
                    input {
                        r#type: "text",
                        class: "{input_cls}",
                        placeholder: "e.g., order-123",
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
                    FormHint { text: "Which specific object/workflow to call" }
                }
            }

            FormField {
                label: "What to do",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., charge_card",
                    value: "{config.handler_name}",
                    oninput: move |e| {
                        config.write().handler_name = e.value().clone();
                    }
                }
            }

            FormField {
                label: "What to send (JSON)",
                textarea {
                    class: "{textarea_cls}",
                    rows: 4,
                    placeholder: r#"{"amount": 100, "currency": "USD"}"#,
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

            FormField {
                label: "Only run if...",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., {{ steps.validate.valid }} == true",
                    value: "{condition_value}",
                    oninput: move |e| {
                        let value = e.value().clone();
                        config.write().condition = if value.trim().is_empty() {
                            None
                        } else {
                            Some(value)
                        };
                    }
                }
                FormHint { text: "Leave empty to always run this step" }
            }
        }
    }
}

#[component]
pub fn ServiceCallNodeCard() -> Element {
    rsx! {
        NodeCard {
            icon_bg: "bg-green-100",
            icon: "🔗",
            title: "Call Service",
            subtitle: "Call another service and wait for result",
        }
    }
}
