//! Execution step runner.

use crate::graph::{ExecutionState, NodeId, Workflow};

impl Workflow {
    // ===========================================================================
    // Condition Branch Skipping
    // ===========================================================================

    fn execute_condition_and_skip_branches(&mut self, node_id: NodeId, output: &serde_json::Value) {
        let result = output
            .get("result")
            .and_then(serde_json::Value::as_bool)
            .is_some_and(|value| value);
        let skip_port = if result { "false" } else { "true" };

        let branch_targets: Vec<NodeId> = self
            .connections
            .iter()
            .filter(|c| c.source == node_id && c.source_port.0 == skip_port)
            .map(|c| c.target)
            .collect();

        let branch_descendants = self.collect_descendants(&branch_targets);

        let mut skip_set: std::collections::HashSet<NodeId> = std::collections::HashSet::new();
        skip_set.extend(branch_targets);
        skip_set.extend(branch_descendants);

        for skip_id in &skip_set {
            if let Some(skip_node) = self.nodes.iter_mut().find(|n| n.id == *skip_id) {
                if !skip_node.skipped {
                    skip_node.skipped = true;
                    let _ = Self::set_node_status(skip_node, ExecutionState::Skipped);
                }
            }
        }

        let target_ids: Vec<NodeId> = self.nodes.iter().map(|n| n.id).collect();
        for target_id in target_ids {
            if skip_set.contains(&target_id) {
                continue;
            }
            let incoming: Vec<NodeId> = self
                .connections
                .iter()
                .filter(|c| c.target == target_id)
                .map(|c| c.source)
                .collect();

            if !incoming.is_empty() && incoming.iter().all(|src| skip_set.contains(src)) {
                if let Some(target_node) = self.nodes.iter_mut().find(|n| n.id == target_id) {
                    target_node.skipped = true;
                    let _ = Self::set_node_status(target_node, ExecutionState::Skipped);
                }
            }
        }
    }

    // ===========================================================================
    // Execution Step Runner
    // ===========================================================================

    pub async fn step(&mut self) -> bool {
        if self.current_step >= self.execution_queue.len() {
            self.nodes.iter_mut().for_each(|node| {
                node.executing = false;
            });
            return false;
        }

        let node_id = match self.execution_queue.get(self.current_step) {
            Some(id) => *id,
            None => return false,
        };

        if self
            .nodes
            .iter()
            .find(|n| n.id == node_id)
            .is_some_and(|n| n.skipped)
        {
            if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                let _ = Self::set_node_status(node, ExecutionState::Skipped);
            }
            self.current_step += 1;
            return true;
        }

        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.executing = true;
            let _ = Self::set_node_status(node, ExecutionState::Running);
        }

        let parent_outputs: Vec<serde_json::Value> = self
            .connections
            .iter()
            .filter(|c| c.target == node_id)
            .filter_map(|c| {
                self.nodes
                    .iter()
                    .find(|n| n.id == c.source)
                    .and_then(|n| n.last_output.clone())
            })
            .collect();

        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            let node_type = node.node_type.clone();
            let node_config_json = node.config.clone();
            let resolved_config = self.resolve_expressions(&node_config_json);
            let output = self
                .execute_node_type(&node_type, &resolved_config, &parent_outputs)
                .await;

            if node_type == "condition" {
                self.execute_condition_and_skip_branches(node_id, &output);
            }

            if let Some(n) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                if let Some(err) = output.get("error").and_then(serde_json::Value::as_str) {
                    n.error = Some(err.to_string());
                    let _ = Self::set_node_status(n, ExecutionState::Failed);
                } else {
                    let _ = Self::set_node_status(n, ExecutionState::Completed);
                }
                n.executing = false;
                n.last_output = Some(output);
            }
        }

        self.current_step += 1;
        true
    }
}
