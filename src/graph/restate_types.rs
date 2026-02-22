#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceKind {
    Service,
    VirtualObject,
    Workflow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ContextType {
    Service,
    ObjectExclusive,
    ObjectShared,
    WorkflowExclusive,
    WorkflowShared,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseServiceKindError(pub String);

impl std::fmt::Display for ParseServiceKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid ServiceKind: {}", self.0)
    }
}

impl std::error::Error for ParseServiceKindError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseContextTypeError(pub String);

impl std::fmt::Display for ParseContextTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid ContextType: {}", self.0)
    }
}

impl std::error::Error for ParseContextTypeError {}

impl fmt::Display for ServiceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Service => write!(f, "Service"),
            Self::VirtualObject => write!(f, "VirtualObject"),
            Self::Workflow => write!(f, "Workflow"),
        }
    }
}

impl FromStr for ServiceKind {
    type Err = ParseServiceKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "service" => Ok(Self::Service),
            "virtual-object" | "virtualobject" => Ok(Self::VirtualObject),
            "workflow" => Ok(Self::Workflow),
            _ => Err(ParseServiceKindError(s.to_string())),
        }
    }
}

impl fmt::Display for ContextType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Service => write!(f, "Service"),
            Self::ObjectExclusive => write!(f, "ObjectExclusive"),
            Self::ObjectShared => write!(f, "ObjectShared"),
            Self::WorkflowExclusive => write!(f, "WorkflowExclusive"),
            Self::WorkflowShared => write!(f, "WorkflowShared"),
        }
    }
}

impl FromStr for ContextType {
    type Err = ParseContextTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "service" => Ok(Self::Service),
            "object-exclusive" | "objectexclusive" => Ok(Self::ObjectExclusive),
            "object-shared" | "objectshared" => Ok(Self::ObjectShared),
            "workflow-exclusive" | "workflowexclusive" => Ok(Self::WorkflowExclusive),
            "workflow-shared" | "workflowshared" => Ok(Self::WorkflowShared),
            _ => Err(ParseContextTypeError(s.to_string())),
        }
    }
}

impl ContextType {
    #[must_use]
    pub const fn is_exclusive(&self) -> bool {
        matches!(self, Self::ObjectExclusive | Self::WorkflowExclusive)
    }

    #[must_use]
    pub const fn is_shared(&self) -> bool {
        matches!(self, Self::ObjectShared | Self::WorkflowShared)
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::no_effect_underscore_binding
)]
mod tests {
    use super::*;

    mod service_kind {
        use super::*;

        mod variants {
            use super::*;

            #[test]
            fn service_kind_has_service_variant() {
                let _kind = ServiceKind::Service;
            }

            #[test]
            fn service_kind_has_virtual_object_variant() {
                let _kind = ServiceKind::VirtualObject;
            }

            #[test]
            fn service_kind_has_workflow_variant() {
                let _kind = ServiceKind::Workflow;
            }

            #[test]
            fn service_kind_has_exactly_three_variants() {
                let variants: Vec<ServiceKind> = vec![
                    ServiceKind::Service,
                    ServiceKind::VirtualObject,
                    ServiceKind::Workflow,
                ];
                assert_eq!(variants.len(), 3);
            }
        }

        mod derives {
            use super::*;

            #[test]
            fn service_kind_impls_copy() {
                fn assert_copy<T: Copy>() {}
                assert_copy::<ServiceKind>();
            }

            #[test]
            fn service_kind_impls_clone() {
                fn assert_clone<T: Clone>() {}
                assert_clone::<ServiceKind>();
            }

            #[test]
            fn service_kind_impls_debug() {
                fn assert_debug<T: std::fmt::Debug>() {}
                assert_debug::<ServiceKind>();
            }

            #[test]
            fn service_kind_impls_partial_eq() {
                fn assert_partial_eq<T: PartialEq>() {}
                assert_partial_eq::<ServiceKind>();
            }

            #[test]
            fn service_kind_impls_eq() {
                fn assert_eq<T: Eq>() {}
                assert_eq::<ServiceKind>();
            }

            #[test]
            fn service_kind_impls_hash() {
                fn assert_hash<T: std::hash::Hash>() {}
                assert_hash::<ServiceKind>();
            }

            #[test]
            fn service_kind_clones_correctly() {
                let original = ServiceKind::VirtualObject;
                let cloned = original.clone();
                assert_eq!(original, cloned);
            }

            #[test]
            fn service_kind_copies_correctly() {
                let original = ServiceKind::Workflow;
                let copied: ServiceKind = original;
                assert_eq!(original, copied);
            }

            #[test]
            fn service_kind_equality_works() {
                assert_eq!(ServiceKind::Service, ServiceKind::Service);
                assert_ne!(ServiceKind::Service, ServiceKind::VirtualObject);
            }
        }

        mod display {
            use super::*;

            #[test]
            fn service_variant_displays_as_capitalized() {
                let kind = ServiceKind::Service;
                let display = format!("{kind}");
                assert_eq!(display, "Service");
            }

            #[test]
            fn virtual_object_variant_displays_as_capitalized() {
                let kind = ServiceKind::VirtualObject;
                let display = format!("{kind}");
                assert_eq!(display, "VirtualObject");
            }

            #[test]
            fn workflow_variant_displays_as_capitalized() {
                let kind = ServiceKind::Workflow;
                let display = format!("{kind}");
                assert_eq!(display, "Workflow");
            }
        }

        mod from_str {
            use super::*;

            #[test]
            fn parses_service_lowercase() {
                let result: Result<ServiceKind, _> = "service".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ServiceKind::Service);
            }

            #[test]
            fn parses_service_uppercase() {
                let result: Result<ServiceKind, _> = "Service".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ServiceKind::Service);
            }

            #[test]
            fn parses_virtual_object_lowercase() {
                let result: Result<ServiceKind, _> = "virtual-object".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ServiceKind::VirtualObject);
            }

            #[test]
            fn parses_virtual_object_uppercase() {
                let result: Result<ServiceKind, _> = "VirtualObject".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ServiceKind::VirtualObject);
            }

            #[test]
            fn parses_workflow_lowercase() {
                let result: Result<ServiceKind, _> = "workflow".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ServiceKind::Workflow);
            }

            #[test]
            fn parses_workflow_uppercase() {
                let result: Result<ServiceKind, _> = "Workflow".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ServiceKind::Workflow);
            }

            #[test]
            fn rejects_invalid_input() {
                let result: Result<ServiceKind, _> = "invalid".parse();
                assert!(result.is_err());
            }

            #[test]
            fn rejects_empty_string() {
                let result: Result<ServiceKind, _> = "".parse();
                assert!(result.is_err());
            }
        }

        mod serialization {
            use super::*;

            #[test]
            fn serializes_service_as_kebab_case() {
                let kind = ServiceKind::Service;
                let json = serde_json::to_string(&kind).expect("Should serialize");
                assert!(json.contains("service"));
            }

            #[test]
            fn serializes_virtual_object_as_kebab_case() {
                let kind = ServiceKind::VirtualObject;
                let json = serde_json::to_string(&kind).expect("Should serialize");
                assert!(json.contains("virtual-object"));
            }

            #[test]
            fn serializes_workflow_as_kebab_case() {
                let kind = ServiceKind::Workflow;
                let json = serde_json::to_string(&kind).expect("Should serialize");
                assert!(json.contains("workflow"));
            }

            #[test]
            fn roundtrip_preserves_service() {
                let original = ServiceKind::Service;
                let json = serde_json::to_string(&original).expect("Should serialize");
                let restored: ServiceKind =
                    serde_json::from_str(&json).expect("Should deserialize");
                assert_eq!(original, restored);
            }

            #[test]
            fn roundtrip_preserves_virtual_object() {
                let original = ServiceKind::VirtualObject;
                let json = serde_json::to_string(&original).expect("Should serialize");
                let restored: ServiceKind =
                    serde_json::from_str(&json).expect("Should deserialize");
                assert_eq!(original, restored);
            }

            #[test]
            fn roundtrip_preserves_workflow() {
                let original = ServiceKind::Workflow;
                let json = serde_json::to_string(&original).expect("Should serialize");
                let restored: ServiceKind =
                    serde_json::from_str(&json).expect("Should deserialize");
                assert_eq!(original, restored);
            }
        }
    }

    mod context_type {
        use super::*;

        mod variants {
            use super::*;

            #[test]
            fn context_type_has_service_variant() {
                let _ctx = ContextType::Service;
            }

            #[test]
            fn context_type_has_object_exclusive_variant() {
                let _ctx = ContextType::ObjectExclusive;
            }

            #[test]
            fn context_type_has_object_shared_variant() {
                let _ctx = ContextType::ObjectShared;
            }

            #[test]
            fn context_type_has_workflow_exclusive_variant() {
                let _ctx = ContextType::WorkflowExclusive;
            }

            #[test]
            fn context_type_has_workflow_shared_variant() {
                let _ctx = ContextType::WorkflowShared;
            }

            #[test]
            fn context_type_has_exactly_five_variants() {
                let variants: Vec<ContextType> = vec![
                    ContextType::Service,
                    ContextType::ObjectExclusive,
                    ContextType::ObjectShared,
                    ContextType::WorkflowExclusive,
                    ContextType::WorkflowShared,
                ];
                assert_eq!(variants.len(), 5);
            }
        }

        mod derives {
            use super::*;

            #[test]
            fn context_type_impls_copy() {
                fn assert_copy<T: Copy>() {}
                assert_copy::<ContextType>();
            }

            #[test]
            fn context_type_impls_clone() {
                fn assert_clone<T: Clone>() {}
                assert_clone::<ContextType>();
            }

            #[test]
            fn context_type_impls_debug() {
                fn assert_debug<T: std::fmt::Debug>() {}
                assert_debug::<ContextType>();
            }

            #[test]
            fn context_type_impls_partial_eq() {
                fn assert_partial_eq<T: PartialEq>() {}
                assert_partial_eq::<ContextType>();
            }

            #[test]
            fn context_type_impls_eq() {
                fn assert_eq<T: Eq>() {}
                assert_eq::<ContextType>();
            }

            #[test]
            fn context_type_impls_hash() {
                fn assert_hash<T: std::hash::Hash>() {}
                assert_hash::<ContextType>();
            }

            #[test]
            fn context_type_clones_correctly() {
                let original = ContextType::ObjectExclusive;
                let cloned = original.clone();
                assert_eq!(original, cloned);
            }

            #[test]
            fn context_type_copies_correctly() {
                let original = ContextType::WorkflowShared;
                let copied: ContextType = original;
                assert_eq!(original, copied);
            }

            #[test]
            fn context_type_equality_works() {
                assert_eq!(ContextType::Service, ContextType::Service);
                assert_ne!(ContextType::Service, ContextType::ObjectExclusive);
            }
        }

        mod display {
            use super::*;

            #[test]
            fn service_variant_displays_as_capitalized() {
                let ctx = ContextType::Service;
                let display = format!("{ctx}");
                assert_eq!(display, "Service");
            }

            #[test]
            fn object_exclusive_variant_displays_as_capitalized() {
                let ctx = ContextType::ObjectExclusive;
                let display = format!("{ctx}");
                assert_eq!(display, "ObjectExclusive");
            }

            #[test]
            fn object_shared_variant_displays_as_capitalized() {
                let ctx = ContextType::ObjectShared;
                let display = format!("{ctx}");
                assert_eq!(display, "ObjectShared");
            }

            #[test]
            fn workflow_exclusive_variant_displays_as_capitalized() {
                let ctx = ContextType::WorkflowExclusive;
                let display = format!("{ctx}");
                assert_eq!(display, "WorkflowExclusive");
            }

            #[test]
            fn workflow_shared_variant_displays_as_capitalized() {
                let ctx = ContextType::WorkflowShared;
                let display = format!("{ctx}");
                assert_eq!(display, "WorkflowShared");
            }
        }

        mod from_str {
            use super::*;

            #[test]
            fn parses_service() {
                let result: Result<ContextType, _> = "service".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ContextType::Service);
            }

            #[test]
            fn parses_object_exclusive_kebab() {
                let result: Result<ContextType, _> = "object-exclusive".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ContextType::ObjectExclusive);
            }

            #[test]
            fn parses_object_shared_kebab() {
                let result: Result<ContextType, _> = "object-shared".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ContextType::ObjectShared);
            }

            #[test]
            fn parses_workflow_exclusive_kebab() {
                let result: Result<ContextType, _> = "workflow-exclusive".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ContextType::WorkflowExclusive);
            }

            #[test]
            fn parses_workflow_shared_kebab() {
                let result: Result<ContextType, _> = "workflow-shared".parse();
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), ContextType::WorkflowShared);
            }

            #[test]
            fn rejects_invalid_input() {
                let result: Result<ContextType, _> = "invalid".parse();
                assert!(result.is_err());
            }

            #[test]
            fn rejects_empty_string() {
                let result: Result<ContextType, _> = "".parse();
                assert!(result.is_err());
            }
        }

        mod is_exclusive {
            use super::*;

            #[test]
            fn service_is_not_exclusive() {
                assert!(!ContextType::Service.is_exclusive());
            }

            #[test]
            fn object_exclusive_is_exclusive() {
                assert!(ContextType::ObjectExclusive.is_exclusive());
            }

            #[test]
            fn object_shared_is_not_exclusive() {
                assert!(!ContextType::ObjectShared.is_exclusive());
            }

            #[test]
            fn workflow_exclusive_is_exclusive() {
                assert!(ContextType::WorkflowExclusive.is_exclusive());
            }

            #[test]
            fn workflow_shared_is_not_exclusive() {
                assert!(!ContextType::WorkflowShared.is_exclusive());
            }
        }

        mod is_shared {
            use super::*;

            #[test]
            fn service_is_not_shared() {
                assert!(!ContextType::Service.is_shared());
            }

            #[test]
            fn object_exclusive_is_not_shared() {
                assert!(!ContextType::ObjectExclusive.is_shared());
            }

            #[test]
            fn object_shared_is_shared() {
                assert!(ContextType::ObjectShared.is_shared());
            }

            #[test]
            fn workflow_exclusive_is_not_shared() {
                assert!(!ContextType::WorkflowExclusive.is_shared());
            }

            #[test]
            fn workflow_shared_is_shared() {
                assert!(ContextType::WorkflowShared.is_shared());
            }
        }

        mod serialization {
            use super::*;

            #[test]
            fn serializes_service_as_kebab_case() {
                let ctx = ContextType::Service;
                let json = serde_json::to_string(&ctx).expect("Should serialize");
                assert!(json.contains("service"));
            }

            #[test]
            fn serializes_object_exclusive_as_kebab_case() {
                let ctx = ContextType::ObjectExclusive;
                let json = serde_json::to_string(&ctx).expect("Should serialize");
                assert!(json.contains("object-exclusive"));
            }

            #[test]
            fn serializes_object_shared_as_kebab_case() {
                let ctx = ContextType::ObjectShared;
                let json = serde_json::to_string(&ctx).expect("Should serialize");
                assert!(json.contains("object-shared"));
            }

            #[test]
            fn roundtrip_preserves_service() {
                let original = ContextType::Service;
                let json = serde_json::to_string(&original).expect("Should serialize");
                let restored: ContextType =
                    serde_json::from_str(&json).expect("Should deserialize");
                assert_eq!(original, restored);
            }

            #[test]
            fn roundtrip_preserves_object_exclusive() {
                let original = ContextType::ObjectExclusive;
                let json = serde_json::to_string(&original).expect("Should serialize");
                let restored: ContextType =
                    serde_json::from_str(&json).expect("Should deserialize");
                assert_eq!(original, restored);
            }
        }
    }
}
