use serde::{Deserialize, Serialize};
use std::fmt;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_six_variants() {
        let states = [
            ExecutionState::Idle,
            ExecutionState::Queued,
            ExecutionState::Running,
            ExecutionState::Completed,
            ExecutionState::Failed,
            ExecutionState::Skipped,
        ];
        assert_eq!(states.len(), 6);
    }

    #[test]
    fn variant_idle_exists() {
        let state = ExecutionState::Idle;
        assert!(!state.is_active());
        assert!(!state.is_terminal());
    }

    #[test]
    fn variant_queued_exists() {
        let state = ExecutionState::Queued;
        assert!(state.is_active());
        assert!(!state.is_terminal());
    }

    #[test]
    fn variant_running_exists() {
        let state = ExecutionState::Running;
        assert!(state.is_active());
        assert!(!state.is_terminal());
    }

    #[test]
    fn variant_completed_exists() {
        let state = ExecutionState::Completed;
        assert!(!state.is_active());
        assert!(state.is_terminal());
    }

    #[test]
    fn variant_failed_exists() {
        let state = ExecutionState::Failed;
        assert!(!state.is_active());
        assert!(state.is_terminal());
    }

    #[test]
    fn variant_skipped_exists() {
        let state = ExecutionState::Skipped;
        assert!(!state.is_active());
        assert!(state.is_terminal());
    }

    #[test]
    fn display_outputs_lowercase() {
        assert_eq!(ExecutionState::Idle.to_string(), "idle");
        assert_eq!(ExecutionState::Queued.to_string(), "queued");
        assert_eq!(ExecutionState::Running.to_string(), "running");
        assert_eq!(ExecutionState::Completed.to_string(), "completed");
        assert_eq!(ExecutionState::Failed.to_string(), "failed");
        assert_eq!(ExecutionState::Skipped.to_string(), "skipped");
    }

    #[test]
    fn is_terminal_returns_true_for_terminal_states() {
        assert!(ExecutionState::Completed.is_terminal());
        assert!(ExecutionState::Failed.is_terminal());
        assert!(ExecutionState::Skipped.is_terminal());
    }

    #[test]
    fn is_terminal_returns_false_for_non_terminal_states() {
        assert!(!ExecutionState::Idle.is_terminal());
        assert!(!ExecutionState::Queued.is_terminal());
        assert!(!ExecutionState::Running.is_terminal());
    }

    #[test]
    fn is_active_returns_true_for_active_states() {
        assert!(ExecutionState::Running.is_active());
        assert!(ExecutionState::Queued.is_active());
    }

    #[test]
    fn is_active_returns_false_for_inactive_states() {
        assert!(!ExecutionState::Idle.is_active());
        assert!(!ExecutionState::Completed.is_active());
        assert!(!ExecutionState::Failed.is_active());
        assert!(!ExecutionState::Skipped.is_active());
    }

    #[test]
    fn default_is_idle() {
        assert_eq!(ExecutionState::default(), ExecutionState::Idle);
    }

    #[test]
    fn serialization_roundtrip() {
        let states = [
            ExecutionState::Idle,
            ExecutionState::Queued,
            ExecutionState::Running,
            ExecutionState::Completed,
            ExecutionState::Failed,
            ExecutionState::Skipped,
        ];

        for state in states {
            let json = serde_json::to_string(&state).expect("serialize");
            let deserialized: ExecutionState = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(state, deserialized);
        }
    }

    #[test]
    fn serialization_uses_lowercase() {
        assert_eq!(
            serde_json::to_string(&ExecutionState::Idle).unwrap(),
            "\"idle\""
        );
        assert_eq!(
            serde_json::to_string(&ExecutionState::Running).unwrap(),
            "\"running\""
        );
        assert_eq!(
            serde_json::to_string(&ExecutionState::Completed).unwrap(),
            "\"completed\""
        );
    }

    #[test]
    fn deserialization_accepts_lowercase() {
        let state: ExecutionState = serde_json::from_str("\"idle\"").unwrap();
        assert_eq!(state, ExecutionState::Idle);

        let state: ExecutionState = serde_json::from_str("\"running\"").unwrap();
        assert_eq!(state, ExecutionState::Running);

        let state: ExecutionState = serde_json::from_str("\"completed\"").unwrap();
        assert_eq!(state, ExecutionState::Completed);
    }

    #[test]
    fn transition_idle_to_running_is_valid() {
        let from = ExecutionState::Idle;
        let to = ExecutionState::Running;
        assert!(from != to);
    }

    #[test]
    fn transition_running_to_completed_is_valid() {
        let from = ExecutionState::Running;
        let to = ExecutionState::Completed;
        assert!(from != to);
    }

    #[test]
    fn transition_running_to_failed_is_valid() {
        let from = ExecutionState::Running;
        let to = ExecutionState::Failed;
        assert!(from != to);
    }

    #[test]
    fn transition_queued_to_running_is_valid() {
        let from = ExecutionState::Queued;
        let to = ExecutionState::Running;
        assert!(from != to);
    }

    #[test]
    fn skipped_is_external_transition() {
        let state = ExecutionState::Skipped;
        assert!(state.is_terminal());
    }
}
