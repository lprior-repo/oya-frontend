#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

use super::execution_types::ExecutionConfig;
use super::expressions::ExpressionContext;
use super::WorkflowExecutionError;
use super::{NodeId, Workflow};

// ===========================================================================
// Execution Plan Preparation (Topological Sort)
// ===========================================================================

impl Workflow {
    fn compare_execution_priority(&self, a: NodeId, b: NodeId) -> std::cmp::Ordering {
        let node_a = self.nodes.iter().find(|node| node.id == a);
        let node_b = self.nodes.iter().find(|node| node.id == b);

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
    fn verify_graph_connectivity(&self) -> Result<(), WorkflowExecutionError> {
        if self.nodes.is_empty() {
            return Ok(());
        }

        // Build adjacency list for traversal
        let node_ids: std::collections::HashSet<NodeId> = self.nodes.iter().map(|n| n.id).collect();

        // Build outgoing edges map (node -> nodes it depends on)
        let mut outgoing: std::collections::HashMap<NodeId, Vec<NodeId>> =
            std::collections::HashMap::new();

        for node in &self.nodes {
            let targets: Vec<NodeId> = self
                .connections
                .iter()
                .filter(|conn| conn.source == node.id && node_ids.contains(&conn.target))
                .map(|conn| conn.target)
                .collect();
            outgoing.insert(node.id, targets);
        }

        // Find a starting node (any node with no incoming edges, or first node)
        let mut entry_node = None;
        for node in &self.nodes {
            let has_incoming: bool = self
                .connections
                .iter()
                .any(|conn| conn.target == node.id && node_ids.contains(&conn.source));
            if !has_incoming {
                entry_node = Some(node.id);
                break;
            }
        }

        // If no entry node found, use first node (this should never happen after check_non_empty)
        let start_node = if let Some(node) = entry_node {
            node
        } else if let Some(first_node) = self.nodes.first() {
            first_node.id
        } else {
            // This should never happen after check_non_empty() passes
            // Return InvalidWorkflowState error instead of panicking
            return Err(WorkflowExecutionError::InvalidWorkflowState {
                reason: "verify_graph_connectivity: graph should have at least one node"
                    .to_string(),
            });
        };

        // BFS/DFS to find all reachable nodes
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![start_node];

        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            if let Some(targets) = outgoing.get(&current) {
                for target in targets {
                    if !visited.contains(target) {
                        stack.push(*target);
                    }
                }
            }
        }

        // Check if all nodes are reachable
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
        let node_ids: std::collections::HashSet<NodeId> = self.nodes.iter().map(|n| n.id).collect();

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
                    .or_insert_with(Vec::new)
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
            seen.entry(key).or_insert_with(Vec::new).push(conn.id);
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
    fn check_non_empty(&self) -> Result<(), WorkflowExecutionError> {
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
    fn build_execution_queue(&self) -> Result<Vec<NodeId>, WorkflowExecutionError> {
        let mut queue = Vec::new();
        let node_ids: std::collections::HashSet<NodeId> = self.nodes.iter().map(|n| n.id).collect();

        let mut in_degree: std::collections::HashMap<NodeId, usize> =
            self.nodes.iter().map(|n| (n.id, 0)).collect();

        // Calculate in-degrees from connections
        for conn in &self.connections {
            if node_ids.contains(&conn.source) && node_ids.contains(&conn.target) {
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

        available.sort_by(|a, b| self.compare_execution_priority(*a, *b));

        // Process nodes in topological order
        while !available.is_empty() {
            let id = available.remove(0);
            queue.push(id);

            // Find nodes that depend on the current node
            let mut new_available: Vec<NodeId> = Vec::new();
            for conn in &self.connections {
                if conn.source == id
                    && node_ids.contains(&conn.source)
                    && node_ids.contains(&conn.target)
                {
                    if let Some(count) = in_degree.get_mut(&conn.target) {
                        *count -= 1;
                        if *count == 0 {
                            new_available.push(conn.target);
                        }
                    }
                }
            }

            available.extend(new_available);
            available.sort_by(|a, b| self.compare_execution_priority(*a, *b));
        }

        // Verify all nodes were processed (no cycles)
        if queue.len() != self.nodes.len() {
            let remaining: Vec<NodeId> = node_ids
                .difference(&queue.iter().copied().collect())
                .copied()
                .collect();

            return Err(WorkflowExecutionError::CycleDetected {
                cycle_nodes: remaining,
            });
        }

        Ok(queue)
    }

    /// DFS-based cycle detection that returns actual cycle nodes.
    fn detect_cycle_dfs(
        &self,
        start_node: NodeId,
        visited: &mut std::collections::HashSet<NodeId>,
        path_set: &mut std::collections::HashSet<NodeId>,
        path: &mut Vec<NodeId>,
    ) -> Option<Vec<NodeId>> {
        visited.insert(start_node);
        path.push(start_node);
        path_set.insert(start_node);

        // Find all nodes that depend on start_node (reverse edges)
        let dependents: Vec<NodeId> = self
            .connections
            .iter()
            .filter(|conn| conn.target == start_node)
            .map(|conn| conn.source)
            .collect();

        for dependent in dependents {
            // Check if dependent is in current path (back edge = cycle)
            if path_set.contains(&dependent) {
                // Found a cycle - extract it
                let cycle_start = path.iter().position(|&n| n == dependent)?;
                let cycle: Vec<NodeId> = path[cycle_start..].to_vec();
                return Some(cycle);
            }
            // Recurse if not yet visited
            if !visited.contains(&dependent) {
                if let Some(cycle) = self.detect_cycle_dfs(dependent, visited, path_set, path) {
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
    fn find_cycle(&self) -> Option<Vec<NodeId>> {
        let mut visited = std::collections::HashSet::new();
        let mut path = Vec::new();
        let mut path_set = std::collections::HashSet::new();

        // Try starting from each node
        for node in &self.nodes {
            if !visited.contains(&node.id) {
                if let Some(cycle) =
                    self.detect_cycle_dfs(node.id, &mut visited, &mut path_set, &mut path)
                {
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
            _ => config.clone(),
        }
    }
}
