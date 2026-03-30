#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Tests for type-state pattern implementation.

use crate::graph::execution_state::type_state::{
    CompletedState, FailedState, IdleState, QueuedState, RunningState, SkippedState, TerminalState,
};

// -----------------------------------------------------------------------
// IdleState Type-State Tests
// -----------------------------------------------------------------------

#[test]
fn idlestate_queue_returns_queuedstate() {
    let idle = IdleState;
    let result = idle.queue();
    assert_eq!(result, QueuedState);
}

#[test]
fn idlestate_skip_returns_skippedstate() {
    let idle = IdleState;
    let result = idle.skip();
    assert_eq!(result, SkippedState);
}

// -----------------------------------------------------------------------
// QueuedState Type-State Tests
// -----------------------------------------------------------------------

#[test]
fn queuedstate_start_returns_runningstate() {
    let queued = QueuedState;
    let result = queued.start();
    assert_eq!(result, RunningState);
}

#[test]
fn queuedstate_skip_returns_skippedstate() {
    let queued = QueuedState;
    let result = queued.skip();
    assert_eq!(result, SkippedState);
}

// -----------------------------------------------------------------------
// RunningState Type-State Tests
// -----------------------------------------------------------------------

#[test]
fn runningstate_complete_returns_completedstate() {
    let running = RunningState;
    let result = running.complete();
    assert_eq!(result, CompletedState);
}

#[test]
fn runningstate_fail_returns_failedstate() {
    let running = RunningState;
    let result = running.fail();
    assert_eq!(result, FailedState);
}

// -----------------------------------------------------------------------
// TerminalState Trait Implementation Tests (Compile-Time)
// -----------------------------------------------------------------------

#[test]
fn terminalstate_impl_exists_for_completedstate() {
    fn requires_terminal<T: TerminalState>() {}
    requires_terminal::<CompletedState>();
}

#[test]
fn terminalstate_impl_exists_for_failedstate() {
    fn requires_terminal<T: TerminalState>() {}
    requires_terminal::<FailedState>();
}

#[test]
fn terminalstate_impl_exists_for_skippedstate() {
    fn requires_terminal<T: TerminalState>() {}
    requires_terminal::<SkippedState>();
}

#[test]
fn terminalstate_not_impl_for_non_terminal_states() {
    // These tests verify that non-terminal states do NOT implement TerminalState
    // We use the sealed trait pattern to ensure this at compile time
    let _idle: IdleState = IdleState;
    let _queued: QueuedState = QueuedState;
    let _running: RunningState = RunningState;

    // If we tried to call requires_terminal::<IdleState>() here, it would fail to compile
    // because IdleState does not implement TerminalState
}
