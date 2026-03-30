#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Tests for execution state core types.

use crate::graph::execution_state::core::{ExecutionState, InvalidTransition};

// -----------------------------------------------------------------------
// ExecutionState Default and Display Tests
// -----------------------------------------------------------------------

#[test]
fn executionstate_returns_idle_default() {
    let state: ExecutionState = ExecutionState::default();
    assert_eq!(state, ExecutionState::Idle);
}

#[test]
fn executionstate_default_is_displayed_as_idle() {
    let state = ExecutionState::default();
    assert_eq!(format!("{}", state), "idle");
}

// -----------------------------------------------------------------------
// ExecutionState is_terminal Tests
// -----------------------------------------------------------------------

#[test]
fn executionstate_is_terminal_returns_true_when_completed() {
    let state = ExecutionState::Completed;
    assert!(state.is_terminal());
}

#[test]
fn executionstate_is_terminal_returns_true_when_failed() {
    let state = ExecutionState::Failed;
    assert!(state.is_terminal());
}

#[test]
fn executionstate_is_terminal_returns_true_when_skipped() {
    let state = ExecutionState::Skipped;
    assert!(state.is_terminal());
}

#[test]
fn executionstate_is_terminal_returns_false_when_idle() {
    let state = ExecutionState::Idle;
    assert!(!state.is_terminal());
}

#[test]
fn executionstate_is_terminal_returns_false_when_queued() {
    let state = ExecutionState::Queued;
    assert!(!state.is_terminal());
}

#[test]
fn executionstate_is_terminal_returns_false_when_running() {
    let state = ExecutionState::Running;
    assert!(!state.is_terminal());
}

// -----------------------------------------------------------------------
// ExecutionState is_active Tests
// -----------------------------------------------------------------------

#[test]
fn executionstate_is_active_returns_true_when_queued() {
    let state = ExecutionState::Queued;
    assert!(state.is_active());
}

#[test]
fn executionstate_is_active_returns_true_when_running() {
    let state = ExecutionState::Running;
    assert!(state.is_active());
}

#[test]
fn executionstate_is_active_returns_false_when_idle() {
    let state = ExecutionState::Idle;
    assert!(!state.is_active());
}

#[test]
fn executionstate_is_active_returns_false_when_completed() {
    let state = ExecutionState::Completed;
    assert!(!state.is_active());
}

#[test]
fn executionstate_is_active_returns_false_when_failed() {
    let state = ExecutionState::Failed;
    assert!(!state.is_active());
}

#[test]
fn executionstate_is_active_returns_false_when_skipped() {
    let state = ExecutionState::Skipped;
    assert!(!state.is_active());
}

// -----------------------------------------------------------------------
// ExecutionState is_idle Tests
// -----------------------------------------------------------------------

#[test]
fn executionstate_is_idle_returns_true_when_idle() {
    let state = ExecutionState::Idle;
    assert!(state.is_idle());
}

#[test]
fn executionstate_is_idle_returns_false_when_queued() {
    let state = ExecutionState::Queued;
    assert!(!state.is_idle());
}

#[test]
fn executionstate_is_idle_returns_false_when_running() {
    let state = ExecutionState::Running;
    assert!(!state.is_idle());
}

#[test]
fn executionstate_is_idle_returns_false_when_completed() {
    let state = ExecutionState::Completed;
    assert!(!state.is_idle());
}

#[test]
fn executionstate_is_idle_returns_false_when_failed() {
    let state = ExecutionState::Failed;
    assert!(!state.is_idle());
}

#[test]
fn executionstate_is_idle_returns_false_when_skipped() {
    let state = ExecutionState::Skipped;
    assert!(!state.is_idle());
}

// -----------------------------------------------------------------------
// InvalidTransition Construction Tests
// -----------------------------------------------------------------------

#[test]
fn invalidtransition_from_constructs_with_idle() {
    let err = InvalidTransition::new(ExecutionState::Idle, ExecutionState::Running);
    assert_eq!(err.from(), ExecutionState::Idle);
}

#[test]
fn invalidtransition_to_constructs_with_running() {
    let err = InvalidTransition::new(ExecutionState::Idle, ExecutionState::Running);
    assert_eq!(err.to(), ExecutionState::Running);
}

// -----------------------------------------------------------------------
// InvalidTransition Display Test
// -----------------------------------------------------------------------

#[test]
fn invalidtransition_display_formats_correctly() {
    let err = InvalidTransition::new(ExecutionState::Idle, ExecutionState::Running);
    assert_eq!(
        format!("{}", err),
        "Invalid state transition: idle -> running"
    );
}
