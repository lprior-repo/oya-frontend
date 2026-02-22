# Contract Specification: ServiceKind and ContextType for Restate Alignment

## Context
- **Feature**: Add ServiceKind and ContextType enums with WorkflowNode compatibility methods
- **Bead ID**: bd-1k0
- **Domain terms**:
  - **ServiceKind**: The type of Restate service (Service, VirtualObject, Workflow)
  - **ContextType**: The execution context type for a handler (Service, ObjectExclusive, ObjectShared, WorkflowExclusive, WorkflowShared)
  - **WorkflowNode**: Graph node representing a workflow step
- **Assumptions**:
  - RestateServiceKind will be renamed to ServiceKind and moved to `src/graph/restate_types.rs`
  - All existing usages of RestateServiceKind in flow_extender will be updated
  - ServiceKind and ContextType will be `Copy` types for efficient passing
- **Open questions**:
  - Should we provide a `from_service_kind()` conversion on ContextType? (deferred - not needed for this bead)

## Preconditions
- [ ] `src/graph/mod.rs` exists and exports `workflow_node` module
- [ ] `src/flow_extender/mod.rs` contains `RestateServiceKind` enum
- [ ] `WorkflowNode` enum has 24 variants with existing methods

## Postconditions
- [ ] `src/graph/restate_types.rs` created with `ServiceKind` and `ContextType` enums
- [ ] `RestateServiceKind` in flow_extender either re-exports or replaced with `ServiceKind`
- [ ] `WorkflowNode` has `compatible_service_kinds(&self) -> &'static [ServiceKind]` method
- [ ] `WorkflowNode` has `required_context_types(&self) -> &'static [ContextType]` method
- [ ] All 24 WorkflowNode variants have defined compatibility mappings
- [ ] All types implement Display, FromStr, Serialize, Deserialize

## Invariants
- [ ] `compatible_service_kinds()` always returns a non-empty slice
- [ ] `required_context_types()` always returns a non-empty slice
- [ ] ServiceKind has exactly 3 variants
- [ ] ContextType has exactly 5 variants
- [ ] Display output is parseable by FromStr (roundtrip invariant)
- [ ] All returned slices are sorted by variant declaration order

## Error Taxonomy

```rust
/// Error parsing ServiceKind from string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseServiceKindError(pub String);

impl std::fmt::Display for ParseServiceKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid ServiceKind: '{}'. Expected: service, virtual-object, or workflow", self.0)
    }
}

impl std::error::Error for ParseServiceKindError {}

/// Error parsing ContextType from string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseContextTypeError(pub String);

impl std::fmt::Display for ParseContextTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "Invalid ContextType: '{}'. Expected: service, object-exclusive, object-shared, workflow-exclusive, or workflow-shared", 
            self.0
        )
    }
}

impl std::error::Error for ParseContextTypeError {}
```

## Contract Signatures

```rust
// src/graph/restate_types.rs

/// Kind of Restate service - determines available capabilities
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceKind {
    Service,
    VirtualObject,
    Workflow,
}

impl ServiceKind {
    /// Returns all ServiceKind variants
    pub const fn all() -> &'static [ServiceKind];
    
    /// Returns the kebab-case string representation
    pub const fn as_str(self) -> &'static str;
}

impl std::fmt::Display for ServiceKind { /* kebab-case output */ }
impl std::str::FromStr for ServiceKind { /* parse kebab-case, returns ParseServiceKindError */ }

/// Execution context type - determines what operations are available
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ContextType {
    Service,
    ObjectExclusive,
    ObjectShared,
    WorkflowExclusive,
    WorkflowShared,
}

impl ContextType {
    /// Returns all ContextType variants
    pub const fn all() -> &'static [ContextType];
    
    /// Returns the kebab-case string representation
    pub const fn as_str(self) -> &'static str;
    
    /// Returns the ServiceKind this context type belongs to
    pub const fn service_kind(self) -> ServiceKind;
}

impl std::fmt::Display for ContextType { /* kebab-case output */ }
impl std::str::FromStr for ContextType { /* parse kebab-case, returns ParseContextTypeError */ }
```

```rust
// src/graph/workflow_node.rs - additions to WorkflowNode impl

impl WorkflowNode {
    /// Returns the Restate service kinds this node type is compatible with.
    /// 
    /// - Entry/Durable/Flow/Timing nodes: all three service kinds
    /// - State nodes (get/set/clear-state): VirtualObject and Workflow only
    /// - Promise/Awakeable nodes: Workflow only
    #[must_use]
    pub const fn compatible_service_kinds(&self) -> &'static [ServiceKind];
    
    /// Returns the execution context types this node type requires.
    /// 
    /// - Most nodes: all context types
    /// - State nodes: exclusive contexts only (ObjectExclusive, WorkflowExclusive)
    /// - Promise/Awakeable nodes: WorkflowExclusive only
    #[must_use]
    pub const fn required_context_types(&self) -> &'static [ContextType];
}
```

## Compatibility Matrix

### ServiceKind by Node Category

| Node Category | Variants | Compatible ServiceKinds |
|---------------|----------|-------------------------|
| Entry | HttpHandler, KafkaHandler, CronTrigger, WorkflowSubmit | [Service, VirtualObject, Workflow] |
| Durable | Run, ServiceCall, ObjectCall, WorkflowCall, SendMessage, DelayedSend | [Service, VirtualObject, Workflow] |
| State | GetState, SetState, ClearState | [VirtualObject, Workflow] |
| Flow | Condition, Switch, Loop, Parallel, Compensate | [Service, VirtualObject, Workflow] |
| Timing | Sleep, Timeout | [Service, VirtualObject, Workflow] |
| Signal (promise) | DurablePromise, Awakeable, ResolvePromise | [Workflow] |
| Signal (handler) | SignalHandler | [Service, VirtualObject, Workflow] |

### ContextType by Node Category

| Node Category | Required ContextTypes |
|---------------|----------------------|
| Entry | [Service, ObjectExclusive, ObjectShared, WorkflowExclusive, WorkflowShared] |
| Durable | [Service, ObjectExclusive, ObjectShared, WorkflowExclusive, WorkflowShared] |
| State | [ObjectExclusive, WorkflowExclusive] |
| Flow | [Service, ObjectExclusive, ObjectShared, WorkflowExclusive, WorkflowShared] |
| Timing | [Service, ObjectExclusive, ObjectShared, WorkflowExclusive, WorkflowShared] |
| Signal (promise) | [WorkflowExclusive] |
| Signal (handler) | [Service, ObjectExclusive, ObjectShared, WorkflowExclusive, WorkflowShared] |

## Non-goals
- [ ] Validation of workflow-level service kind consistency (separate bead)
- [ ] UI for selecting service kind (separate bead)
- [ ] Code generation templates (separate bead)
- [ ] Migration path for existing workflows (separate bead)
