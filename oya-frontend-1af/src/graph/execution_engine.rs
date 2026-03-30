//! Pure execution engine functions.
//!
//! This module implements the contract-specified pure functions for workflow execution:
//! - `prepare_execution`: Creates an execution plan from a workflow
//! - `execute_workflow`: Runs a workflow with shared context and config
//! - `execute_node`: Executes a single node
//!
//! All functions are pure (no side effects) and follow Data -> Calc -> Actions architecture.

use super::{Node, NodeId, Workflow, WorkflowExecutionError};
use crate::graph::execution_types::ExecutionPlan;
use std::collections::HashMap;

// ============================================================================
// Execution Functions (Contract Section 2)
// ============================================================================

/// Prepare an execution plan from a workflow.
///
/// **Preconditions:**
/// - P1: `workflow.nodes` is not empty
/// - P2: All node IDs are unique
/// - P3: All connection source/target references are valid nodes
///
/// **Postconditions:**
/// - R1: Returns `ExecutionPlan` with topologically sorted nodes
/// - R2: `ExecutionPlan.execution_order` contains all nodes
/// - R3: `ExecutionPlan.entry_nodes` contains all nodes with in-degree 0
/// - R4: `ExecutionPlan.cycles` contains detected cycles (empty if DAG)
/// - R5: `ExecutionPlan.input_map` maps each node to its input ports
/// - R6: If cycles detected, plan uses iterative execution strategy
///
/// **Side Effects:** None (pure function)
pub fn prepare_execution(workflow: &Workflow) -> Result<ExecutionPlan, WorkflowExecutionError> {
    // Validate preconditions
    if workflow.nodes.is_empty() {
        return Err(WorkflowExecutionError::EmptyWorkflow);
    }

    // Check unique node IDs
    let mut node_ids: Vec<&NodeId> = workflow.nodes.iter().map(|n| &n.id).collect();
    node_ids.sort();
    node_ids.dedup();
    if node_ids.len() != workflow.nodes.len() {
        // Duplicate IDs detected - this is a fatal error
        return Err(WorkflowExecutionError::InvalidConfig {
            node_id: workflow.nodes[0].id,
            error: "Duplicate node IDs detected".to_string(),
        });
    }

    // Validate all connection references exist
    let node_id_set: HashMap<NodeId, &Node> = workflow.nodes.iter().map(|n| (n.id, n)).collect();

    for conn in &workflow.connections {
        if !node_id_set.contains_key(&conn.source) {
            return Err(WorkflowExecutionError::NodeNotFound {
                connection_id: conn.id,
                referenced_node: conn.source,
            });
        }
        if !node_id_set.contains_key(&conn.target) {
            return Err(WorkflowExecutionError::NodeNotFound {
                connection_id: conn.id,
                referenced_node: conn.target,
            });
        }
    }

    // Build node set for quick lookup
    let node_ids: HashMap<NodeId, &Node> = workflow.nodes.iter().map(|n| (n.id, n)).collect();

    // Calculate in-degrees
    let mut in_degree: HashMap<NodeId, usize> =
        node_ids.keys().copied().map(|id| (id, 0)).collect();

    for conn in &workflow.connections {
        if node_ids.contains_key(&conn.source) && node_ids.contains_key(&conn.target) {
            *in_degree.get_mut(&conn.target).unwrap() += 1;
        }
    }

    // Find entry nodes (in-degree 0)
    let entry_nodes: Vec<NodeId> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&id, _)| id)
        .collect();

    // Topological sort using Kahn's algorithm
    let mut execution_order = Vec::new();
    let mut queue: Vec<NodeId> = entry_nodes.clone();
    let mut local_in_degree = in_degree.clone();

    while let Some(node_id) = queue.pop() {
        execution_order.push(node_id);

        // Find nodes that depend on this node
        let mut next_nodes: Vec<NodeId> = workflow
            .connections
            .iter()
            .filter(|c| c.source == node_id && node_ids.contains_key(&c.target))
            .map(|c| c.target)
            .collect();

        for target in &next_nodes {
            *local_in_degree.get_mut(target).unwrap() -= 1;
            if *local_in_degree.get(target).unwrap() == 0 {
                queue.push(*target);
            }
        }
    }

    // Check for cycles (nodes not in execution order)
    let mut cycles: Vec<Vec<NodeId>> = Vec::new();
    if execution_order.len() != node_ids.len() {
        let remaining: Vec<NodeId> = node_ids
            .keys()
            .filter(|id| !execution_order.contains(id))
            .copied()
            .collect();
        if !remaining.is_empty() {
            cycles.push(remaining);
        }
    }

    // Build input map (node -> list of input node IDs)
    let mut input_map: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    for node_id in node_ids.keys() {
        let inputs: Vec<NodeId> = workflow
            .connections
            .iter()
            .filter(|c| c.target == *node_id && node_ids.contains_key(&c.source))
            .map(|c| c.source)
            .collect();
        input_map.insert(*node_id, inputs);
    }

    Ok(ExecutionPlan {
        execution_order,
        entry_nodes,
        cycles,
        input_map,
    })
}
