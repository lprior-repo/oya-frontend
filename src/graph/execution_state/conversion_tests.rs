#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Tests for From trait conversions.

use crate::graph::execution_state::{
    CompletedState, ExecutionState, FailedState, IdleState, QueuedState, RunningState, SkippedState,
};

// -----------------------------------------------------------------------
// From Trait Conversion Tests
// -----------------------------------------------------------------------

#[test]
fn from_idlestate_converts_to_idle() {
    let state: ExecutionState = IdleState.into();
    assert_eq!(state, ExecutionState::Idle);
}

#[test]
fn from_queuedstate_converts_to_queued() {
    let state: ExecutionState = QueuedState.into();
    assert_eq!(state, ExecutionState::Queued);
}

#[test]
fn from_runningstate_converts_to_running() {
    let state: ExecutionState = RunningState.into();
    assert_eq!(state, ExecutionState::Running);
}

#[test]
fn from_completedstate_converts_to_completed() {
    let state: ExecutionState = CompletedState.into();
    assert_eq!(state, ExecutionState::Completed);
}

#[test]
fn from_failedstate_converts_to_failed() {
    let state: ExecutionState = FailedState.into();
    assert_eq!(state, ExecutionState::Failed);
}

#[test]
fn from_skippedstate_converts_to_skipped() {
    let state: ExecutionState = SkippedState.into();
    assert_eq!(state, ExecutionState::Skipped);
}
