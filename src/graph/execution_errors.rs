//! Workflow execution error types.
//!
//! This module defines all error types that can occur during workflow execution,
//! following the Design-by-Contract principles from the specification.

use crate::graph::{ExecutionState, NodeId};

// ===========================================================================
// Workflow Execution Errors
// ===========================================================================

/// Errors that can occur during workflow execution.
///
/// This enum represents all possible failure modes during workflow execution,
/// following the contract specification for comprehensive error handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowExecutionError {
    /// A cycle was detected in the workflow graph.
    /// Contains the node IDs that form the cycle.
    CycleDetected {
        /// Node IDs that form the cycle, in order.
        cycle_nodes: Vec<NodeId>,
    },

    /// A node was not found during execution.
    ///
    /// This occurs when a connection references a node that doesn't exist.
    NodeNotFound {
        /// The connection ID that references the missing node.
        connection_id: uuid::Uuid,
        /// The ID of the node that was not found.
        referenced_node: NodeId,
    },

    /// A node execution failed.
    ExecutionFailed {
        /// The node ID that failed.
        node_id: NodeId,
        /// The error message describing the failure.
        error: String,
        /// Whether the failure is retryable.
        retryable: bool,
    },

    /// A node execution exceeded its timeout.
    Timeout {
        /// The node ID that timed out (if known).
        node_id: Option<NodeId>,
        /// The actual duration in milliseconds.
        duration_ms: u64,
        /// The timeout limit in milliseconds.
        limit_ms: u64,
    },

    /// A node execution exceeded the memory limit.
    MemoryLimitExceeded {
        /// The node ID that exceeded the limit (if known).
        node_id: Option<NodeId>,
        /// The memory used in bytes.
        bytes_used: u64,
        /// The memory limit in bytes.
        limit_bytes: u64,
    },

    /// An invalid configuration was detected.
    InvalidConfig {
        /// The node ID with the invalid configuration.
        node_id: NodeId,
        /// The error message describing the configuration issue.
        error: String,
    },

    /// The workflow has no entry nodes to start execution.
    NoEntryNodes,

    /// The workflow is empty (no nodes).
    EmptyWorkflow,

    /// An invalid state transition was attempted.
    InvalidStateTransition {
        /// The node ID where the transition failed.
        node_id: NodeId,
        /// The current state before the transition.
        from: ExecutionState,
        /// The requested state after the transition.
        to: ExecutionState,
    },
}

impl std::fmt::Display for WorkflowExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CycleDetected { cycle_nodes } => {
                write!(
                    f,
                    "Cycle detected in workflow: {} -> {} -> ...",
                    cycle_nodes
                        .first()
                        .map(|n| n.to_string())
                        .unwrap_or_default(),
                    cycle_nodes
                        .get(1)
                        .map(|n| n.to_string())
                        .unwrap_or_default()
                )
            }
            Self::NodeNotFound {
                connection_id,
                referenced_node,
            } => write!(
                f,
                "Node not found: connection {} references non-existent node {}",
                connection_id, referenced_node
            ),
            Self::ExecutionFailed {
                node_id,
                error,
                retryable,
            } => {
                if *retryable {
                    write!(f, "Node {} failed (retryable): {}", node_id, error)
                } else {
                    write!(f, "Node {} failed (non-retryable): {}", node_id, error)
                }
            }
            Self::Timeout {
                node_id,
                duration_ms,
                limit_ms,
            } => match node_id {
                Some(id) => write!(
                    f,
                    "Node {} timed out: {}ms > {}ms limit",
                    id, duration_ms, limit_ms
                ),
                None => write!(
                    f,
                    "Execution timed out: {}ms > {}ms limit",
                    duration_ms, limit_ms
                ),
            },
            Self::MemoryLimitExceeded {
                node_id,
                bytes_used,
                limit_bytes,
            } => match node_id {
                Some(id) => write!(
                    f,
                    "Node {} exceeded memory: {}B > {}B limit",
                    id, bytes_used, limit_bytes
                ),
                None => write!(
                    f,
                    "Execution exceeded memory: {}B > {}B limit",
                    bytes_used, limit_bytes
                ),
            },
            Self::InvalidConfig { node_id, error } => {
                write!(f, "Node {} has invalid configuration: {}", node_id, error)
            }
            Self::NoEntryNodes => write!(f, "Workflow has no entry nodes to start execution"),
            Self::EmptyWorkflow => write!(f, "Workflow is empty (no nodes)"),
            Self::InvalidStateTransition { node_id, from, to } => write!(
                f,
                "Invalid state transition for node {}: {:?} -> {:?}",
                node_id, from, to
            ),
        }
    }
}

impl std::error::Error for WorkflowExecutionError {}
