#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Type-state pattern implementation for execution state machine.
//!
//! This module implements the type-state pattern to make illegal states
//! unrepresentable at compile time. Each state is represented by a distinct
//! type, and transitions are methods on those types.

// ===========================================================================
// Type-State Pattern (Makes illegal states unrepresentable)
// ===========================================================================

/// Marker type representing the Idle state.
///
/// This type-state enables compile-time enforcement of valid transitions
/// from the [`ExecutionState::Idle`](super::core::ExecutionState::Idle) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IdleState;

/// Marker type representing the Queued state.
///
/// This type-state enables compile-time enforcement of valid transitions
/// from the [`ExecutionState::Queued`](super::core::ExecutionState::Queued) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct QueuedState;

/// Marker type representing the Running state.
///
/// This type-state enables compile-time enforcement of valid transitions
/// from the [`ExecutionState::Running`](super::core::ExecutionState::Running) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunningState;

/// Marker type representing the Completed state (terminal).
///
/// This type-state enables compile-time enforcement that no outgoing
/// transitions are possible from the [`ExecutionState::Completed`](super::core::ExecutionState::Completed) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CompletedState;

/// Marker type representing the Failed state (terminal).
///
/// This type-state enables compile-time enforcement that no outgoing
/// transitions are possible from the [`ExecutionState::Failed`](super::core::ExecutionState::Failed) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FailedState;

/// Marker type representing the Skipped state (terminal).
///
/// This type-state enables compile-time enforcement that no outgoing
/// transitions are possible from the [`ExecutionState::Skipped`](super::core::ExecutionState::Skipped) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkippedState;

// IdleState transitions
impl IdleState {
    /// Transition from Idle to Queued
    #[must_use]
    pub const fn queue(self) -> QueuedState {
        QueuedState
    }

    /// Transition from Idle to Skipped
    #[must_use]
    pub const fn skip(self) -> SkippedState {
        SkippedState
    }
}

// QueuedState transitions
impl QueuedState {
    /// Transition from Queued to Running
    #[must_use]
    pub const fn start(self) -> RunningState {
        RunningState
    }

    /// Transition from Queued to Skipped
    #[must_use]
    pub const fn skip(self) -> SkippedState {
        SkippedState
    }
}

// RunningState transitions
impl RunningState {
    /// Transition from Running to Completed
    #[must_use]
    pub const fn complete(self) -> CompletedState {
        CompletedState
    }

    /// Transition from Running to Failed
    #[must_use]
    pub const fn fail(self) -> FailedState {
        FailedState
    }
}

// ===========================================================================
// Terminal State Trait
// ===========================================================================

/// Marker trait for terminal states (states with no outgoing transitions).
///
/// This trait is used to enforce at compile time that only terminal states
/// can be used in contexts where terminal behavior is expected.
///
/// # Examples
///
/// ```
/// use oya_frontend_aik::graph::execution_state::{CompletedState, FailedState, SkippedState, TerminalState};
///
/// fn assert_terminal<S: TerminalState>() {}
/// assert_terminal::<CompletedState>();
/// assert_terminal::<FailedState>();
/// assert_terminal::<SkippedState>();
/// ```
pub trait TerminalState {}

impl TerminalState for CompletedState {}
impl TerminalState for FailedState {}
impl TerminalState for SkippedState {}
