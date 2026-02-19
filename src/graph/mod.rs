#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod expressions;
use expressions::ExpressionContext;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NodeId(pub Uuid);

impl NodeId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PortName(pub String);

impl<S: Into<String>> From<S> for PortName {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub node_type: String,
    pub x: f32,
    pub y: f32,
    pub config: serde_json::Value,
    pub last_output: Option<serde_json::Value>,
    pub selected: bool,
    pub executing: bool,
    pub skipped: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Connection {
    pub id: Uuid,
    pub source: NodeId,
    pub target: NodeId,
    pub source_port: PortName,
    pub target_port: PortName,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunRecord {
    pub id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub results: std::collections::HashMap<NodeId, serde_json::Value>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Workflow {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub viewport: Viewport,
    pub execution_queue: Vec<NodeId>,
    pub current_step: usize,
    pub history: Vec<RunRecord>,
}

impl Default for Workflow {
    fn default() -> Self {
        Self::new()
    }
}

impl Workflow {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            viewport: Viewport {
                x: 0.0,
                y: 0.0,
                zoom: 1.0,
            },
            execution_queue: Vec::new(),
            current_step: 0,
            history: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_type: &str, x: f32, y: f32) -> NodeId {
        let id = NodeId::new();
        let name = format!("{node_type} {}", self.nodes.len() + 1);
        self.nodes.push(Node {
            id,
            name,
            node_type: node_type.to_string(),
            x,
            y,
            config: serde_json::json!({}),
            last_output: None,
            selected: false,
            executing: false,
            skipped: false,
            error: None,
        });
        id
    }

    pub fn add_connection(
        &mut self,
        source: NodeId,
        target: NodeId,
        source_port: &PortName,
        target_port: &PortName,
    ) -> bool {
        if source == target {
            return false;
        }
        if self.connections.iter().any(|c| {
            c.source == source
                && c.target == target
                && c.source_port == *source_port
                && c.target_port == *target_port
        }) {
            return false;
        }
        self.connections.push(Connection {
            id: Uuid::new_v4(),
            source,
            target,
            source_port: source_port.clone(),
            target_port: target_port.clone(),
        });
        true
    }

    pub fn update_node_position(&mut self, id: NodeId, dx: f32, dy: f32) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
            node.x += dx;
            node.y += dy;
        }
    }

    pub fn select_node(&mut self, id: NodeId, multi: bool) {
        if !multi {
            for n in &mut self.nodes {
                n.selected = false;
            }
        }
        if let Some(n) = self.nodes.iter_mut().find(|n| n.id == id) {
            n.selected = true;
        }
    }

    pub fn deselect_all(&mut self) {
        for n in &mut self.nodes {
            n.selected = false;
        }
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.retain(|n| n.id != id);
        self.connections
            .retain(|c| c.source != id && c.target != id);
    }

    pub fn zoom(&mut self, delta: f32, cx: f32, cy: f32) {
        let old_zoom = self.viewport.zoom;
        let new_zoom = (old_zoom * (1.0 + delta)).clamp(0.1, 5.0);
        let factor = new_zoom / old_zoom;
        self.viewport.x = (cx - self.viewport.x).mul_add(-factor, cx);
        self.viewport.y = (cy - self.viewport.y).mul_add(-factor, cy);
        self.viewport.zoom = new_zoom;
    }

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

        for conn in &self.connections {
            if let Some(count) = in_degree.get_mut(&conn.target) {
                *count += 1;
            }
        }

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
            for conn in self.connections.iter().filter(|c| c.source == id) {
                if let Some(count) = in_degree.get_mut(&conn.target) {
                    *count -= 1;
                    if *count == 0 {
                        available.push(conn.target);
                    }
                }
            }
        }

        if queue.len() < self.nodes.len() {
            for node in &mut self.nodes {
                if !queue.contains(&node.id) {
                    node.error = Some("Cyclic dependency detected".to_string());
                }
            }
        }

        self.execution_queue = queue;
        self.current_step = 0;
        for node in &mut self.nodes {
            node.executing = false;
            node.last_output = None;
            node.skipped = false;
            if node
                .error
                .as_ref()
                .is_none_or(|e| e != "Cyclic dependency detected")
            {
                node.error = None;
            }
        }
    }

    async fn execute_node_type(
        &self,
        node_type: &str,
        resolved_config: &serde_json::Value,
        parent_outputs: &[serde_json::Value],
    ) -> serde_json::Value {
        match node_type {
            "Trigger" | "Schedule" => serde_json::json!({
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": node_type
            }),
            "JSON Transform" => serde_json::json!({
                "transformed": true,
                "data": parent_outputs,
                "config": resolved_config
            }),
            "Function" => resolved_config
                .get("mapping")
                .cloned()
                .unwrap_or_else(|| serde_json::json!({})),
            "HTTP Request" => self.execute_http_request(resolved_config).await,
            "If" => {
                let condition = resolved_config
                    .get("condition")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("false");
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
            .unwrap_or("https://httpbin.org/get");
        let method = config
            .get("method")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("GET");

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

    pub async fn step(&mut self) -> bool {
        if self.current_step >= self.execution_queue.len() {
            for node in &mut self.nodes {
                node.executing = false;
            }
            return false;
        }

        for node in &mut self.nodes {
            node.executing = false;
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
            self.current_step += 1;
            return true;
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

            if node.node_type == "If" {
                let result = output
                    .get("result")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
                let skip_port = if result { "false" } else { "true" };

                let targets_to_skip: Vec<NodeId> = self
                    .connections
                    .iter()
                    .filter(|c| c.source == node_id && c.source_port.0 == skip_port)
                    .map(|c| c.target)
                    .collect();

                for target_id in targets_to_skip {
                    if let Some(target_node) = self.nodes.iter_mut().find(|n| n.id == target_id) {
                        target_node.skipped = true;
                    }
                }
            }

            if let Some(n) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                if let Some(err) = output.get("error").and_then(serde_json::Value::as_str) {
                    n.error = Some(err.to_string());
                }
                n.executing = true;
                n.last_output = Some(output);
            }
        }

        self.current_step += 1;
        true
    }

    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.current_step < self.execution_queue.len()
    }

    pub fn reset_execution(&mut self) {
        self.current_step = 0;
        for node in &mut self.nodes {
            node.executing = false;
            node.last_output = None;
            node.skipped = false;
            node.error = None;
        }
    }

    pub async fn run(&mut self) {
        self.prepare_run();
        let start_time = chrono::Utc::now();
        let timeout_duration = chrono::Duration::seconds(30);
        let mut results = std::collections::HashMap::new();

        while self.step().await {
            let node_id = self.execution_queue[self.current_step - 1];
            if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
                if let Some(out) = &node.last_output {
                    let _ = results.insert(node_id, out.clone());
                }
            }

            if chrono::Utc::now() - start_time > timeout_duration {
                for node_id in &self.execution_queue[self.current_step..] {
                    if let Some(node) = self.nodes.iter_mut().find(|n| n.id == *node_id) {
                        node.error = Some("Execution timed out".to_string());
                    }
                }
                break;
            }
        }

        let success = self.nodes.iter().all(|n| n.error.is_none());
        self.history.push(RunRecord {
            id: Uuid::new_v4(),
            timestamp: start_time,
            results,
            success,
        });

        if self.history.len() > 10 {
            let _ = self.history.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_new() {
        let workflow = Workflow::new();
        assert!(workflow.nodes.is_empty());
        assert!(workflow.connections.is_empty());
        assert!((workflow.viewport.zoom - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_resolve_expressions_math() {
        let workflow = Workflow::new();
        let config = serde_json::json!("{{ 10 + 20 }}");
        let resolved = workflow.resolve_expressions(&config);
        assert_eq!(resolved, serde_json::json!(30.0));
    }

    #[test]
    fn test_resolve_expressions_string_methods() {
        let workflow = Workflow::new();
        let config = serde_json::json!("{{ \"hello\".to_uppercase() }}");
        let resolved = workflow.resolve_expressions(&config);
        assert_eq!(resolved, serde_json::json!("HELLO"));
    }
}
