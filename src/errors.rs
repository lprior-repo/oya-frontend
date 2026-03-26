use thiserror::Error;

use oya_frontend::graph::NodeId;

#[allow(dead_code)]
pub type AppResult<T> = Result<T, AppError>;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Workflow error: {0}")]
    Workflow(#[source] WorkflowError),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Linter error: {0}")]
    Linter(#[source] oya_frontend::linter::LintError),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Scenario runner error: {0}")]
    ScenarioRunner(#[source] oya_frontend::scenario_runner::ScenarioError),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Metrics error: {0}")]
    Metrics(#[source] oya_frontend::metrics::MetricsError),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Coverage error: {0}")]
    Coverage(#[source] oya_frontend::coverage::CoverageError),

    #[error("Feedback error: {0}")]
    Feedback(String),

    #[error("Flow extension error: {0}")]
    FlowExtension(String),

    #[error("IO error: {0}")]
    Io(#[source] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[source] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[source] serde_yaml::Error),

    #[error("HTTP request error: {0}")]
    Http(#[source] reqwest::Error),

    #[error("Lock acquisition failed")]
    LockAcquisition,
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<serde_yaml::Error> for AppError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Yaml(e)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum WorkflowError {
    #[error("Node {0} not found")]
    NodeNotFound(NodeId),

    #[error("Connection would create a cycle")]
    CycleDetected,

    #[error("Connection already exists")]
    DuplicateConnection,

    #[error("Invalid connection: {0}")]
    InvalidConnection(String),

    #[error("Port not found: {0}")]
    #[allow(dead_code)]
    PortNotFound(String),

    #[error("Cannot connect node to itself")]
    SelfConnection,
}

pub type WorkflowResult<T> = Result<T, WorkflowError>;
