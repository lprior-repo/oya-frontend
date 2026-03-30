//! Configuration validation helpers.

use crate::graph::{workflow_node::WorkflowNode, Node};

// ===========================================================================
// Config Validation Helper
// ===========================================================================

pub fn workflow_node_from_persisted(node: &Node) -> Result<WorkflowNode, String> {
    let mut config_object = node.config.as_object().cloned().unwrap_or_default();
    let config_type = config_object
        .get("type")
        .and_then(serde_json::Value::as_str)
        .map(std::string::ToString::to_string)
        .unwrap_or_default();

    let resolved_type = if node.node_type.is_empty() {
        config_type
    } else {
        node.node_type.clone()
    };

    if resolved_type.is_empty() {
        return Err("<missing-node-type>".to_string());
    }

    config_object.insert(
        "type".to_string(),
        serde_json::Value::String(resolved_type.clone()),
    );

    serde_json::from_value::<WorkflowNode>(serde_json::Value::Object(config_object)).or_else(|_| {
        resolved_type
            .parse::<WorkflowNode>()
            .map_err(|_| resolved_type)
    })
}
