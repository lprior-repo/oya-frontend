#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Additional tests for state transition validation functions.

use crate::graph::execution_state::{
    try_transition, ExecutionState, InvalidTransition, StateTransition,
};

// -----------------------------------------------------------------------
// try_transition Valid Transition Tests
// -----------------------------------------------------------------------

#[test]
fn try_transition_returns_ok_when_idle_to_queued() {
    let result = try_transition(ExecutionState::Idle, ExecutionState::Queued);
    assert_eq!(result, Ok(StateTransition::IdleToQueued));
}

#[test]
fn try_transition_returns_ok_when_idle_to_skipped() {
    let result = try_transition(ExecutionState::Idle, ExecutionState::Skipped);
    assert_eq!(result, Ok(StateTransition::IdleToSkipped));
}

#[test]
fn try_transition_returns_ok_when_queued_to_running() {
    let result = try_transition(ExecutionState::Queued, ExecutionState::Running);
    assert_eq!(result, Ok(StateTransition::QueuedToRunning));
}

#[test]
fn try_transition_returns_ok_when_queued_to_skipped() {
    let result = try_transition(ExecutionState::Queued, ExecutionState::Skipped);
    assert_eq!(result, Ok(StateTransition::QueuedToSkipped));
}

#[test]
fn try_transition_returns_ok_when_running_to_completed() {
    let result = try_transition(ExecutionState::Running, ExecutionState::Completed);
    assert_eq!(result, Ok(StateTransition::RunningToCompleted));
}

#[test]
fn try_transition_returns_ok_when_running_to_failed() {
    let result = try_transition(ExecutionState::Running, ExecutionState::Failed);
    assert_eq!(result, Ok(StateTransition::RunningToFailed));
}

// -----------------------------------------------------------------------
// try_transition Self-Loop Tests
// -----------------------------------------------------------------------

#[test]
fn try_transition_returns_err_when_idle_to_idle() {
    let result = try_transition(ExecutionState::Idle, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Idle,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_returns_err_when_queued_to_queued() {
    let result = try_transition(ExecutionState::Queued, ExecutionState::Queued);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Queued,
            ExecutionState::Queued
        ))
    );
}

#[test]
fn try_transition_returns_err_when_running_to_running() {
    let result = try_transition(ExecutionState::Running, ExecutionState::Running);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Running,
            ExecutionState::Running
        ))
    );
}

// -----------------------------------------------------------------------
// try_transition Terminal State Tests (Completed)
// -----------------------------------------------------------------------

#[test]
fn try_transition_returns_err_when_completed_to_idle() {
    let result = try_transition(ExecutionState::Completed, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_returns_err_when_completed_to_queued() {
    let result = try_transition(ExecutionState::Completed, ExecutionState::Queued);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Queued
        ))
    );
}

#[test]
fn try_transition_returns_err_when_completed_to_running() {
    let result = try_transition(ExecutionState::Completed, ExecutionState::Running);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Running
        ))
    );
}

#[test]
fn try_transition_returns_err_when_completed_to_completed() {
    let result = try_transition(ExecutionState::Completed, ExecutionState::Completed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Completed
        ))
    );
}

#[test]
fn try_transition_returns_err_when_completed_to_failed() {
    let result = try_transition(ExecutionState::Completed, ExecutionState::Failed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Failed
        ))
    );
}

#[test]
fn try_transition_returns_err_when_completed_to_skipped() {
    let result = try_transition(ExecutionState::Completed, ExecutionState::Skipped);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Completed,
            ExecutionState::Skipped
        ))
    );
}

// -----------------------------------------------------------------------
// try_transition Terminal State Tests (Failed)
// -----------------------------------------------------------------------

#[test]
fn try_transition_returns_err_when_failed_to_idle() {
    let result = try_transition(ExecutionState::Failed, ExecutionState::Idle);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Idle
        ))
    );
}

#[test]
fn try_transition_returns_err_when_failed_to_queued() {
    let result = try_transition(ExecutionState::Failed, ExecutionState::Queued);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Queued
        ))
    );
}

#[test]
fn try_transition_returns_err_when_failed_to_running() {
    let result = try_transition(ExecutionState::Failed, ExecutionState::Running);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Running
        ))
    );
}

#[test]
fn try_transition_returns_err_when_failed_to_completed() {
    let result = try_transition(ExecutionState::Failed, ExecutionState::Completed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Completed
        ))
    );
}

#[test]
fn try_transition_returns_err_when_failed_to_failed() {
    let result = try_transition(ExecutionState::Failed, ExecutionState::Failed);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Failed
        ))
    );
}

#[test]
fn try_transition_returns_err_when_failed_to_skipped() {
    let result = try_transition(ExecutionState::Failed, ExecutionState::Skipped);
    assert_eq!(
        result,
        Err(InvalidTransition::new(
            ExecutionState::Failed,
            ExecutionState::Skipped
        ))
    );
}
