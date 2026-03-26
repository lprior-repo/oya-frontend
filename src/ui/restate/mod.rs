#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Restate UI components
//!
//! Components for displaying Restate invocation details and journal entries

pub mod details_panel;
pub mod journal_viewer;
pub mod panel;

pub use details_panel::RestateInvocationDetails;
pub use journal_viewer::RestateJournalViewer;
pub use panel::RestateInvocationsPanel;
