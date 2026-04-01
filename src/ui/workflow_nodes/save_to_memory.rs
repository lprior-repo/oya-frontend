use crate::ui::workflow_nodes::schema::{MemoryKey, SaveToMemoryConfig};
use crate::ui::workflow_nodes::shared::{
    FormField, FormHint, NodeCard, input_classes, textarea_classes, json_to_display,
    parse_json_draft, CARD_CLASSES, LABEL_CLASSES,
};
use dioxus::prelude::*;

const FOCUS_RING: &str = "indigo";

#[component]
pub fn SaveToMemoryForm(config: ReadOnlySignal<SaveToMemoryConfig>) -> Element {
    let mut write_config = config.writer();
    let input_cls = input_classes(FOCUS_RING);
    let textarea_cls = textarea_classes(FOCUS_RING);

    let initial_draft = json_to_display(&config.read().value);
    let draft = use_signal(move || initial_draft);
    let parse_error = use_signal(|| None::<String>);

    rsx! {
        div {
            class: "space-y-4",
            div {
                class: "bg-indigo-50 border border-indigo-200 rounded-lg p-3 mb-4",
                p {
                    class: "text-sm text-indigo-800",
                    "💾 ",
                    strong { "Save Data" },
                    " - Store a value you can use later."
                }
            }
            FormField {
                label: "What name should this have?",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., order_total, user_email, approval_status",
                    value: "{config.read().key.as_str()}",
                    oninput: move |e| write_config.write().key = MemoryKey::new(e.value()),
                }
                FormHint { text: "Use this name to load the data later" }
            }
            FormField {
                label: "What to save (JSON)",
                textarea {
                    class: "{textarea_cls}",
                    rows: 4,
                    placeholder: r#"{"amount": 100, "currency": "USD"}"#,
                    value: "{draft}",
                    oninput: move |e| {
                        let next_value = e.value().clone();
                        draft.set(next_value.clone());
                        match parse_json_draft(next_value.as_str()) {
                            Ok(value) => {
                                parse_error.set(None);
                                write_config.write().value = value;
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
                p { class: "text-sm text-gray-600", "💡 This is like saving a variable. The data persists until you overwrite it or the workflow ends." }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::workflow_nodes::shared::parse_json_draft;

    #[test]
    fn parse_json_draft_accepts_valid_json() {
        assert!(parse_json_draft(r#"{"ok":true}"#).is_ok());
    }

    #[test]
    fn parse_json_draft_rejects_invalid_json() {
        assert!(parse_json_draft("{not-json}").is_err());
    }
}

#[component]
pub fn SaveToMemoryNodeCard() -> Element {
    rsx! {
        NodeCard {
            icon_bg: "bg-indigo-100",
            icon: "💾",
            title: "Save to Memory",
            subtitle: "Store a value for later",
        }
    }
}
