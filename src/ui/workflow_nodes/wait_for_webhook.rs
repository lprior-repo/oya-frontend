use crate::ui::workflow_nodes::schema::WaitForWebhookConfig;
use crate::ui::workflow_nodes::shared::{FormField, FormHint, NodeCard, input_classes, CARD_CLASSES, LABEL_CLASSES, PRESET_BTN_CLASSES};
use dioxus::prelude::*;

const FOCUS_RING: &str = "teal";

#[component]
pub fn WaitForWebhookForm(config: Signal<WaitForWebhookConfig>) -> Element {
    let input_cls = input_classes(FOCUS_RING);

    rsx! {
        div {
            class: "space-y-4",

            div {
                class: "bg-teal-50 border border-teal-200 rounded-lg p-3 mb-4",
                p {
                    class: "text-sm text-teal-800",
                    "🪝 ",
                    strong { "Wait for Webhook" },
                    " - Pause until an external service calls back."
                }
            }

            div {
                class: "bg-blue-50 border border-blue-200 rounded-lg p-3 mb-4",
                p {
                    class: "text-sm text-blue-800 font-mono",
                    "POST /restate/awakeables/{id}/resolve"
                }
            }

            FormField {
                label: "Awakeable ID",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., payment_callback, approval_response",
                    value: "{config.awakeable_id}",
                    oninput: move |e| {
                        config.write().awakeable_id = e.value().clone();
                    }
                }
                FormHint { text: "Use the same awakeable ID in your callback resolver" }
            }

            FormField {
                label: "Give up after...",
                div {
                    class: "grid grid-cols-3 gap-2",
                    role: "group",
                    aria_label: "Timeout presets",
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            config.write().timeout_ms = Some(60000);
                        },
                        "1 minute"
                    }
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            config.write().timeout_ms = Some(3600000);
                        },
                        "1 hour"
                    }
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            config.write().timeout_ms = None;
                        },
                        "No timeout"
                    }
                }
            }

            div {
                class: "bg-yellow-50 p-3 rounded-lg",
                p {
                    class: "text-sm text-yellow-800",
                    "💡 The workflow pauses here. External services call the webhook to continue."
                }
            }
        }
    }
}

#[component]
pub fn WaitForWebhookNodeCard() -> Element {
    rsx! {
        NodeCard {
            icon_bg: "bg-teal-100",
            icon: "🪝",
            title: "Wait for Webhook",
            subtitle: "Pause until external callback",
        }
    }
}
