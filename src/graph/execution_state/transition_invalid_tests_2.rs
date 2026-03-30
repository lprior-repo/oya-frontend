#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Invalid transition error tests for try_transition_or_error (Failed and Skipped sources).

use crate::graph::execution_state::{try_transition_or_error, ExecutionState, InvalidTransition};

// -----------------------------------------------------------------------
// try_transition_or_error Invalid Transition Tests (Failed source)
// -----------------------------------------------------------------------

#[test]
fn try_transition_or_error_returns_err_when_failed_to_idle() {
    let result = try_transition_or_error(ExecutionState::Failed, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_failed_to_queued() {
    let result = try_transition_or_error(ExecutionState::Failed, ExecutionState::Queued);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Queued
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_failed_to_running() {
    let result = try_transition_or_error(ExecutionState::Failed, ExecutionState::Running);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Running
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_failed_to_completed() {
    let result = try_transition_or_error(ExecutionState::Failed, ExecutionState::Completed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Completed
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_failed_to_skipped() {
    let result = try_transition_or_error(ExecutionState::Failed, ExecutionState::Skipped);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Skipped
        ))
    );
}

// -----------------------------------------------------------------------
// try_transition_or_error Invalid Transition Tests (Skipped source)
// -----------------------------------------------------------------------

#[test]
fn try_transition_or_error_returns_err_when_skipped_to_idle() {
    let result = try_transition_or_error(ExecutionState::Skipped, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_skipped_to_queued() {
    let result = try_transition_or_error(ExecutionState::Skipped, ExecutionState::Queued);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Queued
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_skipped_to_running() {
    let result = try_transition_or_error(ExecutionState::Skipped, ExecutionState::Running);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Running
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_skipped_to_completed() {
    let result = try_transition_or_error(ExecutionState::Skipped, ExecutionState::Completed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Completed
        ))
    );
}

#[test]
fn try_transition_or_error_returns_err_when_skipped_to_failed() {
    let result = try_transition_or_error(ExecutionState::Skipped, ExecutionState::Failed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Skipped,
            ExecutionState::Failed
        ))
    );
}
