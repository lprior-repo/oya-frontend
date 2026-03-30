//! Service call implementations.

use crate::graph::Workflow;

impl Workflow {
    pub(super) async fn execute_service_call_internal(
        &self,
        node_type: &str,
        config: &serde_json::Value,
    ) -> serde_json::Value {
        let str_val = |key: &str| -> &str {
            config
                .get(key)
                .and_then(serde_json::Value::as_str)
                .unwrap_or("")
        };

        let base = &self.restate_ingress_url;
        let url = match node_type {
            "service-call" => {
                let service = str_val("service");
                let endpoint = str_val("endpoint");
                if service.is_empty() || endpoint.is_empty() {
                    return serde_json::json!({
                        "error": "service-call requires 'service' and 'endpoint' config"
                    });
                }
                format!("{base}/{service}/{endpoint}")
            }
            "object-call" => {
                let object = str_val("object_name");
                let handler = str_val("handler");
                let key = {
                    let k = str_val("key");
                    if k.is_empty() {
                        "default"
                    } else {
                        k
                    }
                };
                if object.is_empty() || handler.is_empty() {
                    return serde_json::json!({
                        "error": "object-call requires 'object_name' and 'handler' config"
                    });
                }
                format!("{base}/{object}/{key}/{handler}")
            }
            "workflow-call" => {
                let workflow = str_val("workflow_name");
                if workflow.is_empty() {
                    return serde_json::json!({
                        "error": "workflow-call requires 'workflow_name' config"
                    });
                }
                let id = uuid::Uuid::new_v4().to_string();
                format!("{base}/{workflow}/{id}/run")
            }
            _ => return serde_json::json!({ "executed": true }),
        };

        let payload = config
            .get("payload")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));

        let client = reqwest::Client::new();
        match client.post(&url).json(&payload).send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                match resp.json::<serde_json::Value>().await {
                    Ok(body) => {
                        let inv_id = body
                            .get("id")
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_string);
                        serde_json::json!({
                            "status": status,
                            "restate_invocation_id": inv_id,
                            "body": body
                        })
                    }
                    Err(err) => serde_json::json!({ "status": status, "error": err.to_string() }),
                }
            }
            Err(err) => serde_json::json!({ "error": err.to_string() }),
        }
    }
}
