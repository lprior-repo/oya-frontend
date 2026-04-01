use dioxus::prelude::*;

/// Shared CSS class constants used across all workflow node forms and cards.
pub const CARD_CLASSES: &str =
    "flex items-center gap-3 p-3 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow";
pub const LABEL_CLASSES: &str = "block text-sm font-medium text-gray-700 mb-1";
pub const PRESET_BTN_CLASSES: &str = "px-3 py-2 text-sm border rounded-md hover:bg-gray-50";

/// Returns the standard input classes with a per-node focus ring color.
///
/// Every workflow node form uses the same base input styling but with a
/// different Tailwind `focus:ring-{color}-500`. This function avoids
/// duplicating the long base string in each module.
pub fn input_classes(focus_ring_color: &str) -> String {
    format!(
        "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-{focus_ring_color}-500"
    )
}

/// Returns the standard textarea classes with a per-node focus ring color
/// and monospace font styling for JSON/code editing.
pub fn textarea_classes(focus_ring_color: &str) -> String {
    format!(
        "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-{focus_ring_color}-500 font-mono text-sm"
    )
}

// ---------------------------------------------------------------------------
// JSON helpers (duplicated across load_from_memory, save_to_memory,
// send_message, service_call, delayed_message)
// ---------------------------------------------------------------------------

/// Pretty-print a `serde_json::Value` for display. Returns an empty string
/// on serialization failure.
pub fn json_to_display(value: &serde_json::Value) -> String {
    serde_json::to_string_pretty(value).map_or_else(|_| String::new(), |v| v)
}

/// Pretty-print an optional `serde_json::Value`. Returns an empty string
/// when the value is `None` or serialization fails.
pub fn optional_json_to_display(value: Option<&serde_json::Value>) -> String {
    value.map_or_else(String::new, json_to_display)
}

/// Parse a string as JSON. Used for live textarea draft validation.
pub fn parse_json_draft(input: &str) -> Result<serde_json::Value, String> {
    serde_json::from_str(input).map_err(|error| format!("Invalid JSON: {error}"))
}

/// Parse a string as optional JSON: empty/whitespace input maps to `Ok(None)`.
pub fn parse_optional_json_draft(input: &str) -> Result<Option<serde_json::Value>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        serde_json::from_str(trimmed)
            .map(Some)
            .map_err(|error| format!("Invalid JSON: {error}"))
    }
}

// ---------------------------------------------------------------------------
// NodeCard component
// ---------------------------------------------------------------------------

/// Props for the shared `NodeCard` component.
///
/// Each workflow node type renders the same card layout: a colored circle
/// containing an emoji icon, a bold title, and a muted subtitle. The only
/// variation is the color, icon, title, and subtitle text.
#[derive(Props, Clone, PartialEq)]
pub struct NodeCardProps {
    pub icon_bg: &'static str,
    pub icon: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
}

/// A reusable card component for workflow node type selectors.
///
/// All 11 node card components (`HttpTriggerNodeCard`, `ServiceCallNodeCard`,
/// etc.) previously duplicated the same `rsx!` structure with only four
/// varying fields. This component consolidates that pattern.
#[component]
pub fn NodeCard(props: NodeCardProps) -> Element {
    rsx! {
        div {
            class: "{CARD_CLASSES}",

            div {
                class: "w-10 h-10 {props.icon_bg} rounded-full flex items-center justify-center",
                span {
                    class: "text-xl",
                    "{props.icon}"
                }
            },

            div {
                class: "flex-1",
                h3 {
                    class: "font-medium text-gray-900",
                    "{props.title}"
                }
                p {
                    class: "text-sm text-gray-500",
                    "{props.subtitle}"
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Form field helper components
// ---------------------------------------------------------------------------

/// Props for a labelled form field wrapper.
#[derive(Props, Clone, PartialEq)]
pub struct FormFieldProps {
    pub label: &'static str,
    pub children: Element,
}

/// A labelled form field with the standard label styling.
///
/// Wraps any input/select/textarea with a consistent `<label>` element
/// using `LABEL_CLASSES`.
#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    rsx! {
        div {
            class: "form-field",
            label { class: "{LABEL_CLASSES}", "{props.label}" }
            {props.children}
        }
    }
}

/// Props for the form description hint pattern.
#[derive(Props, Clone, PartialEq)]
pub struct FormHintProps {
    pub text: &'static str,
}

/// A muted hint line below a form field.
#[component]
pub fn FormHint(props: FormHintProps) -> Element {
    rsx! {
        p { class: "text-xs text-gray-500 mt-1", "{props.text}" }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_classes_includes_focus_ring_color() {
        let classes = input_classes("blue");
        assert!(
            classes.contains("focus:ring-blue-500"),
            "expected blue focus ring, got: {classes}"
        );
    }

    #[test]
    fn textarea_classes_includes_focus_ring_color_and_mono() {
        let classes = textarea_classes("green");
        assert!(classes.contains("focus:ring-green-500"));
        assert!(classes.contains("font-mono"));
    }

    #[test]
    fn json_to_display_pretty_prints_object() {
        let value = serde_json::json!({"a": 1});
        let display = json_to_display(&value);
        assert!(display.contains('\n'));
    }

    #[test]
    fn optional_json_to_display_none_is_empty() {
        assert_eq!(optional_json_to_display(None), "");
    }

    #[test]
    fn optional_json_to_display_some_is_pretty() {
        let value = serde_json::json!(42);
        let display = optional_json_to_display(Some(&value));
        assert_eq!(display, "42");
    }

    #[test]
    fn parse_json_draft_valid_returns_ok() {
        assert!(parse_json_draft(r#"{"ok":true}"#).is_ok());
    }

    #[test]
    fn parse_json_draft_invalid_returns_err() {
        assert!(parse_json_draft("{not-json}").is_err());
    }

    #[test]
    fn parse_optional_json_draft_empty_is_none() {
        assert_eq!(parse_optional_json_draft("   "), Ok(None));
    }

    #[test]
    fn parse_optional_json_draft_valid_is_some() {
        assert!(matches!(
            parse_optional_json_draft(r#"{"fallback":1}"#),
            Ok(Some(_))
        ));
    }

    #[test]
    fn parse_optional_json_draft_invalid_is_err() {
        assert!(parse_optional_json_draft("{not-json}").is_err());
    }
}
