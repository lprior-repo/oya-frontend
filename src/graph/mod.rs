#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

pub mod expressions;
pub mod layout;

use expressions::ExpressionContext;
use layout::DagLayout;

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

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PortName(pub String);

impl<S: Into<String>> From<S> for PortName {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NodeCategory {
    Entry,
    Durable,
    State,
    Flow,
    Timing,
    Signal,
}

impl fmt::Display for NodeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Entry => "entry",
            Self::Durable => "durable",
            Self::State => "state",
            Self::Flow => "flow",
            Self::Timing => "timing",
            Self::Signal => "signal",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub description: String,
    pub node_type: String,
    pub category: NodeCategory,
    pub icon: String,
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
    fn set_node_status(node: &mut Node, status: &str) {
        if let Some(obj) = node.config.as_object_mut() {
            obj.insert("status".to_string(), serde_json::Value::String(status.to_string()));
        } else {
            node.config = serde_json::json!({ "status": status });
        }
    }

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

    #[allow(clippy::too_many_lines)]
    fn get_node_metadata(node_type: &str) -> (NodeCategory, String, String) {
        match node_type {
            "http-handler" => (
                NodeCategory::Entry,
                "HTTP Handler".to_string(),
                "globe".to_string(),
            ),
            "kafka-handler" => (
                NodeCategory::Entry,
                "Kafka Consumer".to_string(),
                "kafka".to_string(),
            ),
            "cron-trigger" => (
                NodeCategory::Entry,
                "Cron Trigger".to_string(),
                "clock".to_string(),
            ),
            "workflow-submit" => (
                NodeCategory::Entry,
                "Workflow Submit".to_string(),
                "play-circle".to_string(),
            ),
            "run" => (
                NodeCategory::Durable,
                "Durable Step".to_string(),
                "shield".to_string(),
            ),
            "service-call" => (
                NodeCategory::Durable,
                "Service Call".to_string(),
                "arrow-right".to_string(),
            ),
            "object-call" => (
                NodeCategory::Durable,
                "Object Call".to_string(),
                "box".to_string(),
            ),
            "workflow-call" => (
                NodeCategory::Durable,
                "Workflow Call".to_string(),
                "workflow".to_string(),
            ),
            "send-message" => (
                NodeCategory::Durable,
                "Send Message".to_string(),
                "send".to_string(),
            ),
            "delayed-send" => (
                NodeCategory::Durable,
                "Delayed Message".to_string(),
                "clock-send".to_string(),
            ),
            "get-state" => (
                NodeCategory::State,
                "Get State".to_string(),
                "download".to_string(),
            ),
            "set-state" => (
                NodeCategory::State,
                "Set State".to_string(),
                "upload".to_string(),
            ),
            "clear-state" => (
                NodeCategory::State,
                "Clear State".to_string(),
                "eraser".to_string(),
            ),
            "condition" => (
                NodeCategory::Flow,
                "If / Else".to_string(),
                "git-branch".to_string(),
            ),
            "switch" => (
                NodeCategory::Flow,
                "Switch".to_string(),
                "git-fork".to_string(),
            ),
            "loop" => (
                NodeCategory::Flow,
                "Loop / Iterate".to_string(),
                "repeat".to_string(),
            ),
            "parallel" => (
                NodeCategory::Flow,
                "Parallel".to_string(),
                "layers".to_string(),
            ),
            "compensate" => (
                NodeCategory::Flow,
                "Compensate".to_string(),
                "undo".to_string(),
            ),
            "sleep" => (
                NodeCategory::Timing,
                "Sleep / Timer".to_string(),
                "timer".to_string(),
            ),
            "timeout" => (
                NodeCategory::Timing,
                "Timeout".to_string(),
                "alarm".to_string(),
            ),
            "durable-promise" => (
                NodeCategory::Signal,
                "Durable Promise".to_string(),
                "sparkles".to_string(),
            ),
            "awakeable" => (
                NodeCategory::Signal,
                "Awakeable".to_string(),
                "bell".to_string(),
            ),
            "resolve-promise" => (
                NodeCategory::Signal,
                "Resolve Promise".to_string(),
                "check-circle".to_string(),
            ),
            "signal-handler" => (
                NodeCategory::Signal,
                "Signal Handler".to_string(),
                "radio".to_string(),
            ),
            _ => (
                NodeCategory::Durable,
                "Unknown Node".to_string(),
                "help-circle".to_string(),
            ),
        }
    }

    pub fn add_node(&mut self, node_type: &str, x: f32, y: f32) -> NodeId {
        let mut final_x = x;
        let mut final_y = y;

        while self
            .nodes
            .iter()
            .any(|n| (n.x - final_x).abs() < 10.0 && (n.y - final_y).abs() < 10.0)
        {
            final_x += 30.0;
            final_y += 30.0;
        }

        let id = NodeId::new();
        let name = format!("{node_type} {}", self.nodes.len() + 1);
        let (category, icon, description) = Self::get_node_metadata(node_type);

        self.nodes.push(Node {
            id,
            name,
            description,
            node_type: node_type.to_string(),
            category,
            icon,
            x: final_x,
            y: final_y,
            config: serde_json::json!({}),
            last_output: None,
            selected: false,
            executing: false,
            skipped: false,
            error: None,
        });
        id
    }

    pub fn add_node_at_viewport_center(&mut self, node_type: &str) {
        let vx = self.viewport.x;
        let vy = self.viewport.y;
        let vz = self.viewport.zoom;
        let nx = (400.0 - vx) / vz;
        let ny = (300.0 - vy) / vz;
        self.add_node(node_type, nx, ny);
    }

    pub fn update_node_position(&mut self, id: NodeId, dx: f32, dy: f32) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
            node.x += dx;
            node.y += dy;
            node.x = (node.x / 10.0).round() * 10.0;
            node.y = (node.y / 10.0).round() * 10.0;
        }
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

    pub fn apply_layout(&mut self) {
        let layout = DagLayout::default();
        layout.apply(self);
    }

    pub fn zoom(&mut self, delta: f32, cx: f32, cy: f32) {
        let old_zoom = self.viewport.zoom;
        let new_zoom = (old_zoom * (1.0 + delta)).clamp(0.1, 5.0);
        let factor = new_zoom / old_zoom;
        self.viewport.x = (cx - self.viewport.x).mul_add(-factor, cx);
        self.viewport.y = (cy - self.viewport.y).mul_add(-factor, cy);
        self.viewport.zoom = new_zoom;
    }

    pub fn fit_view(&mut self, viewport_width: f32, viewport_height: f32, padding: f32) {
        let bounds = self
            .nodes
            .iter()
            .fold(None::<(f32, f32, f32, f32)>, |acc, node| {
                let right = node.x + 220.0;
                let bottom = node.y + 68.0;
                match acc {
                    Some((min_x, min_y, max_x, max_y)) => {
                        Some((min_x.min(node.x), min_y.min(node.y), max_x.max(right), max_y.max(bottom)))
                    }
                    None => Some((node.x, node.y, right, bottom)),
                }
            });

        if let Some((min_x, min_y, max_x, max_y)) = bounds {
            let width = max_x - min_x;
            let height = max_y - min_y;
            let scale_x = (viewport_width - padding) / width.max(1.0_f32);
            let scale_y = (viewport_height - padding) / height.max(1.0_f32);
            let zoom = scale_x.min(scale_y).clamp(0.15, 1.5);
            let center_x = f32::midpoint(min_x, max_x);
            let center_y = f32::midpoint(min_y, max_y);

            self.viewport.zoom = zoom;
            self.viewport.x = viewport_width / 2.0 - center_x * zoom;
            self.viewport.y = viewport_height / 2.0 - center_y * zoom;
        }
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

        self.execution_queue = queue;
        self.current_step = 0;
        for node in &mut self.nodes {
            node.executing = false;
            node.last_output = None;
            node.skipped = false;
            node.error = None;
            Self::set_node_status(node, "pending");
        }
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
            for node in &mut self.nodes {
                node.executing = false;
            }
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

                for target_id in targets_to_skip {
                    if let Some(target_node) = self.nodes.iter_mut().find(|n| n.id == target_id) {
                        target_node.skipped = true;
                        Self::set_node_status(target_node, "skipped");
                    }
                }
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
