#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

use super::execution_types::ExecutionConfig;
use super::expressions::ExpressionContext;
use super::graph_ops;
use super::WorkflowExecutionError;
use super::{NodeId, Workflow};

// ===========================================================================
// Execution Plan Preparation (Topological Sort)
// ===========================================================================

impl Workflow {
    /// Compare two node IDs by execution priority (x, then y, then name).
    ///
    /// Accepts a pre-built lookup map to avoid repeated O(n) linear scans
    /// during sorting, turning each comparison from O(n) to O(1).
    fn compare_execution_priority(
        node_map: &std::collections::HashMap<NodeId, &super::Node>,
        a: NodeId,
        b: NodeId,
    ) -> std::cmp::Ordering {
        let node_a = node_map.get(&a);
        let node_b = node_map.get(&b);

        match (node_a, node_b) {
            (Some(left), Some(right)) => left
                .x
                .partial_cmp(&right.x)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    left.y
                        .partial_cmp(&right.y)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .then_with(|| left.name.cmp(&right.name)),
            _ => std::cmp::Ordering::Equal,
        }
    }

    /// Checks if all nodes are connected (no isolated subgraphs).
    ///
    /// This ensures graph connectivity invariant - all nodes must be
    /// reachable from at least one entry node.
    ///
    /// Optimized to build adjacency maps once (O(n+m)) instead of scanning
    /// connections per node (O(n*m)).
    fn verify_graph_connectivity(&self) -> Result<(), WorkflowExecutionError> {
        if self.nodes.is_empty() {
            return Ok(());
        }

        let node_ids = graph_ops::collect_node_ids(&self.nodes);
        let outgoing = graph_ops::build_outgoing_adjacency(&self.connections, &node_ids);
        let (has_incoming, _) = graph_ops::build_connection_membership(&self.connections);

        // Find a starting node (any node with no incoming edges, or first node)
        let entry_node = self
            .nodes
            .iter()
            .find(|node| !has_incoming.contains(&node.id))
            .map(|node| node.id);

        let start_node = if let Some(node) = entry_node {
            node
        } else if let Some(first_node) = self.nodes.first() {
            first_node.id
        } else {
            return Err(WorkflowExecutionError::InvalidWorkflowState {
                reason: "verify_graph_connectivity: graph should have at least one node"
                    .to_string(),
            });
        };

        let visited = graph_ops::find_reachable(&[start_node], &outgoing);

        if visited.len() != node_ids.len() {
            let unreachable: Vec<NodeId> = node_ids.difference(&visited).copied().collect();

            if !unreachable.is_empty() {
                return Err(WorkflowExecutionError::InvalidWorkflowState {
                    reason: format!(
                        "graph connectivity violation: isolated subgraph detected with {} unreachable nodes: {:?}",
                        unreachable.len(),
                        unreachable
                    ),
                });
            }
        }

        Ok(())
    }

    /// Validates that all dependencies reference existing nodes.
    fn validate_dependencies_exist(&self) -> Result<(), WorkflowExecutionError> {
        let node_ids = graph_ops::collect_node_ids(&self.nodes);

        let mut missing_deps: Vec<(NodeId, NodeId)> = Vec::new();

        for conn in &self.connections {
            if !node_ids.contains(&conn.source) {
                continue;
            }
            if !node_ids.contains(&conn.target) {
                missing_deps.push((conn.source, conn.target));
            }
        }

        if !missing_deps.is_empty() {
            // Group by source node
            let mut nodes_with_missing: std::collections::HashMap<NodeId, Vec<NodeId>> =
                std::collections::HashMap::new();

            for (source, missing) in &missing_deps {
                nodes_with_missing
                    .entry(*source)
                    .or_default()
                    .push(*missing);
            }

            let nodes: Vec<NodeId> = nodes_with_missing.keys().copied().collect();
            let all_missing: Vec<NodeId> = missing_deps.iter().map(|(_, m)| *m).collect();

            return Err(WorkflowExecutionError::UnresolvedDependencies {
                nodes,
                missing_deps: all_missing,
            });
        }

        Ok(())
    }

    /// Checks for duplicate connections in the connection list.
    fn check_duplicate_connections(&self) -> Result<(), WorkflowExecutionError> {
        let mut seen: std::collections::HashMap<(NodeId, NodeId), Vec<uuid::Uuid>> =
            std::collections::HashMap::new();

        for conn in &self.connections {
            let key = (conn.source, conn.target);
            seen.entry(key).or_default().push(conn.id);
        }

        for ((source, target), ids) in &seen {
            if ids.len() > 1 {
                return Err(WorkflowExecutionError::InvalidWorkflowState {
                    reason: format!(
                        "duplicate connection from node {} to node {} ({} connections)",
                        source,
                        target,
                        ids.len()
                    ),
                });
            }
        }

        Ok(())
    }

    /// Returns error if workflow has dirty state.
    fn check_dirty_state(&self) -> Result<(), WorkflowExecutionError> {
        if !self.execution_queue.is_empty() {
            return Err(WorkflowExecutionError::InvalidWorkflowState {
                reason: "execution_queue is not empty".to_string(),
            });
        }

        // Check if any node is executing (dirty executed state)
        if self.nodes.iter().any(|n| n.executing) {
            return Err(WorkflowExecutionError::InvalidWorkflowState {
                reason: "some nodes are in executing state".to_string(),
            });
        }

        Ok(())
    }

    /// Returns error if workflow is empty.
    const fn check_non_empty(&self) -> Result<(), WorkflowExecutionError> {
        if self.nodes.is_empty() {
            return Err(WorkflowExecutionError::EmptyWorkflow);
        }
        Ok(())
    }

    /// Returns error if self-references detected.
    fn check_self_references(&self) -> Result<(), WorkflowExecutionError> {
        for conn in &self.connections {
            if conn.source == conn.target {
                return Err(WorkflowExecutionError::CycleDetected {
                    cycle_nodes: vec![conn.source],
                });
            }
        }
        Ok(())
    }

    /// Prepare the workflow for execution.
    ///
    /// # Errors
    ///
    /// Returns `WorkflowExecutionError` if the workflow is invalid.
    pub fn prepare_run(&mut self) -> Result<(), WorkflowExecutionError> {
        // Precondition checks (Data layer) - FIRST, before any state changes
        // This ensures we fail fast if there's a problem
        self.check_non_empty()?;
        self.check_dirty_state()?;
        self.validate_dependencies_exist()?;
        self.check_duplicate_connections()?;
        self.verify_graph_connectivity()?;
        self.check_self_references()?;

        // Cycle detection before building queue (Calc layer)
        if let Some(cycle) = self.find_cycle() {
            return Err(WorkflowExecutionError::CycleDetected { cycle_nodes: cycle });
        }

        // Build execution queue using Kahn's algorithm (Calc layer)
        let execution_queue = self.build_execution_queue()?;

        // Update state (Action layer) - reset all node states
        self.execution_queue = execution_queue;
        self.current_step = 0;

        for node in &mut self.nodes {
            node.executing = false;
            node.last_output = None;
            node.skipped = false;
            node.error = None;
            let _ = Self::set_node_pending_status(node);
        }

        // Reset memory tracking
        self.current_memory_bytes = 0;
        self.execution_failed = false;

        Ok(())
    }

    /// Uses Kahn's algorithm to build the execution queue.
    ///
    /// Returns a vector of node IDs in topological order.
    ///
    /// Optimized to pre-build an adjacency map (source -> targets) and a node
    /// lookup `HashMap` so that each comparison during sort is O(1) instead of O(n),
    /// and finding dependents is O(k) instead of O(m).
    fn build_execution_queue(&self) -> Result<Vec<NodeId>, WorkflowExecutionError> {
        let mut queue = Vec::with_capacity(self.nodes.len());
        let node_ids: std::collections::HashSet<NodeId> = self.nodes.iter().map(|n| n.id).collect();

        // Pre-build node lookup for O(1) comparisons during sort
        let node_map: std::collections::HashMap<NodeId, &super::Node> =
            self.nodes.iter().map(|n| (n.id, n)).collect();

        // Pre-build adjacency map: source -> list of targets, in a single pass
        let mut adjacency: std::collections::HashMap<NodeId, Vec<NodeId>> =
            std::collections::HashMap::new();
        let mut in_degree: std::collections::HashMap<NodeId, usize> =
            self.nodes.iter().map(|n| (n.id, 0)).collect();

        for conn in &self.connections {
            if node_ids.contains(&conn.source) && node_ids.contains(&conn.target) {
                adjacency.entry(conn.source).or_default().push(conn.target);
                if let Some(count) = in_degree.get_mut(&conn.target) {
                    *count += 1;
                }
            }
        }

        // Start with all nodes that have no dependencies
        let mut available: Vec<NodeId> = self
            .nodes
            .iter()
            .filter(|n| in_degree.get(&n.id).is_some_and(|&d| d == 0))
            .map(|n| n.id)
            .collect();

        available.sort_by(|a, b| Self::compare_execution_priority(&node_map, *a, *b));

        // Process nodes in topological order
        while !available.is_empty() {
            let id = available.remove(0);
            queue.push(id);

            // Use pre-built adjacency map for O(k) lookup instead of O(m) scan
            if let Some(targets) = adjacency.get(&id) {
                for target in targets {
                    if let Some(count) = in_degree.get_mut(target) {
                        *count -= 1;
                        if *count == 0 {
                            available.push(*target);
                        }
                    }
                }
            }

            available.sort_by(|a, b| Self::compare_execution_priority(&node_map, *a, *b));
        }

        // Verify all nodes were processed (no cycles)
        if queue.len() != self.nodes.len() {
            let queue_set: std::collections::HashSet<NodeId> =
                queue.iter().copied().collect();
            let remaining: Vec<NodeId> = node_ids
                .difference(&queue_set)
                .copied()
                .collect();

            return Err(WorkflowExecutionError::CycleDetected {
                cycle_nodes: remaining,
            });
        }

        Ok(queue)
    }

    /// DFS-based cycle detection that returns actual cycle nodes.
    ///
    /// Uses a pre-built reverse adjacency map for O(1) neighbor lookup
    /// instead of scanning all connections per DFS call.
    fn detect_cycle_dfs(
        reverse_adj: &std::collections::HashMap<NodeId, Vec<NodeId>>,
        start_node: NodeId,
        visited: &mut std::collections::HashSet<NodeId>,
        path_set: &mut std::collections::HashSet<NodeId>,
        path: &mut Vec<NodeId>,
    ) -> Option<Vec<NodeId>> {
        visited.insert(start_node);
        path.push(start_node);
        path_set.insert(start_node);

        // Use pre-built reverse adjacency for O(1) lookup instead of O(m) scan
        let dependents = reverse_adj
            .get(&start_node)
            .map(std::vec::Vec::as_slice)
            .unwrap_or_default();

        for &dependent in dependents {
            // Check if dependent is in current path (back edge = cycle)
            if path_set.contains(&dependent) {
                // Found a cycle - extract it
                let cycle_start = path.iter().position(|&n| n == dependent)?;
                let cycle: Vec<NodeId> = path[cycle_start..].to_vec();
                return Some(cycle);
            }
            // Recurse if not yet visited
            if !visited.contains(&dependent) {
                if let Some(cycle) =
                    Self::detect_cycle_dfs(reverse_adj, dependent, visited, path_set, path)
                {
                    return Some(cycle);
                }
            }
            // If node is visited but not in current path, skip it
            // (it was already explored in a previous branch)
        }

        path.pop();
        path_set.remove(&start_node);
        None
    }

    /// Finds a cycle in the workflow graph using DFS.
    ///
    /// Returns a vector of node IDs that form the cycle if one exists.
    /// The cycle nodes are those that are part of the cycle.
    ///
    /// Builds a reverse adjacency map once for O(n+m) complexity instead
    /// of O(n*m) from scanning connections on each DFS step.
    fn find_cycle(&self) -> Option<Vec<NodeId>> {
        let node_ids: std::collections::HashSet<NodeId> = self.nodes.iter().map(|n| n.id).collect();

        // Build reverse adjacency: target -> list of sources (dependents)
        let mut reverse_adj: std::collections::HashMap<NodeId, Vec<NodeId>> =
            std::collections::HashMap::new();
        for conn in &self.connections {
            if node_ids.contains(&conn.source) && node_ids.contains(&conn.target) {
                reverse_adj.entry(conn.target).or_default().push(conn.source);
            }
        }

        let mut visited = std::collections::HashSet::new();
        let mut path = Vec::new();
        let mut path_set = std::collections::HashSet::new();

        // Try starting from each node
        for node in &self.nodes {
            if !visited.contains(&node.id) {
                if let Some(cycle) = Self::detect_cycle_dfs(
                    &reverse_adj,
                    node.id,
                    &mut visited,
                    &mut path_set,
                    &mut path,
                ) {
                    return Some(cycle);
                }
            }
        }

        None
    }

    pub(super) fn collect_descendants(
        &self,
        start_ids: &[NodeId],
    ) -> std::collections::HashSet<NodeId> {
        let mut visited = std::collections::HashSet::new();
        let mut stack: Vec<NodeId> = start_ids.to_vec();

        while let Some(current) = stack.pop() {
            if !visited.insert(current) {
                continue;
            }

            for target in self
                .connections
                .iter()
                .filter(|c| c.source == current)
                .map(|c| c.target)
            {
                if !visited.contains(&target) {
                    stack.push(target);
                }
            }
        }

        visited
    }

    // ===========================================================================
    // Execution Configuration Management
    // ===========================================================================

    /// Sets the execution configuration for this workflow.
    ///
    /// This method allows configuring memory limits, timeouts, and other
    /// runtime constraints before execution.
    ///
    /// # Arguments
    /// * `config` - The execution configuration to apply
    ///
    /// # Returns
    /// The workflow with the new configuration applied
    ///
    /// # Examples
    /// ```
    /// use oya_frontend::graph::{Workflow, execution_types::ExecutionConfig};
    ///
    /// let workflow = Workflow::new().with_execution_config(
    ///     ExecutionConfig::new().with_memory_limit(1024 * 1024) // 1MB limit
    /// );
    /// assert_eq!(workflow.execution_config.memory_limit_bytes, Some(1024 * 1024));
    /// ```
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_execution_config(mut self, config: ExecutionConfig) -> Self {
        self.execution_config = config;
        self
    }

    /// Sets a memory limit for this workflow execution.
    ///
    /// When the total memory usage of all node outputs exceeds this limit,
    /// execution will stop and the workflow will be marked as failed.
    ///
    /// # Arguments
    /// * `memory_limit_bytes` - Maximum memory allowed in bytes
    ///
    /// # Returns
    /// The workflow with the memory limit configured
    ///
    /// # Examples
    /// ```
    /// use oya_frontend::graph::Workflow;
    ///
    /// let workflow = Workflow::new().with_memory_limit(1024 * 1024); // 1MB
    /// assert_eq!(workflow.execution_config.memory_limit_bytes, Some(1024 * 1024));
    /// ```
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // uses RwLockWriteGuard which is not const-safe
    pub fn with_memory_limit(mut self, memory_limit_bytes: u64) -> Self {
        self.execution_config = self.execution_config.with_memory_limit(memory_limit_bytes);
        self
    }

    // ===========================================================================
    // Expression Resolution
    // ===========================================================================

    #[must_use]
    pub fn resolve_expressions(&self, config: &serde_json::Value) -> serde_json::Value {
        self.resolve_expressions_with_depth(config, 0)
    }

    /// Resolves expressions with a depth limit to prevent stack overflow.
    /// Max depth is 100 to prevent excessive recursion.
    fn resolve_expressions_with_depth(
        &self,
        config: &serde_json::Value,
        depth: usize,
    ) -> serde_json::Value {
        // MAJOR: Enforce depth limit to prevent stack overflow
        const MAX_DEPTH: usize = 100;
        if depth > MAX_DEPTH {
            // Return config unchanged if depth exceeded
            return config.clone();
        }

        let ctx = ExpressionContext::new(&self.nodes);
        match config {
            serde_json::Value::String(s) => {
                if s.starts_with("{{") && s.ends_with("}}") {
                    let inner = s[2..s.len() - 2].trim();
                    return ctx.resolve(inner);
                }
                config.clone()
            }
            serde_json::Value::Object(map) => {
                let new_map = map
                    .iter()
                    .map(|(k, v)| (k.clone(), self.resolve_expressions_with_depth(v, depth + 1)))
                    .collect();
                serde_json::Value::Object(new_map)
            }
            serde_json::Value::Array(arr) => serde_json::Value::Array(
                arr.iter()
                    .map(|v| self.resolve_expressions_with_depth(v, depth + 1))
                    .collect(),
            ),
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {
                config.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Connection, PortName};
    use std::collections::HashMap;
    use uuid::Uuid;

    // ---------------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------------

    fn main_port() -> PortName {
        PortName::from("main")
    }

    fn add_connection(workflow: &mut Workflow, source: NodeId, target: NodeId) {
        workflow.connections.push(Connection {
            id: Uuid::new_v4(),
            source,
            target,
            source_port: main_port(),
            target_port: main_port(),
        });
    }

    /// Calls `build_execution_queue` through `prepare_run`, which resets
    /// dirty state so it can be called repeatedly. Returns the queue or
    /// the first error encountered.
    fn prepare_and_get_queue(workflow: &mut Workflow) -> Result<Vec<NodeId>, WorkflowExecutionError> {
        workflow.prepare_run()?;
        Ok(workflow.execution_queue.clone())
    }

    // ---------------------------------------------------------------------------
    // build_execution_queue — topological ordering
    // ---------------------------------------------------------------------------

    #[test]
    fn given_linear_chain_when_building_queue_then_order_follows_dependencies() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let b = workflow.add_node("run", 10.0, 0.0);
        let c = workflow.add_node("run", 20.0, 0.0);

        add_connection(&mut workflow, a, b);
        add_connection(&mut workflow, b, c);

        let queue = prepare_and_get_queue(&mut workflow);
        let order = queue.expect("linear chain should produce a valid queue");

        let pos_a = order.iter().position(|&id| id == a).expect("a in queue");
        let pos_b = order.iter().position(|&id| id == b).expect("b in queue");
        let pos_c = order.iter().position(|&id| id == c).expect("c in queue");

        assert!(pos_a < pos_b, "a must come before b: {pos_a} < {pos_b}");
        assert!(pos_b < pos_c, "b must come before c: {pos_b} < {pos_c}");
    }

    #[test]
    fn given_diamond_dependency_when_building_queue_then_converging_nodes_preserve_ordering() {
        //     A
        //    / \
        //   B   C
        //    \ /
        //     D
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let b = workflow.add_node("run", 10.0, 0.0);
        let c = workflow.add_node("run", 10.0, 100.0);
        let d = workflow.add_node("run", 20.0, 0.0);

        add_connection(&mut workflow, a, b);
        add_connection(&mut workflow, a, c);
        add_connection(&mut workflow, b, d);
        add_connection(&mut workflow, c, d);

        let queue = prepare_and_get_queue(&mut workflow);
        let order = queue.expect("diamond should produce a valid queue");

        let pos_a = order.iter().position(|&id| id == a).expect("a");
        let pos_b = order.iter().position(|&id| id == b).expect("b");
        let pos_c = order.iter().position(|&id| id == c).expect("c");
        let pos_d = order.iter().position(|&id| id == d).expect("d");

        // A must come before B and C; B and C must come before D
        assert!(pos_a < pos_b, "a before b");
        assert!(pos_a < pos_c, "a before c");
        assert!(pos_b < pos_d, "b before d");
        assert!(pos_c < pos_d, "c before d");
    }

    #[test]
    fn given_cycle_when_building_queue_then_cycle_detected_error_is_returned() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let b = workflow.add_node("run", 10.0, 0.0);

        // A -> B -> A  (cycle)
        add_connection(&mut workflow, a, b);
        add_connection(&mut workflow, b, a);

        let result = prepare_and_get_queue(&mut workflow);
        assert!(
            matches!(result, Err(WorkflowExecutionError::CycleDetected { .. })),
            "expected CycleDetected, got {result:?}"
        );
    }

    #[test]
    fn given_self_loop_when_preparing_run_then_cycle_detected_error_is_returned() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        add_connection(&mut workflow, a, a);

        let result = prepare_and_get_queue(&mut workflow);
        assert!(
            matches!(result, Err(WorkflowExecutionError::CycleDetected { .. })),
            "self-loop should be detected as cycle, got {result:?}"
        );
    }

    #[test]
    fn given_empty_workflow_when_preparing_run_then_empty_workflow_error_is_returned() {
        let mut workflow = Workflow::new();
        let result = workflow.prepare_run();

        assert_eq!(result, Err(WorkflowExecutionError::EmptyWorkflow));
    }

    #[test]
    fn given_single_node_when_building_queue_then_queue_contains_that_node() {
        let mut workflow = Workflow::new();
        let node = workflow.add_node("run", 0.0, 0.0);

        let queue = prepare_and_get_queue(&mut workflow);
        let order = queue.expect("single node should succeed");

        assert_eq!(order.len(), 1);
        assert_eq!(order[0], node);
    }

    #[test]
    fn given_multiple_roots_when_building_queue_then_all_roots_appear_before_dependents() {
        // Two independent roots feeding into a shared sink.
        //   A  B
        //    \ /
        //     C
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let b = workflow.add_node("run", 10.0, 0.0);
        let c = workflow.add_node("run", 20.0, 0.0);

        add_connection(&mut workflow, a, c);
        add_connection(&mut workflow, b, c);

        // Directly call build_execution_queue (bypasses connectivity check
        // since A and B are both roots with no path between them).
        let order = workflow
            .build_execution_queue()
            .expect("multiple roots should succeed");

        let pos_a = order.iter().position(|&id| id == a).expect("a");
        let pos_b = order.iter().position(|&id| id == b).expect("b");
        let pos_c = order.iter().position(|&id| id == c).expect("c");

        assert!(pos_a < pos_c, "a before c");
        assert!(pos_b < pos_c, "b before c");
        assert_eq!(order.len(), 3);
    }

    // ---------------------------------------------------------------------------
    // build_execution_queue — priority ordering (x coordinate, then name)
    // ---------------------------------------------------------------------------

    #[test]
    fn given_two_root_nodes_when_building_queue_then_lower_x_comes_first() {
        let mut workflow = Workflow::new();
        let right = workflow.add_node("run", 100.0, 0.0);
        let left = workflow.add_node("run", 10.0, 0.0);
        // No connections — both are roots, so ordering is by priority.
        // Use build_execution_queue directly (bypasses connectivity check).

        let order = workflow
            .build_execution_queue()
            .expect("two roots should succeed");

        let pos_left = order.iter().position(|&id| id == left).expect("left");
        let pos_right = order.iter().position(|&id| id == right).expect("right");

        assert!(
            pos_left < pos_right,
            "node with lower x should come first: left@10 vs right@100"
        );
    }

    // ---------------------------------------------------------------------------
    // verify_graph_connectivity
    // ---------------------------------------------------------------------------

    #[test]
    fn given_isolated_subgraph_when_preparing_run_then_connectivity_error_is_returned() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let b = workflow.add_node("run", 10.0, 0.0);
        // Node C is isolated — no connection to/from A or B
        let _c = workflow.add_node("run", 20.0, 0.0);

        add_connection(&mut workflow, a, b);

        let result = workflow.prepare_run();
        assert!(
            matches!(result, Err(WorkflowExecutionError::InvalidWorkflowState { .. })),
            "isolated node should cause connectivity violation, got {result:?}"
        );
    }

    // ---------------------------------------------------------------------------
    // validate_dependencies_exist
    // ---------------------------------------------------------------------------

    #[test]
    fn given_connection_to_nonexistent_target_when_preparing_run_then_unresolved_deps_error_is_returned(
    ) {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let ghost = NodeId::new();

        // Directly inject a connection referencing a non-existent node
        workflow.connections.push(Connection {
            id: Uuid::new_v4(),
            source: a,
            target: ghost,
            source_port: main_port(),
            target_port: main_port(),
        });

        let result = workflow.prepare_run();
        assert!(
            matches!(
                result,
                Err(WorkflowExecutionError::UnresolvedDependencies { .. })
            ),
            "connection to ghost node should fail, got {result:?}"
        );
    }

    // ---------------------------------------------------------------------------
    // check_duplicate_connections
    // ---------------------------------------------------------------------------

    #[test]
    fn given_duplicate_connections_when_preparing_run_then_invalid_state_error_is_returned() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let b = workflow.add_node("run", 10.0, 0.0);

        // Add two identical connections (same source -> target)
        add_connection(&mut workflow, a, b);
        add_connection(&mut workflow, a, b);

        let result = workflow.prepare_run();
        assert!(
            matches!(result, Err(WorkflowExecutionError::InvalidWorkflowState { .. })),
            "duplicate connections should be rejected, got {result:?}"
        );
    }

    // ---------------------------------------------------------------------------
    // check_dirty_state
    // ---------------------------------------------------------------------------

    #[test]
    fn given_nonempty_execution_queue_when_preparing_run_then_dirty_state_error_is_returned() {
        let mut workflow = Workflow::new();
        workflow.add_node("run", 0.0, 0.0);
        workflow.execution_queue.push(NodeId::new());

        let result = workflow.prepare_run();
        assert!(
            matches!(result, Err(WorkflowExecutionError::InvalidWorkflowState { .. })),
            "dirty execution_queue should be rejected, got {result:?}"
        );
    }

    // ---------------------------------------------------------------------------
    // compare_execution_priority — edge cases
    // ---------------------------------------------------------------------------

    #[test]
    fn given_unknown_node_ids_when_comparing_priority_then_ordering_is_equal() {
        let node_map: HashMap<NodeId, &super::super::Node> = HashMap::new();
        let a = NodeId::new();
        let b = NodeId::new();

        let ordering = Workflow::compare_execution_priority(&node_map, a, b);
        assert_eq!(ordering, std::cmp::Ordering::Equal);
    }

    #[test]
    fn given_same_x_different_y_when_comparing_priority_then_lower_y_comes_first() {
        let mut workflow = Workflow::new();
        let upper = workflow.add_node("run", 50.0, 10.0);
        let lower = workflow.add_node("run", 50.0, 90.0);

        let node_map: HashMap<NodeId, &super::super::Node> =
            workflow.nodes.iter().map(|n| (n.id, n)).collect();

        let ordering = Workflow::compare_execution_priority(&node_map, upper, lower);
        assert_eq!(ordering, std::cmp::Ordering::Less);
    }

    // ---------------------------------------------------------------------------
    // find_cycle — three-node cycle
    // ---------------------------------------------------------------------------

    #[test]
    fn given_three_node_cycle_when_preparing_run_then_cycle_detected_error_is_returned() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let b = workflow.add_node("run", 10.0, 0.0);
        let c = workflow.add_node("run", 20.0, 0.0);

        add_connection(&mut workflow, a, b);
        add_connection(&mut workflow, b, c);
        add_connection(&mut workflow, c, a);

        let result = workflow.prepare_run();
        assert!(
            matches!(result, Err(WorkflowExecutionError::CycleDetected { .. })),
            "three-node cycle should be detected, got {result:?}"
        );
    }

    // ---------------------------------------------------------------------------
    // collect_descendants
    // ---------------------------------------------------------------------------

    #[test]
    fn given_branching_graph_when_collecting_descendants_then_all_downstream_nodes_are_found() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let b = workflow.add_node("run", 10.0, 0.0);
        let c = workflow.add_node("run", 10.0, 100.0);
        let d = workflow.add_node("run", 20.0, 0.0);

        add_connection(&mut workflow, a, b);
        add_connection(&mut workflow, a, c);
        add_connection(&mut workflow, b, d);

        let descendants = workflow.collect_descendants(&[a]);

        assert!(descendants.contains(&a), "should include start node");
        assert!(descendants.contains(&b), "should include b");
        assert!(descendants.contains(&c), "should include c");
        assert!(descendants.contains(&d), "should include d");
        assert_eq!(descendants.len(), 4);
    }

    #[test]
    fn given_no_connections_when_collecting_descendants_then_only_start_ids_returned() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let _b = workflow.add_node("run", 10.0, 0.0);

        let descendants = workflow.collect_descendants(&[a]);
        assert_eq!(descendants.len(), 1);
        assert!(descendants.contains(&a));
    }

    // ---------------------------------------------------------------------------
    // Larger graph stress test
    // ---------------------------------------------------------------------------

    #[test]
    fn given_wide_dag_when_building_queue_then_topological_order_is_valid() {
        // Build a 5-layer wide DAG where each layer fans out.
        // Layer 0: 1 node
        // Layer 1: 3 nodes
        // Layer 2: 5 nodes
        // Layer 3: 3 nodes
        // Layer 4: 1 node
        let mut workflow = Workflow::new();

        let l0 = workflow.add_node("run", 0.0, 0.0);

        let l1a = workflow.add_node("run", 10.0, 0.0);
        let l1b = workflow.add_node("run", 10.0, 100.0);
        let l1c = workflow.add_node("run", 10.0, 200.0);

        let l2a = workflow.add_node("run", 20.0, 0.0);
        let l2b = workflow.add_node("run", 20.0, 50.0);
        let l2c = workflow.add_node("run", 20.0, 100.0);
        let l2d = workflow.add_node("run", 20.0, 150.0);
        let l2e = workflow.add_node("run", 20.0, 200.0);

        let l3a = workflow.add_node("run", 30.0, 50.0);
        let l3b = workflow.add_node("run", 30.0, 150.0);
        let l3c = workflow.add_node("run", 30.0, 200.0);

        let l4 = workflow.add_node("run", 40.0, 100.0);

        // Fan out from l0 to l1
        add_connection(&mut workflow, l0, l1a);
        add_connection(&mut workflow, l0, l1b);
        add_connection(&mut workflow, l0, l1c);

        // l1 to l2
        add_connection(&mut workflow, l1a, l2a);
        add_connection(&mut workflow, l1a, l2b);
        add_connection(&mut workflow, l1b, l2c);
        add_connection(&mut workflow, l1c, l2d);
        add_connection(&mut workflow, l1c, l2e);

        // l2 to l3
        add_connection(&mut workflow, l2a, l3a);
        add_connection(&mut workflow, l2b, l3a);
        add_connection(&mut workflow, l2c, l3b);
        add_connection(&mut workflow, l2d, l3b);
        add_connection(&mut workflow, l2e, l3c);

        // l3 to l4
        add_connection(&mut workflow, l3a, l4);
        add_connection(&mut workflow, l3b, l4);
        add_connection(&mut workflow, l3c, l4);

        let queue = prepare_and_get_queue(&mut workflow);
        let order = queue.expect("wide DAG should succeed");
        assert_eq!(order.len(), 13);

        // Verify all edges respect the topological order
        let positions: HashMap<NodeId, usize> =
            order.iter().enumerate().map(|(i, &id)| (id, i)).collect();

        for conn in &workflow.connections {
            let src_pos = positions
                .get(&conn.source)
                .copied()
                .unwrap_or(usize::MAX);
            let tgt_pos = positions
                .get(&conn.target)
                .copied()
                .unwrap_or(usize::MAX);
            assert!(
                src_pos < tgt_pos,
                "edge {:?} -> {:?}: source pos {} must be < target pos {}",
                conn.source,
                conn.target,
                src_pos,
                tgt_pos
            );
        }
    }
}
