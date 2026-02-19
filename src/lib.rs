#![allow(dead_code)]

#[cfg(not(target_arch = "wasm32"))]
pub mod feedback;
pub mod graph;

#[cfg(not(target_arch = "wasm32"))]
pub mod agent_feedback;
#[cfg(not(target_arch = "wasm32"))]
pub mod coverage;
#[cfg(not(target_arch = "wasm32"))]
pub mod dashboard;
#[cfg(not(target_arch = "wasm32"))]
pub mod deployment;
#[cfg(not(target_arch = "wasm32"))]
pub mod linter;
#[cfg(not(target_arch = "wasm32"))]
pub mod metrics;
#[cfg(not(target_arch = "wasm32"))]
pub mod scenario_runner;
#[cfg(not(target_arch = "wasm32"))]
pub mod twin_runtime;

#[cfg(not(target_arch = "wasm32"))]
pub use linter::{SpecLinter, LintReport};
#[cfg(not(target_arch = "wasm32"))]
pub use scenario_runner::{ScenarioRunner, ValidationReport};
#[cfg(not(target_arch = "wasm32"))]
pub use feedback::{FeedbackSanitizer, sanitize_results};
#[cfg(not(target_arch = "wasm32"))]
pub use twin_runtime::{TwinInstance, load_twin_definition};
#[cfg(not(target_arch = "wasm32"))]
pub use metrics::{MetricsStore, MetricsSummary};
#[cfg(not(target_arch = "wasm32"))]
pub use deployment::{TwinDeploymentManager};
#[cfg(not(target_arch = "wasm32"))]
pub use coverage::{CoverageAnalyzer, CoverageReport};
#[cfg(not(target_arch = "wasm32"))]
pub use agent_feedback::{FeedbackGenerator, AgentFeedback};
pub use graph::{Workflow, Node, Connection, Viewport};
