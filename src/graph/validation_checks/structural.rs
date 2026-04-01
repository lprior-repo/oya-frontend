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

/// Single-pass connection scan: build incoming/outgoing sets once (O(n+m))
/// instead of scanning all connections per node (O(n*m)).
pub fn validate_orphan_nodes(workflow: &Workflow, issues: &mut Vec<ValidationIssue>) {
    use std::collections::HashSet;

    // Build sets of nodes that have incoming and outgoing connections in one pass
    let mut has_incoming: HashSet<NodeId> = HashSet::new();
    let mut has_outgoing: HashSet<NodeId> = HashSet::new();

    for conn in &workflow.connections {
        has_outgoing.insert(conn.source);
        has_incoming.insert(conn.target);
    }

    for node in &workflow.nodes {
        if node.category == NodeCategory::Entry {
            continue;
        }

        let incoming = has_incoming.contains(&node.id);
        let outgoing = has_outgoing.contains(&node.id);

        if !incoming && !outgoing && workflow.nodes.len() > 1 {
            issues.push(ValidationIssue::warning_for_node(
                format!("Node '{}' is not connected to anything", node.name),
                node.id,
            ));
        } else if !incoming && workflow.nodes.len() > 1 {
            issues.push(ValidationIssue::warning_for_node(
                format!("Node '{}' has no incoming connections", node.name),
                node.id,
            ));
        }
    }
}
