#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Explicit state transition functions and conversions.
//!
//! This module provides functions for checking and performing state transitions
//! at the [`ExecutionState`] level, along with conversion implementations.

use super::core::{ExecutionState, InvalidTransition};
use super::type_state::{
    CompletedState, FailedState, IdleState, QueuedState, RunningState, SkippedState,
};

// ===========================================================================
// Explicit State Transitions
// ===========================================================================

/// Represents a valid state transition in the execution state machine.
///
/// This enum encodes all allowed transitions, making it impossible to
/// represent invalid transitions at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateTransition {
    IdleToQueued,
    IdleToSkipped,
    QueuedToRunning,
    QueuedToSkipped,
    RunningToCompleted,
    RunningToFailed,
}

impl StateTransition {
    /// Apply this transition and return the resulting [`ExecutionState`]
    #[must_use]
    pub const fn apply(self) -> ExecutionState {
        match self {
            Self::IdleToQueued => ExecutionState::Queued,
            Self::IdleToSkipped | Self::QueuedToSkipped => ExecutionState::Skipped,
            Self::QueuedToRunning => ExecutionState::Running,
            Self::RunningToCompleted => ExecutionState::Completed,
            Self::RunningToFailed => ExecutionState::Failed,
        }
    }

    /// Returns the source and target states for this transition
    #[must_use]
    pub const fn from_states(self) -> (ExecutionState, ExecutionState) {
        match self {
            Self::IdleToQueued => (ExecutionState::Idle, ExecutionState::Queued),
            Self::IdleToSkipped => (ExecutionState::Idle, ExecutionState::Skipped),
            Self::QueuedToRunning => (ExecutionState::Queued, ExecutionState::Running),
            Self::QueuedToSkipped => (ExecutionState::Queued, ExecutionState::Skipped),
            Self::RunningToCompleted => (ExecutionState::Running, ExecutionState::Completed),
            Self::RunningToFailed => (ExecutionState::Running, ExecutionState::Failed),
        }
    }
}

// ===========================================================================
// Transition Validation Functions
// ===========================================================================

/// Checks if a transition from `from` to `to` is valid.
///
/// Returns `Ok(StateTransition)` if the transition is valid, `Err(InvalidTransition)` otherwise.
///
/// # Errors
///
/// Returns [`InvalidTransition`] when the requested state transition is not allowed
/// by the state machine's transition rules.
///
/// # Examples
///
/// ```
/// use oya_frontend::graph::execution_state::{ExecutionState, StateTransition, try_transition};
///
/// // Valid transition
/// let result = try_transition(ExecutionState::Idle, ExecutionState::Queued);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), StateTransition::IdleToQueued);
///
/// // Invalid transition
/// let result = try_transition(ExecutionState::Idle, ExecutionState::Running);
/// assert!(result.is_err());
/// ```
#[must_use = "the transition result should be checked"]
pub const fn try_transition(
    from: ExecutionState,
    to: ExecutionState,
) -> Result<StateTransition, InvalidTransition> {
    match (from, to) {
        (ExecutionState::Idle, ExecutionState::Queued) => Ok(StateTransition::IdleToQueued),
        (ExecutionState::Idle, ExecutionState::Skipped) => Ok(StateTransition::IdleToSkipped),
        (ExecutionState::Queued, ExecutionState::Running) => Ok(StateTransition::QueuedToRunning),
        (ExecutionState::Queued, ExecutionState::Skipped) => Ok(StateTransition::QueuedToSkipped),
        (ExecutionState::Running, ExecutionState::Completed) => {
            Ok(StateTransition::RunningToCompleted)
        }
        (ExecutionState::Running, ExecutionState::Failed) => Ok(StateTransition::RunningToFailed),
        (from, to) => Err(InvalidTransition::new(from, to)),
    }
}

/// Helper function that returns Result for functions that need to report invalid transitions
/// as errors rather than Option.
///
/// # Errors
///
/// Returns [`InvalidTransition`] if the transition from `from` to `to` is not valid.
///
/// # Examples
///
/// ```
/// use oya_frontend_aik::graph::execution_state::{ExecutionState, try_transition_or_error};
///
/// let result = try_transition_or_error(ExecutionState::Idle, ExecutionState::Queued);
/// assert!(result.is_ok());
///
/// let result = try_transition_or_error(ExecutionState::Idle, ExecutionState::Running);
/// assert!(result.is_err());
/// ```
#[must_use = "the transition result should be checked"]
pub const fn try_transition_or_error(
    from: ExecutionState,
    to: ExecutionState,
) -> Result<StateTransition, InvalidTransition> {
    try_transition(from, to)
}

/// Checks if a transition from `from` to `to` is valid.
///
/// Returns `true` if the transition is allowed, `false` otherwise.
#[must_use]
pub const fn can_transition(from: ExecutionState, to: ExecutionState) -> bool {
    try_transition(from, to).is_ok()
}

// ===========================================================================
// State Machine Conversions
// ===========================================================================

impl From<IdleState> for ExecutionState {
    fn from(_: IdleState) -> Self {
        Self::Idle
    }
}

impl From<QueuedState> for ExecutionState {
    fn from(_: QueuedState) -> Self {
        Self::Queued
    }
}

impl From<RunningState> for ExecutionState {
    fn from(_: RunningState) -> Self {
        Self::Running
    }
}

impl From<CompletedState> for ExecutionState {
    fn from(_: CompletedState) -> Self {
        Self::Completed
    }
}

impl From<FailedState> for ExecutionState {
    fn from(_: FailedState) -> Self {
        Self::Failed
    }
}

impl From<SkippedState> for ExecutionState {
    fn from(_: SkippedState) -> Self {
        Self::Skipped
    }
}
