#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Core workflow node types.
//!
//! Contains the `WorkflowNode` enum and related types (`ConditionResult`, `HttpMethod`).
//! Config structs are defined in the `configs` submodule.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use super::NodeCategory;
use crate::graph::restate_types::PortType;

pub mod configs;
pub use configs::*;

// ============================================================================
// WorkflowNode Enum
// ============================================================================

/// The 24 workflow node types in the OYA graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum WorkflowNode {
    HttpHandler(HttpHandlerConfig),
    HttpCall(HttpCallConfig),
    KafkaHandler(KafkaHandlerConfig),
    CronTrigger(CronTriggerConfig),
    WorkflowSubmit(WorkflowSubmitConfig),
    Run(RunConfig),
    ServiceCall(ServiceCallConfig),
    ObjectCall(ObjectCallConfig),
    WorkflowCall(WorkflowCallConfig),
    SendMessage(SendMessageConfig),
    DelayedSend(DelayedSendConfig),
    GetState(GetStateConfig),
    SetState(SetStateConfig),
    ClearState(ClearStateConfig),
    Condition(ConditionConfig),
    Switch(SwitchConfig),
    Loop(LoopConfig),
    Parallel(ParallelConfig),
    Compensate(CompensateConfig),
    Sleep(SleepConfig),
    Timeout(TimeoutConfig),
    DurablePromise(DurablePromiseConfig),
    Awakeable(AwakeableConfig),
    ResolvePromise(ResolvePromiseConfig),
    SignalHandler(SignalHandlerConfig),
}

impl Default for WorkflowNode {
    fn default() -> Self {
        Self::Run(RunConfig::default())
    }
}

// ============================================================================
// FromStr Implementation for WorkflowNode
// ============================================================================

impl FromStr for WorkflowNode {
    type Err = UnknownNodeTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "http-handler" => Ok(Self::HttpHandler(HttpHandlerConfig::default())),
            "http-call" | "http-request" => Ok(Self::HttpCall(HttpCallConfig::default())),
            "kafka-handler" => Ok(Self::KafkaHandler(KafkaHandlerConfig::default())),
            "cron-trigger" => Ok(Self::CronTrigger(CronTriggerConfig::default())),
            "workflow-submit" => Ok(Self::WorkflowSubmit(WorkflowSubmitConfig::default())),
            "run" => Ok(Self::Run(RunConfig::default())),
            "service-call" => Ok(Self::ServiceCall(ServiceCallConfig::default())),
            "object-call" => Ok(Self::ObjectCall(ObjectCallConfig::default())),
            "workflow-call" => Ok(Self::WorkflowCall(WorkflowCallConfig::default())),
            "send-message" => Ok(Self::SendMessage(SendMessageConfig::default())),
            "delayed-send" => Ok(Self::DelayedSend(DelayedSendConfig::default())),
            "get-state" => Ok(Self::GetState(GetStateConfig::default())),
            "set-state" => Ok(Self::SetState(SetStateConfig::default())),
            "clear-state" => Ok(Self::ClearState(ClearStateConfig::default())),
            "condition" => Ok(Self::Condition(ConditionConfig::default())),
            "switch" => Ok(Self::Switch(SwitchConfig::default())),
            "loop" => Ok(Self::Loop(LoopConfig::default())),
            "parallel" => Ok(Self::Parallel(ParallelConfig::default())),
            "compensate" => Ok(Self::Compensate(CompensateConfig::default())),
            "sleep" => Ok(Self::Sleep(SleepConfig::default())),
            "timeout" => Ok(Self::Timeout(TimeoutConfig::default())),
            "durable-promise" => Ok(Self::DurablePromise(DurablePromiseConfig::default())),
            "awakeable" => Ok(Self::Awakeable(AwakeableConfig::default())),
            "resolve-promise" => Ok(Self::ResolvePromise(ResolvePromiseConfig::default())),
            "signal-handler" => Ok(Self::SignalHandler(SignalHandlerConfig::default())),
            _ => Err(UnknownNodeTypeError(s.to_string())),
        }
    }
}

// ============================================================================
// Display Implementation for WorkflowNode
// ============================================================================

impl fmt::Display for WorkflowNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpHandler(_) => write!(f, "http-handler"),
            Self::HttpCall(_) => write!(f, "http-call"),
            Self::KafkaHandler(_) => write!(f, "kafka-handler"),
            Self::CronTrigger(_) => write!(f, "cron-trigger"),
            Self::WorkflowSubmit(_) => write!(f, "workflow-submit"),
            Self::Run(_) => write!(f, "run"),
            Self::ServiceCall(_) => write!(f, "service-call"),
            Self::ObjectCall(_) => write!(f, "object-call"),
            Self::WorkflowCall(_) => write!(f, "workflow-call"),
            Self::SendMessage(_) => write!(f, "send-message"),
            Self::DelayedSend(_) => write!(f, "delayed-send"),
            Self::GetState(_) => write!(f, "get-state"),
            Self::SetState(_) => write!(f, "set-state"),
            Self::ClearState(_) => write!(f, "clear-state"),
            Self::Condition(_) => write!(f, "condition"),
            Self::Switch(_) => write!(f, "switch"),
            Self::Loop(_) => write!(f, "loop"),
            Self::Parallel(_) => write!(f, "parallel"),
            Self::Compensate(_) => write!(f, "compensate"),
            Self::Sleep(_) => write!(f, "sleep"),
            Self::Timeout(_) => write!(f, "timeout"),
            Self::DurablePromise(_) => write!(f, "durable-promise"),
            Self::Awakeable(_) => write!(f, "awakeable"),
            Self::ResolvePromise(_) => write!(f, "resolve-promise"),
            Self::SignalHandler(_) => write!(f, "signal-handler"),
        }
    }
}

// ============================================================================
// WorkflowNode Methods
// ============================================================================

impl WorkflowNode {
    #[must_use]
    pub const fn category(&self) -> NodeCategory {
        match self {
            Self::HttpHandler(_) | Self::KafkaHandler(_) | Self::CronTrigger(_) => {
                NodeCategory::Entry
            }
            Self::HttpCall(_)
            | Self::ServiceCall(_)
            | Self::ObjectCall(_)
            | Self::WorkflowCall(_)
            | Self::SendMessage(_)
            | Self::DelayedSend(_)
            | Self::Run(_) => NodeCategory::Durable,
            Self::GetState(_) | Self::SetState(_) | Self::ClearState(_) => NodeCategory::State,
            Self::Condition(_)
            | Self::Switch(_)
            | Self::Loop(_)
            | Self::Parallel(_)
            | Self::Compensate(_)
            | Self::WorkflowSubmit(_) => NodeCategory::Flow,
            Self::Sleep(_) | Self::Timeout(_) => NodeCategory::Timing,
            Self::SignalHandler(_) => NodeCategory::Signal,
            Self::DurablePromise(_) | Self::Awakeable(_) | Self::ResolvePromise(_) => {
                NodeCategory::Durable
            }
        }
    }

    #[must_use]
    pub const fn icon(&self) -> super::NodeIcon {
        match self {
            Self::HttpHandler(_) => super::NodeIcon::Globe,
            Self::HttpCall(_) | Self::ServiceCall(_) => super::NodeIcon::Call,
            Self::KafkaHandler(_) => super::NodeIcon::Kafka,
            Self::CronTrigger(_) => super::NodeIcon::Clock,
            Self::WorkflowSubmit(_) | Self::WorkflowCall(_) => super::NodeIcon::Workflow,
            Self::Run(_) => super::NodeIcon::Play,
            Self::ObjectCall(_) => super::NodeIcon::Box,
            Self::SendMessage(_) => super::NodeIcon::Send,
            Self::DelayedSend(_) => super::NodeIcon::ClockSend,
            Self::GetState(_) => super::NodeIcon::Database,
            Self::SetState(_) => super::NodeIcon::Save,
            Self::ClearState(_) => super::NodeIcon::Trash,
            Self::Condition(_) => super::NodeIcon::GitBranch,
            Self::Switch(_) => super::NodeIcon::GitMerge,
            Self::Loop(_) => super::NodeIcon::Repeat,
            Self::Parallel(_) => super::NodeIcon::Layers,
            Self::Compensate(_) => super::NodeIcon::Undo,
            Self::Sleep(_) | Self::Timeout(_) => super::NodeIcon::Timer,
            Self::DurablePromise(_) => super::NodeIcon::Shield,
            Self::Awakeable(_) => super::NodeIcon::Radio,
            Self::ResolvePromise(_) => super::NodeIcon::CheckCircle,
            Self::SignalHandler(_) => super::NodeIcon::Bell,
        }
    }

    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::HttpHandler(_) => "HTTP request handler",
            Self::HttpCall(_) => "Call external HTTP API",
            Self::KafkaHandler(_) => "Kafka message handler",
            Self::CronTrigger(_) => "Scheduled cron trigger",
            Self::WorkflowSubmit(_) => "Submit nested workflow",
            Self::Run(_) => "Run arbitrary code",
            Self::ServiceCall(_) => "Call Restate service",
            Self::ObjectCall(_) => "Call Restate object",
            Self::WorkflowCall(_) => "Call Restate workflow",
            Self::SendMessage(_) => "Send message to queue",
            Self::DelayedSend(_) => "Send delayed message",
            Self::GetState(_) => "Get state value",
            Self::SetState(_) => "Set state value",
            Self::ClearState(_) => "Clear state value",
            Self::Condition(_) => "Conditional branch",
            Self::Switch(_) => "Multi-way branch",
            Self::Loop(_) => "Iterate over collection",
            Self::Parallel(_) => "Execute in parallel",
            Self::Compensate(_) => "Compensating transaction",
            Self::Sleep(_) => "Wait for duration",
            Self::Timeout(_) => "Operation timeout",
            Self::DurablePromise(_) => "Durable promise",
            Self::Awakeable(_) => "Awakeable callback",
            Self::ResolvePromise(_) => "Resolve promise",
            Self::SignalHandler(_) => "Signal handler",
        }
    }

    #[must_use]
    pub const fn output_port_type(&self) -> PortType {
        match self {
            Self::HttpHandler(_) | Self::HttpCall(_) | Self::KafkaHandler(_) => PortType::Json,
            Self::CronTrigger(_) => PortType::Event,
            Self::WorkflowSubmit(_)
            | Self::Run(_)
            | Self::ServiceCall(_)
            | Self::ObjectCall(_)
            | Self::WorkflowCall(_)
            | Self::SendMessage(_)
            | Self::DelayedSend(_)
            | Self::GetState(_)
            | Self::SetState(_)
            | Self::ClearState(_)
            | Self::Condition(_)
            | Self::Switch(_)
            | Self::Loop(_)
            | Self::Parallel(_)
            | Self::Compensate(_)
            | Self::Sleep(_)
            | Self::Timeout(_)
            | Self::DurablePromise(_)
            | Self::Awakeable(_)
            | Self::ResolvePromise(_) => PortType::FlowControl,
            Self::SignalHandler(_) => PortType::Signal,
        }
    }

    #[must_use]
    pub const fn input_port_type(&self) -> PortType {
        match self {
            Self::HttpHandler(_) | Self::KafkaHandler(_) => PortType::Json,
            Self::CronTrigger(_) => PortType::Event,
            Self::SignalHandler(_) => PortType::Signal,
            Self::HttpCall(_)
            | Self::ServiceCall(_)
            | Self::ObjectCall(_)
            | Self::WorkflowCall(_)
            | Self::SendMessage(_)
            | Self::DelayedSend(_)
            | Self::GetState(_)
            | Self::SetState(_)
            | Self::ClearState(_)
            | Self::Run(_)
            | Self::WorkflowSubmit(_)
            | Self::Condition(_)
            | Self::Switch(_)
            | Self::Loop(_)
            | Self::Parallel(_)
            | Self::Compensate(_)
            | Self::Sleep(_)
            | Self::Timeout(_)
            | Self::DurablePromise(_)
            | Self::Awakeable(_)
            | Self::ResolvePromise(_) => PortType::FlowControl,
        }
    }
}

// ============================================================================
// ConditionResult
// ============================================================================

/// Result of a condition evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConditionResult {
    True,
    False,
}

impl ConditionResult {
    #[must_use]
    pub const fn is_true(self) -> bool {
        matches!(self, Self::True)
    }

    #[must_use]
    pub const fn is_false(self) -> bool {
        matches!(self, Self::False)
    }

    #[must_use]
    pub const fn branch_port(self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
        }
    }

    #[must_use]
    pub const fn opposite_port(self) -> &'static str {
        match self {
            Self::True => "false",
            Self::False => "true",
        }
    }
}

impl From<bool> for ConditionResult {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<ConditionResult> for bool {
    fn from(value: ConditionResult) -> Self {
        value.is_true()
    }
}

impl fmt::Display for ConditionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

// ============================================================================
// HttpMethod
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl HttpMethod {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Patch => "PATCH",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for HttpMethod {
    type Err = UnknownHttpMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "PATCH" => Ok(Self::Patch),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            _ => Err(UnknownHttpMethodError(s.to_string())),
        }
    }
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownHttpMethodError(pub String);

impl fmt::Display for UnknownHttpMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown HTTP method: {}", self.0)
    }
}

impl std::error::Error for UnknownHttpMethodError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownNodeTypeError(pub String);

impl std::fmt::Display for UnknownNodeTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown node type: {}", self.0)
    }
}

impl std::error::Error for UnknownNodeTypeError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_from_str() {
        assert_eq!(HttpMethod::from_str("GET").unwrap(), HttpMethod::Get);
        assert_eq!(HttpMethod::from_str("post").unwrap(), HttpMethod::Post);
    }

    #[test]
    fn test_condition_result() {
        let result: ConditionResult = true.into();
        assert!(result.is_true());

        let bool: bool = ConditionResult::True.into();
        assert!(bool);
    }
}
