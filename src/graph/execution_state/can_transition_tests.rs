#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Tests for can_transition function.

use crate::graph::execution_state::{can_transition, ExecutionState};

// -----------------------------------------------------------------------
// can_transition Valid Transition Tests
// -----------------------------------------------------------------------

#[test]
fn can_transition_returns_true_when_idle_to_queued() {
    assert!(can_transition(ExecutionState::Idle, ExecutionState::Queued));
}

#[test]
fn can_transition_returns_true_when_queued_to_running() {
    assert!(can_transition(
        ExecutionState::Queued,
        ExecutionState::Running
    ));
}

// -----------------------------------------------------------------------
// can_transition Invalid Transition Tests
// -----------------------------------------------------------------------

#[test]
fn can_transition_returns_false_when_idle_to_running() {
    assert!(!can_transition(
        ExecutionState::Idle,
        ExecutionState::Running
    ));
}

#[test]
fn can_transition_returns_false_when_running_to_idle() {
    assert!(!can_transition(
        ExecutionState::Running,
        ExecutionState::Idle
    ));
}

#[test]
fn can_transition_returns_false_when_completed_to_running() {
    assert!(!can_transition(
        ExecutionState::Completed,
        ExecutionState::Running
    ));
}

#[test]
fn can_transition_returns_false_when_skipped_to_idle() {
    assert!(!can_transition(
        ExecutionState::Skipped,
        ExecutionState::Idle
    ));
}
