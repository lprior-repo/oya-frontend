//! Node execution implementations.

use crate::graph::Workflow;

impl Workflow {
    // ===========================================================================
    // Node Execution Runtime
    // ===========================================================================

    pub(super) async fn execute_node_type(
        &self,
        node_type_str: &str,
        resolved_config: &serde_json::Value,
        parent_outputs: &[serde_json::Value],
    ) -> serde_json::Value {
        match node_type_str {
            "http-handler" | "kafka-handler" | "cron-trigger" => serde_json::json!({
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": node_type_str
            }),
            "http-request" | "http-call" => self.execute_http_request(resolved_config).await,
            "run" => resolved_config
                .get("mapping")
                .cloned()
                .unwrap_or_else(|| resolved_config.clone()),
            "service-call" | "object-call" | "workflow-call" => {
                self.execute_service_call_internal(node_type_str, resolved_config)
                    .await
            }
            "condition" => {
                let condition_value = resolved_config
                    .get("expression")
                    .or_else(|| resolved_config.get("condition"));
                let result = match condition_value {
                    Some(serde_json::Value::Bool(b)) => *b,
                    Some(serde_json::Value::String(s)) => {
                        s == "true" || (!s.is_empty() && s != "false")
                    }
                    _ => false,
                };
                let condition_str = condition_value.and_then(|v| v.as_str()).unwrap_or("");
                serde_json::json!({ "result": result, "condition": condition_str })
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
                let body: serde_json::Value =
                    resp.json().await.unwrap_or_else(|_| serde_json::json!({}));
                serde_json::json!({ "status": status, "url": url, "body": body })
            }
            Err(e) => serde_json::json!({ "error": e.to_string(), "url": url }),
        }
    }
}
