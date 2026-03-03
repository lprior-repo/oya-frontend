#![allow(dead_code)]

#[cfg(not(target_arch = "wasm32"))]
pub mod feedback;
pub mod flow_extender;
pub mod graph;
pub mod restate_client;
pub mod restate_sync;
#[cfg(not(target_arch = "wasm32"))]
pub mod ui;

#[cfg(not(target_arch = "wasm32"))]
pub mod agent_feedback;
#[cfg(not(target_arch = "wasm32"))]
pub mod coverage;
#[cfg(not(target_arch = "wasm32"))]
pub mod dashboard;
#[cfg(not(target_arch = "wasm32"))]
pub mod linter;
#[cfg(not(target_arch = "wasm32"))]
pub mod metrics;
#[cfg(not(target_arch = "wasm32"))]
pub mod scenario_runner;

#[cfg(not(target_arch = "wasm32"))]
pub use agent_feedback::{AgentFeedback, FeedbackGenerator};
#[cfg(not(target_arch = "wasm32"))]
pub use coverage::{CoverageAnalyzer, CoverageReport};
#[cfg(not(target_arch = "wasm32"))]
pub use feedback::{sanitize_results, FeedbackSanitizer};
pub use graph::{Connection, KeyedState, Node, Viewport, Workflow};
#[allow(unused_imports)]
pub use graph::RestateClient as GraphRestateClient;
pub use graph::RestateClientError;
#[cfg(not(target_arch = "wasm32"))]
pub use linter::{LintReport, SpecLinter};
#[cfg(not(target_arch = "wasm32"))]
pub use metrics::{MetricsStore, MetricsSummary};
pub use restate_client::{RestateClient, RestateClientConfig, ClientError};
pub use restate_sync::{InvocationEvent, InvocationPoller, PollResult, PollerError};
#[cfg(not(target_arch = "wasm32"))]
pub use scenario_runner::{ScenarioRunner, ValidationReport};
