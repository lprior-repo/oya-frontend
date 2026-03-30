//! Structural validations for workflows.

use crate::graph::{NodeCategory, NodeId, ValidationIssue, Workflow};

// ===========================================================================
// Structural Validations (Reachability, Entry Points, Orphans)
// ===========================================================================

pub fn validate_entry_points(workflow: &Workflow, issues: &mut Vec<ValidationIssue>) {
    if !workflow
        .nodes
        .iter()
        .any(|n| n.category == NodeCategory::Entry)
    {
        issues.push(ValidationIssue::error(
            "Workflow has no entry point (e.g., HTTP Handler, Kafka Handler)".to_string(),
        ));
    }
}

pub fn validate_reachability(workflow: &Workflow, issues: &mut Vec<ValidationIssue>) {
    if workflow.nodes.is_empty() || workflow.connections.is_empty() {
        return;
    }

    let entry_ids: std::collections::HashSet<NodeId> = workflow
        .nodes
        .iter()
        .filter(|n| n.category == NodeCategory::Entry)
        .map(|n| n.id)
        .collect();

    if entry_ids.is_empty() {
        return;
    }

    let mut reachable = std::collections::HashSet::new();
    let mut stack: Vec<NodeId> = entry_ids.iter().copied().collect();

    while let Some(current) = stack.pop() {
        if reachable.insert(current) {
            for conn in workflow.connections.iter().filter(|c| c.source == current) {
                if !reachable.contains(&conn.target) {
                    stack.push(conn.target);
                }
            }
        }
    }

    for node in &workflow.nodes {
        if !reachable.contains(&node.id) && node.category != NodeCategory::Entry {
            issues.push(ValidationIssue::warning_for_node(
                format!("Node '{}' is not reachable from any entry point", node.name),
                node.id,
            ));
        }
    }
}

pub fn validate_orphan_nodes(workflow: &Workflow, issues: &mut Vec<ValidationIssue>) {
    for node in &workflow.nodes {
        if node.category == NodeCategory::Entry {
            continue;
        }

        let has_incoming = workflow.connections.iter().any(|c| c.target == node.id);
        let has_outgoing = workflow.connections.iter().any(|c| c.source == node.id);

        if !has_incoming && !has_outgoing && workflow.nodes.len() > 1 {
            issues.push(ValidationIssue::warning_for_node(
                format!("Node '{}' is not connected to anything", node.name),
                node.id,
            ));
        } else if !has_incoming && workflow.nodes.len() > 1 {
            issues.push(ValidationIssue::warning_for_node(
                format!("Node '{}' has no incoming connections", node.name),
                node.id,
            ));
        }
    }
}
