//! Workflow runner.

use crate::graph::{ExecutionState, NodeCategory, RunRecord, Workflow};

impl Workflow {
    // ===========================================================================
    // Workflow Runner
    // ===========================================================================

    pub async fn run(&mut self) {
        let _ = self.prepare_run();
        let start_time = chrono::Utc::now();
        let mut results = std::collections::HashMap::new();

        if self.nodes.is_empty()
            || !self
                .nodes
                .iter()
                .any(|node| node.category == NodeCategory::Entry)
        {
            self.history.push(RunRecord {
                id: uuid::Uuid::new_v4(),
                timestamp: start_time,
                results,
                success: false,
                restate_invocation_id: None,
            });
            if self.history.len() > 10 {
                let _ = self.history.remove(0);
            }
            return;
        }

        while !self.execution_failed && self.step().await {
            if let Some(id) = self
                .execution_queue
                .get(self.current_step.saturating_sub(1))
            {
                if let Some(node) = self.nodes.iter().find(|n| n.id == *id) {
                    if let Some(out) = &node.last_output {
                        let _ = results.insert(*id, out.clone());
                    }
                }
            }
        }

        let success = self.nodes.iter().all(|node| {
            if node.error.is_some() {
                return false;
            }

            matches!(
                node.execution_state,
                ExecutionState::Completed | ExecutionState::Skipped
            )
        });

        // Capture the first Restate invocation ID produced by a service/object/workflow-call node.
        let restate_invocation_id = self
            .nodes
            .iter()
            .filter_map(|n| n.last_output.as_ref())
            .find_map(|output| {
                output
                    .get("restate_invocation_id")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_string)
            });

        self.history.push(RunRecord {
            id: uuid::Uuid::new_v4(),
            timestamp: start_time,
            results,
            success,
            restate_invocation_id,
        });

        if self.history.len() > 10 {
            let _ = self.history.remove(0);
        }
    }
}
