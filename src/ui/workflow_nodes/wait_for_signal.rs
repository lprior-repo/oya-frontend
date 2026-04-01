use crate::ui::workflow_nodes::schema::WaitForSignalConfig;
use crate::ui::workflow_nodes::shared::{FormField, FormHint, NodeCard, input_classes, CARD_CLASSES, LABEL_CLASSES, PRESET_BTN_CLASSES};
use dioxus::prelude::*;

const FOCUS_RING: &str = "rose";

#[component]
pub fn WaitForSignalForm(config: Signal<WaitForSignalConfig>) -> Element {
    let input_cls = input_classes(FOCUS_RING);

    rsx! {
        div {
            class: "space-y-4",

            div {
                class: "bg-rose-50 border border-rose-200 rounded-lg p-3 mb-4",
                p {
                    class: "text-sm text-rose-800",
                    "📡 ",
                    strong { "Wait for Signal" },
                    " - Pause until another workflow sends a signal."
                }
            }

            FormField {
                label: "Promise Name",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., payment_complete, order_shipped",
                    value: "{config.promise_name}",
                    oninput: move |e| {
                        config.write().promise_name = e.value().clone();
                    }
                }
                FormHint { text: "Another workflow resolves this promise by name" }
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
                    "💡 Use a 'Resolve Signal' step in another workflow to continue this one."
                }
            }
        }
    }
}

#[component]
pub fn WaitForSignalNodeCard() -> Element {
    rsx! {
        NodeCard {
            icon_bg: "bg-rose-100",
            icon: "📡",
            title: "Wait for Signal",
            subtitle: "Wait for another workflow",
        }
    }
}
