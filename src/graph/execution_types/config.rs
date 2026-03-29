//! Node execution configuration types.

// ===========================================================================
// Execution Configuration
// ===========================================================================

/// Configuration for a single node execution.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeExecutionConfig {
    /// Node-specific timeout (overrides global).
    pub timeout_ms: Option<u64>,
    /// Whether to retry on failure.
    pub retry_count: u32,
    /// Retry backoff in milliseconds.
    pub retry_backoff_ms: u64,
}

impl Default for NodeExecutionConfig {
    fn default() -> Self {
        Self {
            timeout_ms: None,
            retry_count: 0,
            retry_backoff_ms: 100,
        }
    }
}

impl NodeExecutionConfig {
    /// Create new config with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set node-specific timeout.
    #[must_use]
    pub fn with_timeout(self, timeout_ms: u64) -> Self {
        Self {
            timeout_ms: Some(timeout_ms),
            ..self
        }
    }

    /// Set retry count.
    #[must_use]
    pub fn with_retry_count(self, retry_count: u32) -> Self {
        Self {
            retry_count,
            ..self
        }
    }

    /// Set retry backoff.
    #[must_use]
    pub fn with_retry_backoff(self, retry_backoff_ms: u64) -> Self {
        Self {
            retry_backoff_ms,
            ..self
        }
    }
}
