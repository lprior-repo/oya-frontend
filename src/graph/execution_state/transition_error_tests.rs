#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Additional transition validation tests.

use crate::graph::execution_state::{
    try_transition, try_transition_or_error, ExecutionState, InvalidTransition, StateTransition,
};

// -----------------------------------------------------------------------
// try_transition Terminal State Tests (Skipped)
// -----------------------------------------------------------------------

#[test]
fn try_transition_returns_err_when_skipped_to_idle() {
    let result = try_transition(ExecutionState::Skipped, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_returns_err_when_skipped_to_queued() {
    let result = try_transition(ExecutionState::Skipped, ExecutionState::Queued);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Queued
        ))
    );
}

#[test]
fn try_transition_returns_err_when_skipped_to_running() {
    let result = try_transition(ExecutionState::Skipped, ExecutionState::Running);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Running
        ))
    );
}

#[test]
fn try_transition_returns_err_when_skipped_to_completed() {
    let result = try_transition(ExecutionState::Skipped, ExecutionState::Completed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Completed
        ))
    );
}

#[test]
fn try_transition_returns_err_when_skipped_to_failed() {
    let result = try_transition(ExecutionState::Skipped, ExecutionState::Failed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Failed
        ))
    );
}

#[test]
fn try_transition_returns_err_when_skipped_to_skipped() {
    let result = try_transition(ExecutionState::Skipped, ExecutionState::Skipped);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Skipped
        ))
    );
}

// -----------------------------------------------------------------------
// try_transition Reverse/Invalid Transition Tests
// -----------------------------------------------------------------------

#[test]
fn try_transition_returns_err_when_queued_to_idle() {
    let result = try_transition(ExecutionState::Queued, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Queued,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_returns_err_when_running_to_idle() {
    let result = try_transition(ExecutionState::Running, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Running,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_returns_err_when_running_to_queued() {
    let result = try_transition(ExecutionState::Running, ExecutionState::Queued);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Running,
            ExecutionState::Queued
        ))
    );
}

// -----------------------------------------------------------------------
// try_transition_or_error Valid Transition Tests
// -----------------------------------------------------------------------

#[test]
fn try_transition_or_error_returns_ok_when_idle_to_queued() {
    let result = try_transition_or_error(ExecutionState::Idle, ExecutionState::Queued);
    assert_eq!(result, Ok(StateTransition::IdleToQueued));
}

#[test]
fn try_transition_or_error_returns_ok_when_idle_to_skipped() {
    let result = try_transition_or_error(ExecutionState::Idle, ExecutionState::Skipped);
    assert_eq!(result, Ok(StateTransition::IdleToSkipped));
}

#[test]
fn try_transition_or_error_returns_ok_when_queued_to_running() {
    let result = try_transition_or_error(ExecutionState::Queued, ExecutionState::Running);
    assert_eq!(result, Ok(StateTransition::QueuedToRunning));
}

#[test]
fn try_transition_or_error_returns_ok_when_queued_to_skipped() {
    let result = try_transition_or_error(ExecutionState::Queued, ExecutionState::Skipped);
    assert_eq!(result, Ok(StateTransition::QueuedToSkipped));
}

#[test]
fn try_transition_or_error_returns_ok_when_running_to_completed() {
    let result = try_transition_or_error(ExecutionState::Running, ExecutionState::Completed);
    assert_eq!(result, Ok(StateTransition::RunningToCompleted));
}

#[test]
fn try_transition_or_error_returns_ok_when_running_to_failed() {
    let result = try_transition_or_error(ExecutionState::Running, ExecutionState::Failed);
    assert_eq!(result, Ok(StateTransition::RunningToFailed));
}
