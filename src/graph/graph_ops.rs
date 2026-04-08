//! Shared graph operations used across multiple modules.
//!
//! This module centralizes common graph algorithms and lookup patterns
//! to eliminate duplication across `connectivity`, `execution`,
//! `execution_engine`, and `validation_checks`.

#![allow(clippy::implicit_hasher)]

use crate::graph::{Connection, Node, NodeId};
use std::collections::{HashMap, HashSet};

// ===========================================================================
// Node Lookup
// ===========================================================================

/// Build a `NodeId -> &Node` lookup map from a node slice.
///
/// O(n) construction, then O(1) lookups. Use this instead of repeated
/// `nodes.iter().find(|n| n.id == id)` calls which are O(n) each.
#[must_use]
pub fn build_node_lookup(nodes: &[Node]) -> HashMap<NodeId, &Node> {
    nodes.iter().map(|n| (n.id, n)).collect()
}

/// Build a `NodeId -> &mut Node` lookup map from a mutable node slice.
#[must_use]
pub fn build_node_lookup_mut(nodes: &mut [Node]) -> HashMap<NodeId, &mut Node> {
    nodes.iter_mut().map(|n| (n.id, n)).collect()
}

/// Collect all node IDs into a `HashSet`.
///
/// Replaces the repeated `nodes.iter().map(|n| n.id).collect()` pattern.
#[must_use]
pub fn collect_node_ids(nodes: &[Node]) -> HashSet<NodeId> {
    nodes.iter().map(|n| n.id).collect()
}

// ===========================================================================
// Adjacency Maps
// ===========================================================================

/// Build an outgoing adjacency map: `source -> [targets]`.
///
/// Only includes connections where both endpoints exist in `valid_node_ids`.
#[must_use]
pub fn build_outgoing_adjacency(
    connections: &[Connection],
    valid_node_ids: &HashSet<NodeId>,
) -> HashMap<NodeId, Vec<NodeId>> {
    let mut adjacency: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    for conn in connections {
        if valid_node_ids.contains(&conn.source) && valid_node_ids.contains(&conn.target) {
            adjacency.entry(conn.source).or_default().push(conn.target);
        }
    }
    adjacency
}

/// Build a reverse adjacency map: `target -> [sources]`.
///
/// Only includes connections where both endpoints exist in `valid_node_ids`.
#[must_use]
pub fn build_reverse_adjacency(
    connections: &[Connection],
    valid_node_ids: &HashSet<NodeId>,
) -> HashMap<NodeId, Vec<NodeId>> {
    let mut reverse: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    for conn in connections {
        if valid_node_ids.contains(&conn.source) && valid_node_ids.contains(&conn.target) {
            reverse.entry(conn.target).or_default().push(conn.source);
        }
    }
    reverse
}

/// Build outgoing adjacency map with in-degree counts.
///
/// Returns both the adjacency map and a map of `node_id -> in_degree`.
/// In-degrees default to 0 for all nodes in `valid_node_ids`.
#[must_use]
pub fn build_adjacency_with_in_degree(
    connections: &[Connection],
    valid_node_ids: &HashSet<NodeId>,
) -> (HashMap<NodeId, Vec<NodeId>>, HashMap<NodeId, usize>) {
    let mut adjacency: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    let mut in_degree: HashMap<NodeId, usize> = valid_node_ids.iter().map(|&id| (id, 0)).collect();

    for conn in connections {
        if valid_node_ids.contains(&conn.source) && valid_node_ids.contains(&conn.target) {
            adjacency.entry(conn.source).or_default().push(conn.target);
            if let Some(deg) = in_degree.get_mut(&conn.target) {
                *deg += 1;
            }
        }
    }

    (adjacency, in_degree)
}

/// Build incoming/outgoing membership sets from connections.
///
/// Returns `(has_incoming, has_outgoing)` sets for O(1) membership checks.
/// Used by orphan node detection and entry node finding.
#[must_use]
pub fn build_connection_membership(
    connections: &[Connection],
) -> (HashSet<NodeId>, HashSet<NodeId>) {
    let mut has_incoming: HashSet<NodeId> = HashSet::new();
    let mut has_outgoing: HashSet<NodeId> = HashSet::new();

    for conn in connections {
        has_outgoing.insert(conn.source);
        has_incoming.insert(conn.target);
    }

    (has_incoming, has_outgoing)
}

// ===========================================================================
// Graph Traversal
// ===========================================================================

/// Find all nodes reachable from `start_ids` via outgoing edges.
///
/// Uses an iterative DFS with a visited set. Returns the set of all
/// visited node IDs (including the start IDs themselves).
#[must_use]
pub fn find_reachable(
    start_ids: &[NodeId],
    outgoing: &HashMap<NodeId, Vec<NodeId>>,
) -> HashSet<NodeId> {
    let mut visited = HashSet::new();
    let mut stack: Vec<NodeId> = start_ids.to_vec();

    while let Some(current) = stack.pop() {
        if visited.insert(current) {
            if let Some(targets) = outgoing.get(&current) {
                for target in targets {
                    if !visited.contains(target) {
                        stack.push(*target);
                    }
                }
            }
        }
    }

    visited
}

/// Check if a path exists from `from` to `to` via the given connections.
///
/// Uses an iterative DFS. If `from == to`, returns `true` only if there
/// is an explicit self-loop connection.
#[must_use]
pub fn path_exists(connections: &[Connection], from: NodeId, to: NodeId) -> bool {
    if from == to {
        return connections
            .iter()
            .any(|conn| conn.source == from && conn.target == from);
    }

    let mut visited = HashSet::new();
    let mut stack = vec![from];

    while let Some(current) = stack.pop() {
        if current == to {
            return true;
        }

        if visited.insert(current) {
            connections
                .iter()
                .filter(|connection| connection.source == current)
                .for_each(|connection| stack.push(connection.target));
        }
    }

    false
}

// ===========================================================================
// Topological Sort (Kahn's Algorithm)
// ===========================================================================

/// Perform topological sort using Kahn's algorithm.
///
/// Returns the topological order on success, or the set of unprocessed
/// node IDs if a cycle was detected.
///
/// # Arguments
/// * `node_ids` - All node IDs to sort
/// * `adjacency` - Pre-built outgoing adjacency map
/// * `in_degree` - Pre-computed in-degree counts
/// * `compare` - Optional comparison function for tie-breaking among nodes
///   with the same in-degree
///
/// # Errors
///
/// Returns `Err(HashSet<NodeId>)` containing all nodes that could not be
/// ordered (i.e., nodes involved in cycles).
pub fn topological_sort<F>(
    node_ids: &HashSet<NodeId>,
    adjacency: &HashMap<NodeId, Vec<NodeId>>,
    in_degree: &HashMap<NodeId, usize>,
    compare: F,
) -> Result<Vec<NodeId>, HashSet<NodeId>>
where
    F: Fn(&NodeId, &NodeId) -> std::cmp::Ordering,
{
    let mut queue = Vec::with_capacity(node_ids.len());
    let mut local_in_degree = in_degree.clone();

    // Start with all nodes that have no dependencies
    for &id in node_ids {
        if local_in_degree.get(&id).is_some_and(|&d| d == 0) {
            queue.push(id);
        }
    }
    queue.sort_by(|a, b| compare(a, b));

    let mut result = Vec::with_capacity(node_ids.len());

    while !queue.is_empty() {
        let id = queue.remove(0);
        result.push(id);

        if let Some(targets) = adjacency.get(&id) {
            for target in targets {
                if let Some(deg) = local_in_degree.get_mut(target) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(*target);
                    }
                }
            }
        }

        queue.sort_by(|a, b| compare(a, b));
    }

    if result.len() == node_ids.len() {
        Ok(result)
    } else {
        let result_set: HashSet<NodeId> = result.iter().copied().collect();
        let remaining: HashSet<NodeId> = node_ids.difference(&result_set).copied().collect();
        Err(remaining)
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]
mod tests {
    use super::*;
    use crate::graph::PortName;
    use uuid::Uuid;

    fn make_connection(source: NodeId, target: NodeId) -> Connection {
        Connection {
            id: Uuid::new_v4(),
            source,
            target,
            source_port: PortName::from("main"),
            target_port: PortName::from("main"),
        }
    }

    // --- build_node_lookup ---

    #[test]
    fn given_nodes_when_building_lookup_then_all_ids_are_mapped() {
        let a = NodeId::new();
        let b = NodeId::new();
        let nodes = vec![
            Node::from_workflow_node("a".into(), crate::graph::WorkflowNode::default(), 0.0, 0.0),
            Node::from_workflow_node("b".into(), crate::graph::WorkflowNode::default(), 0.0, 0.0),
        ];
        // Override IDs for the test
        let mut nodes = nodes;
        nodes[0].id = a;
        nodes[1].id = b;

        let lookup = build_node_lookup(&nodes);
        assert!(lookup.contains_key(&a));
        assert!(lookup.contains_key(&b));
        assert_eq!(lookup.len(), 2);
    }

    // --- collect_node_ids ---

    #[test]
    fn given_nodes_when_collecting_ids_then_all_ids_are_present() {
        let a = NodeId::new();
        let b = NodeId::new();
        let mut nodes = vec![Node::from_workflow_node(
            "a".into(),
            crate::graph::WorkflowNode::default(),
            0.0,
            0.0,
        )];
        nodes[0].id = a;

        let ids = collect_node_ids(&nodes);
        assert!(ids.contains(&a));
        assert!(!ids.contains(&b));
    }

    // --- build_outgoing_adjacency ---

    #[test]
    fn given_connections_when_building_outgoing_adjacency_then_map_is_correct() {
        let a = NodeId::new();
        let b = NodeId::new();
        let c = NodeId::new();
        let valid: HashSet<NodeId> = [a, b, c].into_iter().collect();

        let connections = vec![make_connection(a, b), make_connection(b, c)];
        let adj = build_outgoing_adjacency(&connections, &valid);

        assert_eq!(adj.get(&a), Some(&vec![b]));
        assert_eq!(adj.get(&b), Some(&vec![c]));
        assert_eq!(adj.get(&c), None);
    }

    // --- build_reverse_adjacency ---

    #[test]
    fn given_connections_when_building_reverse_adjacency_then_map_is_correct() {
        let a = NodeId::new();
        let b = NodeId::new();
        let valid: HashSet<NodeId> = [a, b].into_iter().collect();

        let connections = vec![make_connection(a, b)];
        let rev = build_reverse_adjacency(&connections, &valid);

        assert_eq!(rev.get(&b), Some(&vec![a]));
        assert_eq!(rev.get(&a), None);
    }

    // --- build_adjacency_with_in_degree ---

    #[test]
    fn given_connections_when_building_adjacency_with_in_degree_then_degrees_are_correct() {
        let a = NodeId::new();
        let b = NodeId::new();
        let c = NodeId::new();
        let valid: HashSet<NodeId> = [a, b, c].into_iter().collect();

        let connections = vec![make_connection(a, b), make_connection(a, c)];
        let (adj, in_deg) = build_adjacency_with_in_degree(&connections, &valid);

        assert_eq!(in_deg.get(&a), Some(&0));
        assert_eq!(in_deg.get(&b), Some(&1));
        assert_eq!(in_deg.get(&c), Some(&1));
        assert_eq!(adj.get(&a), Some(&vec![b, c]));
    }

    // --- build_connection_membership ---

    #[test]
    fn given_connections_when_building_membership_then_sets_are_correct() {
        let a = NodeId::new();
        let b = NodeId::new();
        let connections = vec![make_connection(a, b)];
        let (incoming, outgoing) = build_connection_membership(&connections);

        assert!(outgoing.contains(&a));
        assert!(!outgoing.contains(&b));
        assert!(incoming.contains(&b));
        assert!(!incoming.contains(&a));
    }

    // --- find_reachable ---

    #[test]
    fn given_linear_chain_when_finding_reachable_then_all_downstream_found() {
        let a = NodeId::new();
        let b = NodeId::new();
        let c = NodeId::new();

        let mut outgoing = HashMap::new();
        outgoing.insert(a, vec![b]);
        outgoing.insert(b, vec![c]);

        let reachable = find_reachable(&[a], &outgoing);
        assert!(reachable.contains(&a));
        assert!(reachable.contains(&b));
        assert!(reachable.contains(&c));
        assert_eq!(reachable.len(), 3);
    }

    #[test]
    fn given_disconnected_graph_when_finding_reachable_then_only_connected_found() {
        let a = NodeId::new();
        let b = NodeId::new();
        let c = NodeId::new();

        let mut outgoing = HashMap::new();
        outgoing.insert(a, vec![b]);
        // c is not connected

        let reachable = find_reachable(&[a], &outgoing);
        assert!(reachable.contains(&a));
        assert!(reachable.contains(&b));
        assert!(!reachable.contains(&c));
    }

    // --- path_exists ---

    #[test]
    fn given_direct_edge_when_checking_path_exists_then_true() {
        let a = NodeId::new();
        let b = NodeId::new();
        let connections = vec![make_connection(a, b)];

        assert!(path_exists(&connections, a, b));
        assert!(!path_exists(&connections, b, a));
    }

    #[test]
    fn given_transitive_path_when_checking_path_exists_then_true() {
        let a = NodeId::new();
        let b = NodeId::new();
        let c = NodeId::new();
        let connections = vec![make_connection(a, b), make_connection(b, c)];

        assert!(path_exists(&connections, a, c));
    }

    #[test]
    fn given_same_node_no_self_loop_when_checking_path_exists_then_false() {
        let a = NodeId::new();
        assert!(!path_exists(&[], a, a));
    }

    #[test]
    fn given_self_loop_when_checking_path_exists_from_node_to_itself_then_true() {
        let a = NodeId::new();
        let connections = vec![make_connection(a, a)];
        assert!(path_exists(&connections, a, a));
    }

    // --- topological_sort ---

    #[test]
    fn given_linear_chain_when_topsort_then_order_follows_dependencies() {
        let a = NodeId::new();
        let b = NodeId::new();
        let c = NodeId::new();
        let node_ids: HashSet<NodeId> = [a, b, c].into_iter().collect();

        let mut adj = HashMap::new();
        adj.insert(a, vec![b]);
        adj.insert(b, vec![c]);

        let mut in_deg = HashMap::new();
        in_deg.insert(a, 0);
        in_deg.insert(b, 1);
        in_deg.insert(c, 1);

        let result = topological_sort(&node_ids, &adj, &in_deg, |_, _| std::cmp::Ordering::Equal);

        let order = result.expect("linear chain should produce valid order");
        let pos_a = order.iter().position(|&id| id == a).expect("a");
        let pos_b = order.iter().position(|&id| id == b).expect("b");
        let pos_c = order.iter().position(|&id| id == c).expect("c");
        assert!(pos_a < pos_b);
        assert!(pos_b < pos_c);
    }

    #[test]
    fn given_cycle_when_topsort_then_remaining_nodes_returned() {
        let a = NodeId::new();
        let b = NodeId::new();
        let node_ids: HashSet<NodeId> = [a, b].into_iter().collect();

        let mut adj = HashMap::new();
        adj.insert(a, vec![b]);
        adj.insert(b, vec![a]);

        let mut in_deg = HashMap::new();
        in_deg.insert(a, 1);
        in_deg.insert(b, 1);

        let result = topological_sort(&node_ids, &adj, &in_deg, |_, _| std::cmp::Ordering::Equal);

        assert!(result.is_err());
        let remaining = result.err().expect("cycle should remain");
        assert_eq!(remaining.len(), 2);
    }
}
