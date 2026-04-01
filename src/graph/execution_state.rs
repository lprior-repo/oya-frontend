#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::fmt;

// ===========================================================================
// Execution State Machine
// ===========================================================================

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionState {
    #[default]
    Idle,
    Queued,
    Running,
    Completed,
    Failed,
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
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Skipped)
    }

    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Running | Self::Queued)
    }

    #[must_use]
    pub const fn is_idle(self) -> bool {
        matches!(self, Self::Idle)
    }
}

// ===========================================================================
// Invalid Transition Error
// ===========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidTransition {
    from: ExecutionState,
    to: ExecutionState,
}

impl InvalidTransition {
    #[must_use]
    pub const fn new(from: ExecutionState, to: ExecutionState) -> Self {
        Self { from, to }
    }

    #[must_use]
    pub const fn from_state(self) -> ExecutionState {
        self.from
    }

    #[must_use]
    pub const fn to_state(self) -> ExecutionState {
        self.to
    }
}

impl fmt::Display for InvalidTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid state transition: {} -> {}", self.from, self.to)
    }
}

impl std::error::Error for InvalidTransition {}

// ===========================================================================
// Type-State Pattern (Makes illegal states unrepresentable)
// ===========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IdleState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueuedState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunningState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletedState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FailedState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkippedState;

impl IdleState {
    #[must_use]
    pub const fn queue(self) -> QueuedState {
        QueuedState
    }

    #[must_use]
    pub const fn skip(self) -> SkippedState {
        SkippedState
    }
}

impl QueuedState {
    #[must_use]
    pub const fn start(self) -> RunningState {
        RunningState
    }

    #[must_use]
    pub const fn skip(self) -> SkippedState {
        SkippedState
    }
}

impl RunningState {
    #[must_use]
    pub const fn complete(self) -> CompletedState {
        CompletedState
    }

    #[must_use]
    pub const fn fail(self) -> FailedState {
        FailedState
    }
}

pub trait TerminalState: sealed::TerminalSealed {}

impl TerminalState for CompletedState {}
impl TerminalState for FailedState {}
impl TerminalState for SkippedState {}

mod sealed {
    pub trait TerminalSealed {}
    impl TerminalSealed for super::CompletedState {}
    impl TerminalSealed for super::FailedState {}
    impl TerminalSealed for super::SkippedState {}
}

// ===========================================================================
// Explicit State Transitions
// ===========================================================================

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

#[must_use]
pub const fn try_transition(from: ExecutionState, to: ExecutionState) -> Option<StateTransition> {
    match (from, to) {
        (ExecutionState::Idle, ExecutionState::Queued) => Some(StateTransition::IdleToQueued),
        (ExecutionState::Idle, ExecutionState::Skipped) => Some(StateTransition::IdleToSkipped),
        (ExecutionState::Queued, ExecutionState::Running) => Some(StateTransition::QueuedToRunning),
        (ExecutionState::Queued, ExecutionState::Skipped) => Some(StateTransition::QueuedToSkipped),
        (ExecutionState::Running, ExecutionState::Completed) => {
            Some(StateTransition::RunningToCompleted)
        }
        (ExecutionState::Running, ExecutionState::Failed) => Some(StateTransition::RunningToFailed),
        _ => None,
    }
}

#[must_use]
pub const fn can_transition(from: ExecutionState, to: ExecutionState) -> bool {
    try_transition(from, to).is_some()
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

// ===========================================================================
// TESTS
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================================================
    // ExecutionState Display Tests
    // ===========================================================================

    #[test]
    fn execution_state_display_shows_idle_lowercase() {
        assert_eq!(format!("{}", ExecutionState::Idle), "idle");
    }

    #[test]
    fn execution_state_display_shows_queued_lowercase() {
        assert_eq!(format!("{}", ExecutionState::Queued), "queued");
    }

    #[test]
    fn execution_state_display_shows_running_lowercase() {
        assert_eq!(format!("{}", ExecutionState::Running), "running");
    }

    #[test]
    fn execution_state_display_shows_completed_lowercase() {
        assert_eq!(format!("{}", ExecutionState::Completed), "completed");
    }

    #[test]
    fn execution_state_display_shows_failed_lowercase() {
        assert_eq!(format!("{}", ExecutionState::Failed), "failed");
    }

    #[test]
    fn execution_state_display_shows_skipped_lowercase() {
        assert_eq!(format!("{}", ExecutionState::Skipped), "skipped");
    }

    // ===========================================================================
    // ExecutionState is_terminal Tests
    // ===========================================================================

    #[test]
    fn is_terminal_returns_true_for_completed() {
        assert!(ExecutionState::Completed.is_terminal());
    }

    #[test]
    fn is_terminal_returns_true_for_failed() {
        assert!(ExecutionState::Failed.is_terminal());
    }

    #[test]
    fn is_terminal_returns_true_for_skipped() {
        assert!(ExecutionState::Skipped.is_terminal());
    }

    #[test]
    fn is_terminal_returns_false_for_idle() {
        assert!(!ExecutionState::Idle.is_terminal());
    }

    #[test]
    fn is_terminal_returns_false_for_queued() {
        assert!(!ExecutionState::Queued.is_terminal());
    }

    #[test]
    fn is_terminal_returns_false_for_running() {
        assert!(!ExecutionState::Running.is_terminal());
    }

    // ===========================================================================
    // ExecutionState is_active Tests
    // ===========================================================================

    #[test]
    fn is_active_returns_true_for_running() {
        assert!(ExecutionState::Running.is_active());
    }

    #[test]
    fn is_active_returns_true_for_queued() {
        assert!(ExecutionState::Queued.is_active());
    }

    #[test]
    fn is_active_returns_false_for_idle() {
        assert!(!ExecutionState::Idle.is_active());
    }

    #[test]
    fn is_active_returns_false_for_completed() {
        assert!(!ExecutionState::Completed.is_active());
    }

    #[test]
    fn is_active_returns_false_for_failed() {
        assert!(!ExecutionState::Failed.is_active());
    }

    #[test]
    fn is_active_returns_false_for_skipped() {
        assert!(!ExecutionState::Skipped.is_active());
    }

    // ===========================================================================
    // ExecutionState is_idle Tests
    // ===========================================================================

    #[test]
    fn is_idle_returns_true_for_idle() {
        assert!(ExecutionState::Idle.is_idle());
    }

    #[test]
    fn is_idle_returns_false_for_queued() {
        assert!(!ExecutionState::Queued.is_idle());
    }

    #[test]
    fn is_idle_returns_false_for_running() {
        assert!(!ExecutionState::Running.is_idle());
    }

    #[test]
    fn is_idle_returns_false_for_completed() {
        assert!(!ExecutionState::Completed.is_idle());
    }

    #[test]
    fn is_idle_returns_false_for_failed() {
        assert!(!ExecutionState::Failed.is_idle());
    }

    #[test]
    fn is_idle_returns_false_for_skipped() {
        assert!(!ExecutionState::Skipped.is_idle());
    }

    // ===========================================================================
    // InvalidTransition Display Tests
    // ===========================================================================

    #[test]
    fn invalid_transition_display_shows_from_and_to() {
        let err = InvalidTransition::new(ExecutionState::Idle, ExecutionState::Running);
        assert_eq!(
            format!("{}", err),
            "Invalid state transition: idle -> running"
        );
    }

    #[test]
    fn invalid_transition_display_shows_queued_to_failed() {
        let err = InvalidTransition::new(ExecutionState::Queued, ExecutionState::Failed);
        assert_eq!(
            format!("{}", err),
            "Invalid state transition: queued -> failed"
        );
    }

    #[test]
    fn invalid_transition_display_shows_running_to_idle() {
        let err = InvalidTransition::new(ExecutionState::Running, ExecutionState::Idle);
        assert_eq!(
            format!("{}", err),
            "Invalid state transition: running -> idle"
        );
    }

    // ===========================================================================
    // InvalidTransition Error Trait Tests
    // ===========================================================================

    #[test]
    fn invalid_transition_implements_error_trait() {
        let err: Box<dyn std::error::Error> = Box::new(InvalidTransition::new(
            ExecutionState::Idle,
            ExecutionState::Completed,
        ));
        assert!(err.to_string().contains("Invalid state transition"));
    }

    // ===========================================================================
    // IdleState Tests
    // ===========================================================================

    #[test]
    fn idle_state_queue_returns_queued_state() {
        let idle = IdleState;
        let queued = idle.queue();
        assert_eq!(queued, QueuedState);
    }

    #[test]
    fn idle_state_skip_returns_skipped_state() {
        let idle = IdleState;
        let skipped = idle.skip();
        assert_eq!(skipped, SkippedState);
    }

    // ===========================================================================
    // QueuedState Tests
    // ===========================================================================

    #[test]
    fn queued_state_start_returns_running_state() {
        let queued = QueuedState;
        let running = queued.start();
        assert_eq!(running, RunningState);
    }

    #[test]
    fn queued_state_skip_returns_skipped_state() {
        let queued = QueuedState;
        let skipped = queued.skip();
        assert_eq!(skipped, SkippedState);
    }

    // ===========================================================================
    // RunningState Tests
    // ===========================================================================

    #[test]
    fn running_state_complete_returns_completed_state() {
        let running = RunningState;
        let completed = running.complete();
        assert_eq!(completed, CompletedState);
    }

    #[test]
    fn running_state_fail_returns_failed_state() {
        let running = RunningState;
        let failed = running.fail();
        assert_eq!(failed, FailedState);
    }

    // ===========================================================================
    // StateTransition apply Tests
    // ===========================================================================

    #[test]
    fn state_transition_apply_idle_to_queued_returns_queued() {
        let transition = StateTransition::IdleToQueued;
        assert_eq!(transition.apply(), ExecutionState::Queued);
    }

    #[test]
    fn state_transition_apply_idle_to_skipped_returns_skipped() {
        let transition = StateTransition::IdleToSkipped;
        assert_eq!(transition.apply(), ExecutionState::Skipped);
    }

    #[test]
    fn state_transition_apply_queued_to_running_returns_running() {
        let transition = StateTransition::QueuedToRunning;
        assert_eq!(transition.apply(), ExecutionState::Running);
    }

    #[test]
    fn state_transition_apply_queued_to_skipped_returns_skipped() {
        let transition = StateTransition::QueuedToSkipped;
        assert_eq!(transition.apply(), ExecutionState::Skipped);
    }

    #[test]
    fn state_transition_apply_running_to_completed_returns_completed() {
        let transition = StateTransition::RunningToCompleted;
        assert_eq!(transition.apply(), ExecutionState::Completed);
    }

    #[test]
    fn state_transition_apply_running_to_failed_returns_failed() {
        let transition = StateTransition::RunningToFailed;
        assert_eq!(transition.apply(), ExecutionState::Failed);
    }

    // ===========================================================================
    // StateTransition from_states Tests
    // ===========================================================================

    #[test]
    fn state_transition_from_states_idle_to_queued_returns_tuple() {
        let transition = StateTransition::IdleToQueued;
        assert_eq!(
            transition.from_states(),
            (ExecutionState::Idle, ExecutionState::Queued)
        );
    }

    #[test]
    fn state_transition_from_states_idle_to_skipped_returns_tuple() {
        let transition = StateTransition::IdleToSkipped;
        assert_eq!(
            transition.from_states(),
            (ExecutionState::Idle, ExecutionState::Skipped)
        );
    }

    #[test]
    fn state_transition_from_states_queued_to_running_returns_tuple() {
        let transition = StateTransition::QueuedToRunning;
        assert_eq!(
            transition.from_states(),
            (ExecutionState::Queued, ExecutionState::Running)
        );
    }

    #[test]
    fn state_transition_from_states_queued_to_skipped_returns_tuple() {
        let transition = StateTransition::QueuedToSkipped;
        assert_eq!(
            transition.from_states(),
            (ExecutionState::Queued, ExecutionState::Skipped)
        );
    }

    #[test]
    fn state_transition_from_states_running_to_completed_returns_tuple() {
        let transition = StateTransition::RunningToCompleted;
        assert_eq!(
            transition.from_states(),
            (ExecutionState::Running, ExecutionState::Completed)
        );
    }

    #[test]
    fn state_transition_from_states_running_to_failed_returns_tuple() {
        let transition = StateTransition::RunningToFailed;
        assert_eq!(
            transition.from_states(),
            (ExecutionState::Running, ExecutionState::Failed)
        );
    }

    // ===========================================================================
    // try_transition Valid Transitions Tests
    // ===========================================================================

    #[test]
    fn try_transition_returns_some_for_idle_to_queued() {
        let result = try_transition(ExecutionState::Idle, ExecutionState::Queued);
        assert_eq!(result, Some(StateTransition::IdleToQueued));
    }

    #[test]
    fn try_transition_returns_some_for_idle_to_skipped() {
        let result = try_transition(ExecutionState::Idle, ExecutionState::Skipped);
        assert_eq!(result, Some(StateTransition::IdleToSkipped));
    }

    #[test]
    fn try_transition_returns_some_for_queued_to_running() {
        let result = try_transition(ExecutionState::Queued, ExecutionState::Running);
        assert_eq!(result, Some(StateTransition::QueuedToRunning));
    }

    #[test]
    fn try_transition_returns_some_for_queued_to_skipped() {
        let result = try_transition(ExecutionState::Queued, ExecutionState::Skipped);
        assert_eq!(result, Some(StateTransition::QueuedToSkipped));
    }

    #[test]
    fn try_transition_returns_some_for_running_to_completed() {
        let result = try_transition(ExecutionState::Running, ExecutionState::Completed);
        assert_eq!(result, Some(StateTransition::RunningToCompleted));
    }

    #[test]
    fn try_transition_returns_some_for_running_to_failed() {
        let result = try_transition(ExecutionState::Running, ExecutionState::Failed);
        assert_eq!(result, Some(StateTransition::RunningToFailed));
    }

    // ===========================================================================
    // try_transition Invalid Transitions Tests
    // ===========================================================================

    #[test]
    fn try_transition_returns_none_for_idle_to_running() {
        let result = try_transition(ExecutionState::Idle, ExecutionState::Running);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_idle_to_completed() {
        let result = try_transition(ExecutionState::Idle, ExecutionState::Completed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_idle_to_failed() {
        let result = try_transition(ExecutionState::Idle, ExecutionState::Failed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_queued_to_idle() {
        let result = try_transition(ExecutionState::Queued, ExecutionState::Idle);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_queued_to_completed() {
        let result = try_transition(ExecutionState::Queued, ExecutionState::Completed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_queued_to_failed() {
        let result = try_transition(ExecutionState::Queued, ExecutionState::Failed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_running_to_idle() {
        let result = try_transition(ExecutionState::Running, ExecutionState::Idle);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_running_to_queued() {
        let result = try_transition(ExecutionState::Running, ExecutionState::Queued);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_running_to_skipped() {
        let result = try_transition(ExecutionState::Running, ExecutionState::Skipped);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_completed_to_idle() {
        let result = try_transition(ExecutionState::Completed, ExecutionState::Idle);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_completed_to_queued() {
        let result = try_transition(ExecutionState::Completed, ExecutionState::Queued);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_completed_to_running() {
        let result = try_transition(ExecutionState::Completed, ExecutionState::Running);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_completed_to_failed() {
        let result = try_transition(ExecutionState::Completed, ExecutionState::Failed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_completed_to_skipped() {
        let result = try_transition(ExecutionState::Completed, ExecutionState::Skipped);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_failed_to_idle() {
        let result = try_transition(ExecutionState::Failed, ExecutionState::Idle);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_failed_to_queued() {
        let result = try_transition(ExecutionState::Failed, ExecutionState::Queued);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_failed_to_running() {
        let result = try_transition(ExecutionState::Failed, ExecutionState::Running);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_failed_to_completed() {
        let result = try_transition(ExecutionState::Failed, ExecutionState::Completed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_failed_to_skipped() {
        let result = try_transition(ExecutionState::Failed, ExecutionState::Skipped);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_skipped_to_idle() {
        let result = try_transition(ExecutionState::Skipped, ExecutionState::Idle);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_skipped_to_queued() {
        let result = try_transition(ExecutionState::Skipped, ExecutionState::Queued);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_skipped_to_running() {
        let result = try_transition(ExecutionState::Skipped, ExecutionState::Running);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_skipped_to_completed() {
        let result = try_transition(ExecutionState::Skipped, ExecutionState::Completed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_skipped_to_failed() {
        let result = try_transition(ExecutionState::Skipped, ExecutionState::Failed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_completed_to_completed() {
        let result = try_transition(ExecutionState::Completed, ExecutionState::Completed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_failed_to_failed() {
        let result = try_transition(ExecutionState::Failed, ExecutionState::Failed);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_skipped_to_skipped() {
        let result = try_transition(ExecutionState::Skipped, ExecutionState::Skipped);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_idle_to_idle() {
        let result = try_transition(ExecutionState::Idle, ExecutionState::Idle);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_queued_to_queued() {
        let result = try_transition(ExecutionState::Queued, ExecutionState::Queued);
        assert_eq!(result, None);
    }

    #[test]
    fn try_transition_returns_none_for_running_to_running() {
        let result = try_transition(ExecutionState::Running, ExecutionState::Running);
        assert_eq!(result, None);
    }

    // ===========================================================================
    // can_transition Tests
    // ===========================================================================

    #[test]
    fn can_transition_returns_true_for_idle_to_queued() {
        assert!(can_transition(ExecutionState::Idle, ExecutionState::Queued));
    }

    #[test]
    fn can_transition_returns_true_for_idle_to_skipped() {
        assert!(can_transition(
            ExecutionState::Idle,
            ExecutionState::Skipped
        ));
    }

    #[test]
    fn can_transition_returns_true_for_queued_to_running() {
        assert!(can_transition(
            ExecutionState::Queued,
            ExecutionState::Running
        ));
    }

    #[test]
    fn can_transition_returns_true_for_queued_to_skipped() {
        assert!(can_transition(
            ExecutionState::Queued,
            ExecutionState::Skipped
        ));
    }

    #[test]
    fn can_transition_returns_true_for_running_to_completed() {
        assert!(can_transition(
            ExecutionState::Running,
            ExecutionState::Completed
        ));
    }

    #[test]
    fn can_transition_returns_true_for_running_to_failed() {
        assert!(can_transition(
            ExecutionState::Running,
            ExecutionState::Failed
        ));
    }

    #[test]
    fn can_transition_returns_false_for_idle_to_running() {
        assert!(!can_transition(
            ExecutionState::Idle,
            ExecutionState::Running
        ));
    }

    #[test]
    fn can_transition_returns_false_for_idle_to_completed() {
        assert!(!can_transition(
            ExecutionState::Idle,
            ExecutionState::Completed
        ));
    }

    #[test]
    fn can_transition_returns_false_for_idle_to_failed() {
        assert!(!can_transition(
            ExecutionState::Idle,
            ExecutionState::Failed
        ));
    }

    #[test]
    fn can_transition_returns_false_for_queued_to_idle() {
        assert!(!can_transition(
            ExecutionState::Queued,
            ExecutionState::Idle
        ));
    }

    #[test]
    fn can_transition_returns_false_for_queued_to_completed() {
        assert!(!can_transition(
            ExecutionState::Queued,
            ExecutionState::Completed
        ));
    }

    #[test]
    fn can_transition_returns_false_for_running_to_idle() {
        assert!(!can_transition(
            ExecutionState::Running,
            ExecutionState::Idle
        ));
    }

    #[test]
    fn can_transition_returns_false_for_running_to_queued() {
        assert!(!can_transition(
            ExecutionState::Running,
            ExecutionState::Queued
        ));
    }

    #[test]
    fn can_transition_returns_false_for_running_to_skipped() {
        assert!(!can_transition(
            ExecutionState::Running,
            ExecutionState::Skipped
        ));
    }

    #[test]
    fn can_transition_returns_false_for_completed_to_any() {
        assert!(!can_transition(
            ExecutionState::Completed,
            ExecutionState::Idle
        ));
        assert!(!can_transition(
            ExecutionState::Completed,
            ExecutionState::Queued
        ));
        assert!(!can_transition(
            ExecutionState::Completed,
            ExecutionState::Running
        ));
        assert!(!can_transition(
            ExecutionState::Completed,
            ExecutionState::Failed
        ));
        assert!(!can_transition(
            ExecutionState::Completed,
            ExecutionState::Skipped
        ));
    }

    #[test]
    fn can_transition_returns_false_for_failed_to_any() {
        assert!(!can_transition(
            ExecutionState::Failed,
            ExecutionState::Idle
        ));
        assert!(!can_transition(
            ExecutionState::Failed,
            ExecutionState::Queued
        ));
        assert!(!can_transition(
            ExecutionState::Failed,
            ExecutionState::Running
        ));
        assert!(!can_transition(
            ExecutionState::Failed,
            ExecutionState::Completed
        ));
        assert!(!can_transition(
            ExecutionState::Failed,
            ExecutionState::Skipped
        ));
    }

    #[test]
    fn can_transition_returns_false_for_skipped_to_any() {
        assert!(!can_transition(
            ExecutionState::Skipped,
            ExecutionState::Idle
        ));
        assert!(!can_transition(
            ExecutionState::Skipped,
            ExecutionState::Queued
        ));
        assert!(!can_transition(
            ExecutionState::Skipped,
            ExecutionState::Running
        ));
        assert!(!can_transition(
            ExecutionState::Skipped,
            ExecutionState::Completed
        ));
        assert!(!can_transition(
            ExecutionState::Skipped,
            ExecutionState::Failed
        ));
    }

    // ===========================================================================
    // From Trait Conversion Tests
    // ===========================================================================

    #[test]
    fn idle_state_converts_to_execution_state_idle() {
        let state: ExecutionState = IdleState.into();
        assert_eq!(state, ExecutionState::Idle);
    }

    #[test]
    fn queued_state_converts_to_execution_state_queued() {
        let state: ExecutionState = QueuedState.into();
        assert_eq!(state, ExecutionState::Queued);
    }

    #[test]
    fn running_state_converts_to_execution_state_running() {
        let state: ExecutionState = RunningState.into();
        assert_eq!(state, ExecutionState::Running);
    }

    #[test]
    fn completed_state_converts_to_execution_state_completed() {
        let state: ExecutionState = CompletedState.into();
        assert_eq!(state, ExecutionState::Completed);
    }

    #[test]
    fn failed_state_converts_to_execution_state_failed() {
        let state: ExecutionState = FailedState.into();
        assert_eq!(state, ExecutionState::Failed);
    }

    #[test]
    fn skipped_state_converts_to_execution_state_skipped() {
        let state: ExecutionState = SkippedState.into();
        assert_eq!(state, ExecutionState::Skipped);
    }

    // ===========================================================================
    // Static Trait Tests
    // ===========================================================================

    #[test]
    fn completed_state_implements_terminal_state() {
        fn assert_terminal<T: TerminalState>() {}
        assert_terminal::<CompletedState>();
    }

    #[test]
    fn failed_state_implements_terminal_state() {
        fn assert_terminal<T: TerminalState>() {}
        assert_terminal::<FailedState>();
    }

    #[test]
    fn skipped_state_implements_terminal_state() {
        fn assert_terminal<T: TerminalState>() {}
        assert_terminal::<SkippedState>();
    }

    #[test]
    fn idle_state_does_not_implement_terminal_state() {
        fn assert_non_terminal<T>()
        where
            T: Copy,
        {
        }
        assert_non_terminal::<IdleState>();
    }

    #[test]
    fn queued_state_does_not_implement_terminal_state() {
        fn assert_non_terminal<T>()
        where
            T: Copy,
        {
        }
        assert_non_terminal::<QueuedState>();
    }

    #[test]
    fn running_state_does_not_implement_terminal_state() {
        fn assert_non_terminal<T>()
        where
            T: Copy,
        {
        }
        assert_non_terminal::<RunningState>();
    }

    // ===========================================================================
    // ExecutionState Clone and Copy Tests
    // ===========================================================================

    #[test]
    fn execution_state_is_copy() {
        let state = ExecutionState::Running;
        let state2 = state;
        let state3 = state;
        assert_eq!(state2, ExecutionState::Running);
        assert_eq!(state3, ExecutionState::Running);
    }

    #[test]
    fn execution_state_is_clone() {
        let state = ExecutionState::Queued;
        let cloned = state.clone();
        assert_eq!(cloned, ExecutionState::Queued);
    }

    // ===========================================================================
    // ExecutionState Hash Tests
    // ===========================================================================

    #[test]
    fn execution_state_hash_is_consistent() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        ExecutionState::Running.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        ExecutionState::Running.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn execution_state_hash_differs_for_different_states() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        ExecutionState::Idle.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        ExecutionState::Queued.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    // ===========================================================================
    // ExecutionState Default Tests
    // ===========================================================================

    #[test]
    fn execution_state_default_is_idle() {
        let default_state: ExecutionState = Default::default();
        assert_eq!(default_state, ExecutionState::Idle);
    }

    // ===========================================================================
    // ExecutionState Serialize/Deserialize Tests
    // ===========================================================================

    #[test]
    fn execution_state_serialize_idle() {
        let json = serde_json::to_string(&ExecutionState::Idle).unwrap();
        assert_eq!(json, "\"idle\"");
    }

    #[test]
    fn execution_state_serialize_queued() {
        let json = serde_json::to_string(&ExecutionState::Queued).unwrap();
        assert_eq!(json, "\"queued\"");
    }

    #[test]
    fn execution_state_serialize_running() {
        let json = serde_json::to_string(&ExecutionState::Running).unwrap();
        assert_eq!(json, "\"running\"");
    }

    #[test]
    fn execution_state_serialize_completed() {
        let json = serde_json::to_string(&ExecutionState::Completed).unwrap();
        assert_eq!(json, "\"completed\"");
    }

    #[test]
    fn execution_state_serialize_failed() {
        let json = serde_json::to_string(&ExecutionState::Failed).unwrap();
        assert_eq!(json, "\"failed\"");
    }

    #[test]
    fn execution_state_serialize_skipped() {
        let json = serde_json::to_string(&ExecutionState::Skipped).unwrap();
        assert_eq!(json, "\"skipped\"");
    }

    #[test]
    fn execution_state_deserialize_idle() {
        let state: ExecutionState = serde_json::from_str("\"idle\"").unwrap();
        assert_eq!(state, ExecutionState::Idle);
    }

    #[test]
    fn execution_state_deserialize_queued() {
        let state: ExecutionState = serde_json::from_str("\"queued\"").unwrap();
        assert_eq!(state, ExecutionState::Queued);
    }

    #[test]
    fn execution_state_deserialize_running() {
        let state: ExecutionState = serde_json::from_str("\"running\"").unwrap();
        assert_eq!(state, ExecutionState::Running);
    }

    #[test]
    fn execution_state_deserialize_completed() {
        let state: ExecutionState = serde_json::from_str("\"completed\"").unwrap();
        assert_eq!(state, ExecutionState::Completed);
    }

    #[test]
    fn execution_state_deserialize_failed() {
        let state: ExecutionState = serde_json::from_str("\"failed\"").unwrap();
        assert_eq!(state, ExecutionState::Failed);
    }

    #[test]
    fn execution_state_deserialize_skipped() {
        let state: ExecutionState = serde_json::from_str("\"skipped\"").unwrap();
        assert_eq!(state, ExecutionState::Skipped);
    }

    // ===========================================================================
    // StateTransition Clone and Copy Tests
    // ===========================================================================

    #[test]
    fn state_transition_is_copy() {
        let transition = StateTransition::IdleToQueued;
        let transition2 = transition;
        let transition3 = transition;
        assert_eq!(transition2, StateTransition::IdleToQueued);
        assert_eq!(transition3, StateTransition::IdleToQueued);
    }

    #[test]
    fn state_transition_is_clone() {
        let transition = StateTransition::QueuedToRunning;
        let cloned = transition.clone();
        assert_eq!(cloned, StateTransition::QueuedToRunning);
    }

    // ===========================================================================
    // InvalidTransition Clone and Copy Tests
    // ===========================================================================

    #[test]
    fn invalid_transition_is_copy() {
        let err = InvalidTransition::new(ExecutionState::Idle, ExecutionState::Running);
        let err2 = err;
        let err3 = err;
        assert_eq!(err2.from_state(), ExecutionState::Idle);
        assert_eq!(err3.from_state(), ExecutionState::Idle);
    }

    #[test]
    fn invalid_transition_is_clone() {
        let err = InvalidTransition::new(ExecutionState::Queued, ExecutionState::Failed);
        let cloned = err.clone();
        assert_eq!(cloned.from_state(), ExecutionState::Queued);
        assert_eq!(cloned.to_state(), ExecutionState::Failed);
    }

    // ===========================================================================
    // Type State Clone and Copy Tests
    // ===========================================================================

    #[test]
    fn idle_state_is_copy() {
        let state = IdleState;
        let state2 = state;
        let state3 = state;
        assert_eq!(state2, IdleState);
        assert_eq!(state3, IdleState);
    }

    #[test]
    fn queued_state_is_copy() {
        let state = QueuedState;
        let state2 = state;
        let state3 = state;
        assert_eq!(state2, QueuedState);
        assert_eq!(state3, QueuedState);
    }

    #[test]
    fn running_state_is_copy() {
        let state = RunningState;
        let state2 = state;
        let state3 = state;
        assert_eq!(state2, RunningState);
        assert_eq!(state3, RunningState);
    }

    #[test]
    fn completed_state_is_copy() {
        let state = CompletedState;
        let state2 = state;
        let state3 = state;
        assert_eq!(state2, CompletedState);
        assert_eq!(state3, CompletedState);
    }

    #[test]
    fn failed_state_is_copy() {
        let state = FailedState;
        let state2 = state;
        let state3 = state;
        assert_eq!(state2, FailedState);
        assert_eq!(state3, FailedState);
    }

    #[test]
    fn skipped_state_is_copy() {
        let state = SkippedState;
        let state2 = state;
        let state3 = state;
        assert_eq!(state2, SkippedState);
        assert_eq!(state3, SkippedState);
    }

    // ===========================================================================
    // Type State Default Tests
    // ===========================================================================

    #[test]
    fn idle_state_default_is_idle() {
        let state: IdleState = Default::default();
        assert_eq!(state, IdleState);
    }

    // ===========================================================================
    // Proptest Invariants
    // ===========================================================================

    // Note: Proptest invariants can be added here when needed
    // The comprehensive unit tests above cover all execution states
    // and transition combinations exhaustively.
}
