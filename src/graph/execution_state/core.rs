#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

//! Core types for execution state management
//!
//! This module defines the fundamental types for the execution state machine,
//! including the [`ExecutionState`] enum and [`InvalidTransition`] error type.

use serde::{Deserialize, Serialize};
use std::fmt;

// ===========================================================================
// Execution State Machine
// ===========================================================================

/// Represents the current state of an execution in the workflow.
///
/// This is the core type for the state machine, representing all possible
/// states an execution can be in.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionState {
    /// Initial state - execution has not started
    #[default]
    Idle,
    /// Execution has been queued but not yet started
    Queued,
    /// Execution is actively running
    Running,
    /// Execution completed successfully
    Completed,
    /// Execution failed
    Failed,
    /// Execution was skipped
    Skipped,
}

impl fmt::Display for ExecutionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "idle"),
            Self::Queued => write!(f, "queued"),
            Self::Running => write!(f, "running"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Skipped => write!(f, "skipped"),
        }
    }
}

impl ExecutionState {
    /// Returns true if this is a terminal state (no transitions allowed from here)
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Skipped)
    }

    /// Returns true if this is an active state (execution is in progress)
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Running | Self::Queued)
    }

    /// Returns true if this is the idle state
    #[must_use]
    pub const fn is_idle(self) -> bool {
        matches!(self, Self::Idle)
    }
}

// ===========================================================================
// Invalid Transition Error
// ===========================================================================

/// Error representing an invalid state transition.
///
/// This type is used when code attempts to transition between states
/// that are not allowed by the state machine.
///
/// # Examples
///
/// ```
/// use oya_frontend_aik::graph::execution_state::{ExecutionState, InvalidTransition};
///
/// let err = InvalidTransition::new(ExecutionState::Idle, ExecutionState::Running);
/// assert_eq!(format!("{}", err), "Invalid state transition: idle -> running");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidTransition {
    /// The source state of the attempted transition.
    from: ExecutionState,
    /// The target state of the attempted transition.
    to: ExecutionState,
}

impl InvalidTransition {
    /// Creates a new [`InvalidTransition`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use oya_frontend_aik::graph::execution_state::{ExecutionState, InvalidTransition};
    ///
    /// let err = InvalidTransition::new(ExecutionState::Idle, ExecutionState::Running);
    /// assert_eq!(err.from(), ExecutionState::Idle);
    /// assert_eq!(err.to(), ExecutionState::Running);
    /// ```
    #[must_use]
    pub const fn new(from: ExecutionState, to: ExecutionState) -> Self {
        Self { from, to }
    }

    /// Returns the source state of the attempted transition.
    ///
    /// # Examples
    ///
    /// ```
    /// use oya_frontend_aik::graph::execution_state::{ExecutionState, InvalidTransition};
    ///
    /// let err = InvalidTransition::new(ExecutionState::Idle, ExecutionState::Running);
    /// assert_eq!(err.from(), ExecutionState::Idle);
    /// ```
    #[must_use]
    pub const fn from(&self) -> ExecutionState {
        self.from
    }

    /// Returns the target state of the attempted transition.
    ///
    /// # Examples
    ///
    /// ```
    /// use oya_frontend_aik::graph::execution_state::{ExecutionState, InvalidTransition};
    ///
    /// let err = InvalidTransition::new(ExecutionState::Idle, ExecutionState::Running);
    /// assert_eq!(err.to(), ExecutionState::Running);
    /// ```
    #[must_use]
    pub const fn to(&self) -> ExecutionState {
        self.to
    }
}

impl fmt::Display for InvalidTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid state transition: {} -> {}", self.from, self.to)
    }
}

impl std::error::Error for InvalidTransition {}
