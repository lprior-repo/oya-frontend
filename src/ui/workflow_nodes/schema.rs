use serde::{Deserialize, Serialize};

fn deserialize_empty_to_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

fn default_delay_ms() -> u64 {
    60_000
}

fn default_duration_ms() -> u64 {
    1_000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WorkflowNode {
    HttpTrigger(HttpTriggerConfig),
    ScheduleTrigger(ScheduleTriggerConfig),
    ServiceCall(ServiceCallConfig),
    SendMessage(SendMessageConfig),
    DelayedMessage(DelayedMessageConfig),
    SaveToMemory(SaveToMemoryConfig),
    LoadFromMemory(LoadFromMemoryConfig),
    Delay(DelayConfig),
    Router(RouterConfig),
    WaitForWebhook(WaitForWebhookConfig),
    WaitForSignal(WaitForSignalConfig),
    RunCode(RunCodeConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpTriggerConfig {
    pub path: String,
    pub method: HttpMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleTriggerConfig {
    pub schedule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCallConfig {
    #[serde(default)]
    pub target_type: TargetType,
    pub service_name: String,
    #[serde(default, deserialize_with = "deserialize_empty_to_none")]
    pub key: Option<String>,
    pub handler_name: String,
    pub input: serde_json::Value,
    #[serde(default, deserialize_with = "deserialize_empty_to_none")]
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageConfig {
    pub target_type: TargetType,
    pub service_name: String,
    pub key: Option<String>,
    pub handler_name: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayedMessageConfig {
    pub target_type: TargetType,
    pub service_name: String,
    pub key: Option<String>,
    pub handler_name: String,
    pub input: serde_json::Value,
    #[serde(default = "default_delay_ms")]
    pub delay_ms: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum TargetType {
    #[default]
    Service,
    VirtualObject,
    Workflow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveToMemoryConfig {
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadFromMemoryConfig {
    pub key: String,
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayConfig {
    #[serde(default = "default_duration_ms")]
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    pub branches: Vec<RouterBranch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterBranch {
    #[serde(default = "default_router_branch_id")]
    pub id: String,
    pub name: String,
    pub condition: String,
    pub next_node_id: Option<String>,
}

fn default_router_branch_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

impl RouterBranch {
    pub fn new(name: String) -> Self {
        Self {
            id: default_router_branch_id(),
            name,
            condition: String::new(),
            next_node_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitForWebhookConfig {
    pub awakeable_id: String,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitForSignalConfig {
    pub promise_name: String,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunCodeConfig {
    pub code: String,
    pub language: CodeLanguage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeLanguage {
    JavaScript,
    Python,
    Expression,
}

impl Default for WorkflowNode {
    fn default() -> Self {
        WorkflowNode::HttpTrigger(HttpTriggerConfig {
            path: String::new(),
            method: HttpMethod::POST,
        })
    }
}
