use super::expressions::ExpressionContext;
use super::{NodeId, RunRecord, Workflow};

impl Workflow {
    #[must_use]
    pub fn resolve_expressions(&self, config: &serde_json::Value) -> serde_json::Value {
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
                    .map(|(k, v)| (k.clone(), self.resolve_expressions(v)))
                    .collect();
                serde_json::Value::Object(new_map)
            }
            serde_json::Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(|v| self.resolve_expressions(v)).collect())
            }
            _ => config.clone(),
        }
    }

    pub fn prepare_run(&mut self) {
        let mut queue = Vec::new();
        let mut in_degree: std::collections::HashMap<NodeId, usize> =
            self.nodes.iter().map(|n| (n.id, 0)).collect();

        self.connections.iter().for_each(|conn| {
            if let Some(count) = in_degree.get_mut(&conn.target) {
                *count += 1;
            }
        });

        let mut available: Vec<NodeId> = self
            .nodes
            .iter()
            .filter(|n| in_degree.get(&n.id).is_some_and(|&d| d == 0))
            .map(|n| n.id)
            .collect();

        available.sort_by(|a, b| {
            let name_a = self.nodes.iter().find(|n| n.id == *a).map(|n| &n.name);
            let name_b = self.nodes.iter().find(|n| n.id == *b).map(|n| &n.name);
            name_a.cmp(&name_b)
        });

        while let Some(id) = available.pop() {
            queue.push(id);
            self.connections
                .iter()
                .filter(|c| c.source == id)
                .for_each(|conn| {
                    if let Some(count) = in_degree.get_mut(&conn.target) {
                        *count -= 1;
                        if *count == 0 {
                            available.push(conn.target);
                        }
                    }
                });
        }

        self.execution_queue = queue;
        self.current_step = 0;
        self.nodes.iter_mut().for_each(|node| {
            node.executing = false;
            node.last_output = None;
            node.skipped = false;
            node.error = None;
            Self::set_node_status(node, "pending");
        });
    }

    async fn execute_node_type(
        &self,
        node_type: &str,
        resolved_config: &serde_json::Value,
        parent_outputs: &[serde_json::Value],
    ) -> serde_json::Value {
        match node_type {
            "webhook" | "schedule" => serde_json::json!({
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": node_type
            }),
            "transform" => serde_json::json!({
                "transformed": true,
                "data": parent_outputs,
                "config": resolved_config
            }),
            "code" => resolved_config
                .get("mapping")
                .cloned()
                .map_or_else(|| serde_json::json!({}), std::convert::identity),
            "http-request" => self.execute_http_request(resolved_config).await,
            "condition" => {
                let condition = resolved_config
                    .get("condition")
                    .and_then(serde_json::Value::as_str)
                    .map_or("false", |s| s);
                let result = condition == "true" || (!condition.is_empty() && condition != "false");
                serde_json::json!({ "result": result, "condition": condition })
            }
            _ => serde_json::json!({
                "executed": true,
                "step": self.current_step,
                "input_count": parent_outputs.len(),
                "config": resolved_config
            }),
        }
    }

    async fn execute_http_request(&self, config: &serde_json::Value) -> serde_json::Value {
        let url = config
            .get("url")
            .and_then(serde_json::Value::as_str)
            .map_or("https://httpbin.org/get", |s| s);
        let method = config
            .get("method")
            .and_then(serde_json::Value::as_str)
            .map_or("GET", |s| s);

        let client = reqwest::Client::new();
        let rb = match method {
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => client.get(url),
        };

        match rb.send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let body: serde_json::Value = resp
                    .json()
                    .await
                    .map_or_else(|_| serde_json::json!({}), std::convert::identity);
                serde_json::json!({ "status": status, "url": url, "body": body })
            }
            Err(e) => serde_json::json!({ "error": e.to_string(), "url": url }),
        }
    }

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
                Self::set_node_status(node, "skipped");
            }
            self.current_step += 1;
            return true;
        }

        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.executing = true;
            Self::set_node_status(node, "running");
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

        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id).cloned() {
            let resolved_config = self.resolve_expressions(&node.config);
            let output = self
                .execute_node_type(&node.node_type, &resolved_config, &parent_outputs)
                .await;

            if node.node_type == "condition" {
                let result = output
                    .get("result")
                    .and_then(serde_json::Value::as_bool)
                    .is_some_and(|value| value);
                let skip_port = if result { "false" } else { "true" };

                let targets_to_skip: Vec<NodeId> = self
                    .connections
                    .iter()
                    .filter(|c| c.source == node_id && c.source_port.0 == skip_port)
                    .map(|c| c.target)
                    .collect();

                targets_to_skip.iter().for_each(|target_id| {
                    if let Some(target_node) = self.nodes.iter_mut().find(|n| n.id == *target_id) {
                        target_node.skipped = true;
                        Self::set_node_status(target_node, "skipped");
                    }
                });
            }

            if let Some(n) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                if let Some(err) = output.get("error").and_then(serde_json::Value::as_str) {
                    n.error = Some(err.to_string());
                    Self::set_node_status(n, "failed");
                } else {
                    Self::set_node_status(n, "completed");
                }
                n.executing = false;
                n.last_output = Some(output);
            }
        }

        self.current_step += 1;
        true
    }

    pub async fn run(&mut self) {
        self.prepare_run();
        let start_time = chrono::Utc::now();
        let mut results = std::collections::HashMap::new();

        while self.step().await {
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

            node.config
                .get("status")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|status| status == "completed" || status == "skipped")
        });
        self.history.push(RunRecord {
            id: uuid::Uuid::new_v4(),
            timestamp: start_time,
            results,
            success,
        });

        if self.history.len() > 10 {
            let _ = self.history.remove(0);
        }
    }
}
