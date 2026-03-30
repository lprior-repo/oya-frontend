//! Service kinds for Restate workflows.
//!
//! Implements Scott Wlaschin DDD principles:
//! - Parse, don't validate
//! - Make illegal states unrepresentable
//! - Types act as documentation

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

// ===========================================================================
// Service Kind
// ===========================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceKind {
    Handler,
    Workflow,
    Actor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseServiceKindError(pub String);

impl std::fmt::Display for ParseServiceKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid ServiceKind: {}", self.0)
    }
}

impl std::error::Error for ParseServiceKindError {}

impl fmt::Display for ServiceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Handler => write!(f, "Handler"),
            Self::Workflow => write!(f, "Workflow"),
            Self::Actor => write!(f, "Actor"),
        }
    }
}

impl FromStr for ServiceKind {
    type Err = ParseServiceKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "handler" => Ok(Self::Handler),
            "workflow" => Ok(Self::Workflow),
            "actor" => Ok(Self::Actor),
            _ => Err(ParseServiceKindError(s.to_string())),
        }
    }
}

// ===========================================================================
// Context Type
// ===========================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ContextType {
    Synchronous,
    Asynchronous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseContextTypeError(pub String);

impl std::fmt::Display for ParseContextTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid ContextType: {}", self.0)
    }
}

impl std::error::Error for ParseContextTypeError {}

impl fmt::Display for ContextType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Synchronous => write!(f, "Synchronous"),
            Self::Asynchronous => write!(f, "Asynchronous"),
        }
    }
}

impl FromStr for ContextType {
    type Err = ParseContextTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "synchronous" | "sync" => Ok(Self::Synchronous),
            "asynchronous" | "async" => Ok(Self::Asynchronous),
            _ => Err(ParseContextTypeError(s.to_string())),
        }
    }
}

impl ContextType {
    #[must_use]
    pub const fn is_synchronous(&self) -> bool {
        matches!(self, Self::Synchronous)
    }

    #[must_use]
    pub const fn is_asynchronous(&self) -> bool {
        matches!(self, Self::Asynchronous)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_kind_parses_handler() {
        let result: Result<ServiceKind, _> = "handler".parse();
        assert_eq!(result.unwrap(), ServiceKind::Handler);
    }

    #[test]
    fn context_type_parses_synchronous() {
        let result: Result<ContextType, _> = "synchronous".parse();
        assert_eq!(result.unwrap(), ContextType::Synchronous);
    }

    #[test]
    fn context_type_synchronous_is_synchronous() {
        assert!(ContextType::Synchronous.is_synchronous());
    }

    #[test]
    fn context_type_asynchronous_is_asynchronous() {
        assert!(ContextType::Asynchronous.is_asynchronous());
    }
}
