use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ScenarioError {
    #[error("Failed to read scenario file: {0}")]
    ReadError(#[source] std::io::Error),
    
    #[error("Failed to parse YAML: {0}")]
    ParseError(#[source] serde_yaml::Error),
    
    #[error("HTTP request failed: {0}")]
    HttpError(#[source] reqwest::Error),
    
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),
    
    #[error("Setup failed: {0}")]
    SetupFailed(String),
    
    #[error("Invalid action type: {0}")]
    InvalidActionType(String),
    
    #[error("Invalid assertion type: {0}")]
    InvalidAssertionType(String),
    
    #[error("Missing required field '{field}' in {context}")]
    MissingRequiredField { field: String, context: String },
    
    #[error("Invalid scenario state transition: {from} -> {to}")]
    InvalidStateTransition { from: String, to: String },
    
    #[error("Precondition not met: {0}")]
    PreconditionFailed(String),
    
    #[error("Teardown failed: {0}")]
    TeardownFailed(String),
}

pub type ScenarioResult<T> = Result<T, ScenarioError>;
