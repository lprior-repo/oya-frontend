#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Tests for state transition functions.

use crate::graph::execution_state::{ExecutionState, StateTransition};

// -----------------------------------------------------------------------
// StateTransition apply Tests
// -----------------------------------------------------------------------

#[test]
fn statetransition_apply_returns_queued_when_idletoqueued() {
    let transition = StateTransition::IdleToQueued;
    assert_eq!(transition.apply(), ExecutionState::Queued);
}

#[test]
fn statetransition_apply_returns_skipped_when_idletoskipped() {
    let transition = StateTransition::IdleToSkipped;
    assert_eq!(transition.apply(), ExecutionState::Skipped);
}

#[test]
fn statetransition_apply_returns_running_when_queuedtorunning() {
    let transition = StateTransition::QueuedToRunning;
    assert_eq!(transition.apply(), ExecutionState::Running);
}

#[test]
fn statetransition_apply_returns_skipped_when_queuedtoskipped() {
    let transition = StateTransition::QueuedToSkipped;
    assert_eq!(transition.apply(), ExecutionState::Skipped);
}

#[test]
fn statetransition_apply_returns_completed_when_runtocompleted() {
    let transition = StateTransition::RunningToCompleted;
    assert_eq!(transition.apply(), ExecutionState::Completed);
}

#[test]
fn statetransition_apply_returns_failed_when_runtotofailed() {
    let transition = StateTransition::RunningToFailed;
    assert_eq!(transition.apply(), ExecutionState::Failed);
}

// -----------------------------------------------------------------------
// StateTransition from_states Tests
// -----------------------------------------------------------------------

#[test]
fn statetransition_from_states_returns_idle_queued_when_idletoqueued() {
    let transition = StateTransition::IdleToQueued;
    assert_eq!(
        transition.from_states(),
        (ExecutionState::Idle, ExecutionState::Queued)
    );
}

#[test]
fn statetransition_from_states_returns_idle_skipped_when_idletoskipped() {
    let transition = StateTransition::IdleToSkipped;
    assert_eq!(
        transition.from_states(),
        (ExecutionState::Idle, ExecutionState::Skipped)
    );
}

#[test]
fn statetransition_from_states_returns_queued_running_when_queuedtorunning() {
    let transition = StateTransition::QueuedToRunning;
    assert_eq!(
        transition.from_states(),
        (ExecutionState::Queued, ExecutionState::Running)
    );
}

#[test]
fn statetransition_from_states_returns_queued_skipped_when_queuedtoskipped() {
    let transition = StateTransition::QueuedToSkipped;
    assert_eq!(
        transition.from_states(),
        (ExecutionState::Queued, ExecutionState::Skipped)
    );
}

#[test]
fn statetransition_from_states_returns_running_completed_when_runtocompleted() {
    let transition = StateTransition::RunningToCompleted;
    assert_eq!(
        transition.from_states(),
        (ExecutionState::Running, ExecutionState::Completed)
    );
}

#[test]
fn statetransition_from_states_returns_running_failed_when_runtotofailed() {
    let transition = StateTransition::RunningToFailed;
    assert_eq!(
        transition.from_states(),
        (ExecutionState::Running, ExecutionState::Failed)
    );
}
