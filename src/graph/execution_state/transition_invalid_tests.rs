#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Invalid transition error tests for try_transition_or_error.

use crate::graph::execution_state::{try_transition_or_error, ExecutionState, InvalidTransition};

// -----------------------------------------------------------------------
// try_transition_or_error Invalid Transition Tests (Idle source)
// -----------------------------------------------------------------------

#[test]
fn try_transition_or_error_returns_err_when_idle_to_idle() {
    let result = try_transition_or_error(ExecutionState::Idle, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Idle,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_idle_to_running() {
    let result = try_transition_or_error(ExecutionState::Idle, ExecutionState::Running);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Idle,
            ExecutionState::Running
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_idle_to_completed() {
    let result = try_transition_or_error(ExecutionState::Idle, ExecutionState::Completed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Idle,
            ExecutionState::Completed
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_idle_to_failed() {
    let result = try_transition_or_error(ExecutionState::Idle, ExecutionState::Failed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Idle,
            ExecutionState::Failed
        ))
    );
}

// -----------------------------------------------------------------------
// try_transition_or_error Invalid Transition Tests (Queued source)
// -----------------------------------------------------------------------

#[test]
fn try_transition_or_error_returns_err_when_queued_to_idle() {
    let result = try_transition_or_error(ExecutionState::Queued, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Queued,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_queued_to_completed() {
    let result = try_transition_or_error(ExecutionState::Queued, ExecutionState::Completed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Queued,
            ExecutionState::Completed
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_queued_to_failed() {
    let result = try_transition_or_error(ExecutionState::Queued, ExecutionState::Failed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Queued,
            ExecutionState::Failed
        ))
    );
}

// -----------------------------------------------------------------------
// try_transition_or_error Invalid Transition Tests (Running source)
// -----------------------------------------------------------------------

#[test]
fn try_transition_or_error_returns_err_when_running_to_idle() {
    let result = try_transition_or_error(ExecutionState::Running, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Running,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_running_to_queued() {
    let result = try_transition_or_error(ExecutionState::Running, ExecutionState::Queued);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Running,
            ExecutionState::Queued
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_running_to_skipped() {
    let result = try_transition_or_error(ExecutionState::Running, ExecutionState::Skipped);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Running,
            ExecutionState::Skipped
        ))
    );
}

// -----------------------------------------------------------------------
// try_transition_or_error Invalid Transition Tests (Completed source)
// -----------------------------------------------------------------------

#[test]
fn try_transition_or_error_returns_err_when_completed_to_idle() {
    let result = try_transition_or_error(ExecutionState::Completed, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_completed_to_queued() {
    let result = try_transition_or_error(ExecutionState::Completed, ExecutionState::Queued);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Queued
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_completed_to_running() {
    let result = try_transition_or_error(ExecutionState::Completed, ExecutionState::Running);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Running
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_completed_to_failed() {
    let result = try_transition_or_error(ExecutionState::Completed, ExecutionState::Failed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Failed
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_completed_to_skipped() {
    let result = try_transition_or_error(ExecutionState::Completed, ExecutionState::Skipped);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Skipped
        ))
    );
}
