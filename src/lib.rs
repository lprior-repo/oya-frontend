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
pub use graph::{Connection, Node, Viewport, Workflow};
#[cfg(not(target_arch = "wasm32"))]
pub use linter::{LintReport, SpecLinter};
#[cfg(not(target_arch = "wasm32"))]
pub use metrics::{MetricsStore, MetricsSummary};
#[cfg(not(target_arch = "wasm32"))]
pub use scenario_runner::{ScenarioRunner, ValidationReport};
