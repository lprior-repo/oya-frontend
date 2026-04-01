use crate::ui::workflow_nodes::schema::{CronExpression, ScheduleTriggerConfig};
use dioxus::prelude::*;

const CARD_CLASSES: &str = "flex items-center gap-3 p-3 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow";
const INPUT_CLASSES: &str = "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500";
const LABEL_CLASSES: &str = "block text-sm font-medium text-gray-700 mb-1";
const PRESET_BTN_CLASSES: &str = "px-3 py-2 text-sm border rounded-md hover:bg-gray-50";

#[component]
pub fn ScheduleTriggerForm(config: ReadOnlySignal<ScheduleTriggerConfig>) -> Element {
    let mut write_config = config.writer();

    rsx! {
        div {
            class: "space-y-4",

            div {
                class: "bg-blue-50 border border-blue-200 rounded-lg p-3 mb-4",
                p {
                    class: "text-sm text-blue-800",
                    "🕐 ",
                    strong { "Starts here" },
                    " - This workflow runs on a schedule."
                }
            }

            div {
                class: "form-field",
                label {
                    class: "{LABEL_CLASSES}",
                    "When should this run?"
                }
                div {
                    class: "grid grid-cols-3 gap-2 mb-2",
                    role: "group",
                    aria_label: "Schedule presets",
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            write_config.write().schedule = CronExpression::new("0 * * * *");
                        },
                        "Every hour"
                    }
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            write_config.write().schedule = CronExpression::new("0 0 * * *");
                        },
                        "Daily"
                    }
                    button {
                        class: "{PRESET_BTN_CLASSES}",
                        onclick: move |_| {
                            write_config.write().schedule = CronExpression::new("0 0 * * 0");
                        },
                        "Weekly"
                    }
                }
                input {
                    r#type: "text",
                    class: "{INPUT_CLASSES}",
                    placeholder: "0 * * * *",
                    value: "{config.read().schedule.as_str()}",
                    oninput: move |e| {
                        write_config.write().schedule = CronExpression::new(e.value());
                    }
                }
                p {
                    class: "text-xs text-gray-500 mt-1",
                    "Cron expression: minute hour day month weekday"
                }
            }
        }
    }
}

#[component]
pub fn ScheduleTriggerNodeCard() -> Element {
    rsx! {
        div {
            class: "{CARD_CLASSES}",

            div {
                class: "w-10 h-10 bg-purple-100 rounded-full flex items-center justify-center",
                span {
                    class: "text-xl",
                    "🕐"
                }
            }

            div {
                class: "flex-1",
                h3 {
                    class: "font-medium text-gray-900",
                    "Schedule"
                }
                p {
                    class: "text-sm text-gray-500",
                    "Runs automatically on a schedule"
                }
            }
        }
    }
}
