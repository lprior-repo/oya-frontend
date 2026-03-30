#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

use super::execution_types::ExecutionConfig;
use super::expressions::ExpressionContext;
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

    pub fn prepare_run(&mut self) {
        // Reset memory tracking at the start of execution
        self.current_memory_bytes = 0;
        self.execution_failed = false;

        let mut queue = Vec::new();
        let node_ids: std::collections::HashSet<NodeId> = self.nodes.iter().map(|n| n.id).collect();
        let mut in_degree: std::collections::HashMap<NodeId, usize> =
            self.nodes.iter().map(|n| (n.id, 0)).collect();

        self.connections.iter().for_each(|conn| {
            if node_ids.contains(&conn.source) && node_ids.contains(&conn.target) {
                if let Some(count) = in_degree.get_mut(&conn.target) {
                    *count += 1;
                }
            }
        });

        // Start from all nodes with zero indegree (entry nodes or parallel nodes)
        let mut available: Vec<NodeId> = self
            .nodes
            .iter()
            .filter(|n| in_degree.get(&n.id).is_some_and(|&d| d == 0))
            .map(|n| n.id)
            .collect();

        available.sort_by(|a, b| self.compare_execution_priority(*a, *b));

        while !available.is_empty() {
            let id = available.remove(0);
            queue.push(id);
            let mut new_nodes: Vec<NodeId> = Vec::new();
            for conn in &self.connections {
                if conn.source == id
                    && node_ids.contains(&conn.source)
                    && node_ids.contains(&conn.target)
                {
                    if let Some(count) = in_degree.get_mut(&conn.target) {
                        *count -= 1;
                        if *count == 0 {
                            new_nodes.push(conn.target);
                        }
                    }
                }
            }
            available.extend(new_nodes);
            available.sort_by(|a, b| self.compare_execution_priority(*a, *b));
        }

        self.execution_queue = queue;
        self.current_step = 0;
        for node in &mut self.nodes {
            node.executing = false;
            node.last_output = None;
            node.skipped = false;
            node.error = None;
            let _ = Self::set_node_pending_status(node);
        }
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
    /// use oya_frontend::graph::{Workflow, ExecutionConfig};
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
