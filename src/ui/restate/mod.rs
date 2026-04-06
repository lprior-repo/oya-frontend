#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Restate UI components
//!
//! Components for displaying Restate invocation details and journal entries

#[cfg(target_arch = "wasm32")]
pub mod details_panel;
pub mod journal_viewer;
#[cfg(target_arch = "wasm32")]
pub mod panel;

#[cfg(target_arch = "wasm32")]
pub use details_panel::RestateInvocationDetails;
#[cfg(target_arch = "wasm32")]
pub use panel::RestateInvocationsPanel;
