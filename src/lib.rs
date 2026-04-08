#![warn(dead_code)]

#[cfg(not(target_arch = "wasm32"))]
pub mod agent_feedback;
#[cfg(not(target_arch = "wasm32"))]
pub mod connectivity;
#[cfg(not(target_arch = "wasm32"))]
pub mod coverage;
#[cfg(not(target_arch = "wasm32"))]
pub mod dashboard;
pub mod error;
pub mod expression_depth;
#[cfg(not(target_arch = "wasm32"))]
pub mod feedback;
pub mod flow_extender;
pub mod graph;
#[cfg(not(target_arch = "wasm32"))]
pub mod linter;
#[cfg(not(target_arch = "wasm32"))]
pub mod metrics;
pub mod restate_client;
pub mod restate_sync;
#[cfg(not(target_arch = "wasm32"))]
pub mod scenario_runner;

#[cfg(target_arch = "wasm32")]
pub mod hooks;

pub mod errors;
pub mod ui;
