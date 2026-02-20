use super::model::NODE_TEMPLATES;
use super::presentation::{category_dot, category_label, template_matches_query};

#[test]
fn given_entry_category_when_getting_dot_then_it_uses_entry_color() {
    let color = category_dot("entry");

    assert_eq!(color, "bg-blue-400");
}

#[test]
fn given_unknown_category_when_getting_label_then_it_falls_back_to_other() {
    let label = category_label("unknown");

    assert_eq!(label, "Other");
}

#[test]
fn given_schedule_query_when_matching_templates_then_schedule_node_is_found() {
    let schedule_template = NODE_TEMPLATES
        .iter()
        .find(|template| template.node_type == "cron-trigger")
        .expect("cron-trigger template should exist");

    let matches = template_matches_query(schedule_template, "schedule");

    assert!(matches);
}

#[test]
fn given_friendly_phrase_when_matching_templates_then_template_matches_behavior_text() {
    let template = NODE_TEMPLATES
        .iter()
        .find(|item| item.node_type == "send-message")
        .expect("send-message template should exist");

    let matches = template_matches_query(template, "without waiting");

    assert!(matches);
}

#[test]
fn given_non_matching_query_when_matching_templates_then_template_does_not_match() {
    let template = NODE_TEMPLATES
        .iter()
        .find(|item| item.node_type == "http-handler")
        .expect("http-handler template should exist");

    let matches = template_matches_query(template, "totally-unrelated-query");

    assert!(!matches);
}
