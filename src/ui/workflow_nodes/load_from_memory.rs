use crate::ui::workflow_nodes::schema::{LoadFromMemoryConfig, MemoryKey};
use crate::ui::workflow_nodes::shared::{
    FormField, FormHint, NodeCard, input_classes, textarea_classes,
    optional_json_to_display, parse_optional_json_draft, CARD_CLASSES, LABEL_CLASSES,
};
use dioxus::prelude::*;

const FOCUS_RING: &str = "cyan";

#[component]
pub fn LoadFromMemoryForm(config: ReadOnlySignal<LoadFromMemoryConfig>) -> Element {
    let mut write_config = config.writer();
    let input_cls = input_classes(FOCUS_RING);
    let textarea_cls = textarea_classes(FOCUS_RING);

    let initial_draft = optional_json_to_display(config.read().default.as_ref());
    let draft = use_signal(move || initial_draft);
    let parse_error = use_signal(|| None::<String>);

    rsx! {
        div {
            class: "space-y-4",
            div {
                class: "bg-cyan-50 border border-cyan-200 rounded-lg p-3 mb-4",
                p {
                    class: "text-sm text-cyan-800",
                    "📂 ",
                    strong { "Load Data" },
                    " - Get a value you saved earlier."
                }
            }
            FormField {
                label: "Which saved item?",
                input {
                    r#type: "text",
                    class: "{input_cls}",
                    placeholder: "e.g., order_total, user_email",
                    value: "{config.read().key.as_str()}",
                    oninput: move |e| write_config.write().key = MemoryKey::new(e.value()),
                }
                FormHint { text: "Name you used when saving" }
            }
            FormField {
                label: "Default value (if not found)",
                textarea {
                    class: "{textarea_cls}",
                    rows: 2,
                    placeholder: "null",
                    value: "{draft}",
                    oninput: move |e| {
                        let next_value = e.value().clone();
                        draft.set(next_value.clone());
                        match parse_optional_json_draft(next_value.as_str()) {
                            Ok(next_default) => {
                                parse_error.set(None);
                                write_config.write().default = next_default;
                            }
                            Err(error_text) => parse_error.set(Some(error_text)),
                        }
                    }
                }
                if let Some(error_text) = parse_error.read().as_ref() {
                    p { class: "text-xs text-red-600 mt-1", "{error_text}" }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::workflow_nodes::shared::parse_optional_json_draft;

    #[test]
    fn parse_optional_json_draft_empty_is_none() {
        assert_eq!(parse_optional_json_draft("   "), Ok(None));
    }

    #[test]
    fn parse_optional_json_draft_valid_is_some() {
        assert!(matches!(parse_optional_json_draft(r#"{"fallback":1}"#), Ok(Some(_))));
    }

    #[test]
    fn parse_optional_json_draft_invalid_is_err() {
        assert!(parse_optional_json_draft("{not-json}").is_err());
    }
}

#[component]
pub fn LoadFromMemoryNodeCard() -> Element {
    rsx! {
        NodeCard {
            icon_bg: "bg-cyan-100",
            icon: "📂",
            title: "Load from Memory",
            subtitle: "Get a saved value",
        }
    }
}
