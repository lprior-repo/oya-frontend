# Contract Specification: WorkflowNode Enum

## Context

- **Feature**: Create unified WorkflowNode enum with typed configs to replace stringly-typed `node_type: String` and `config: serde_json::Value`
- **Domain terms**:
  - `WorkflowNode` - Sum type representing all 24 node types with their typed configs
  - `NodeType` - Kebab-case string identifier (e.g., "http-handler")
  - `NodeCategory` - Classification (Entry, Durable, State, Flow, Timing, Signal)
  - `TypedConfig` - Variant-specific struct with schema fields only
  - `NodeRuntimeState` - Separate struct for execution state (status, executing, etc.)
- **Assumptions**:
  - Existing workflows in localStorage must remain loadable via migration
  - Serde tag = "type" for JSON compatibility with existing format
  - Category and icon are derivable from variant, not stored separately
- **Open questions**:
  - Should config structs implement validation? (Out of scope for this bead)
  - Should there be a generic `NodeConfig` trait? (Deferred to future bead)

## Preconditions

- [ ] All 24 NODE_TEMPLATES from `src/ui/sidebar/model.rs` are available as reference
- [ ] Existing `NodeCategory` enum in `src/graph/mod.rs` is unchanged
- [ ] Config field requirements from SCOUT analysis are documented
- [ ] Runtime fields are identified for separation (status, configured, journalIndex, etc.)

## Postconditions

- [ ] `WorkflowNode` enum exists in `src/graph/mod.rs` with exactly 24 variants
- [ ] Each variant has a corresponding typed config struct
- [ ] Variant names use PascalCase (http-handler -> HttpHandler)
- [ ] Serde uses `#[serde(tag = "type", rename_all = "kebab-case")]`
- [ ] `category()` method returns correct `NodeCategory` for each variant
- [ ] `icon()` method returns correct icon string for each variant
- [ ] `TryFrom<&str>` implemented for parsing node_type strings
- [ ] `Display` trait outputs kebab-case node_type string
- [ ] Round-trip serialization/deserialization preserves all data
- [ ] `NodeRuntimeState` struct exists for execution state

## Invariants

### Type Invariants

1. **Variant Count**: `WorkflowNode` always has exactly 24 variants
2. **Variant Name Mapping**: Each variant's PascalCase name maps to exactly one kebab-case node_type
3. **Category Consistency**: `variant.category()` always returns the same `NodeCategory` for a given variant
4. **Icon Consistency**: `variant.icon()` always returns the same icon string for a given variant

### Serialization Invariants

5. **Type Tag**: Serialized JSON always contains a `"type"` field with kebab-case value
6. **Round-Trip**: `deserialize(serialize(node))` equals original `node` for all valid configs
7. **No Panic**: Serialization and deserialization never panic on valid input

### Config Invariants

8. **Config Separation**: Config structs contain only schema fields, never runtime state
9. **Default Available**: All config structs implement `Default`
10. **Field Types**: Config field types match SCOUT analysis (String, u64, Option<T>, Vec<T>)

### Error Invariants

11. **Unknown Type**: Parsing unknown node_type string returns `Err`, never panics
12. **Missing Type**: Deserializing JSON without `"type"` field returns `Err`
13. **Descriptive Errors**: All errors contain sufficient context for debugging

## Error Taxonomy

```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum WorkflowNodeError {
    #[error("unknown node type: '{0}' - expected one of: {1}")]
    UnknownNodeType(String, &'static str),
    
    #[error("missing required field '{field}' in {variant} config")]
    MissingConfigField { variant: String, field: String },
    
    #[error("invalid config value for '{field}' in {variant}: {reason}")]
    InvalidConfigValue { variant: String, field: String, reason: String },
    
    #[error("deserialization failed: {0}")]
    DeserializationError(#[source] serde_json::Error),
    
    #[error("serialization failed: {0}")]
    SerializationError(#[source] serde_json::Error),
}
```

## Contract Signatures

### Core Type

```rust
/// Unified enum for all workflow node types with typed configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum WorkflowNode {
    // Entry (4)
    HttpHandler(HttpHandlerConfig),
    KafkaHandler(KafkaHandlerConfig),
    CronTrigger(CronTriggerConfig),
    WorkflowSubmit(WorkflowSubmitConfig),
    
    // Durable (6)
    Run(RunConfig),
    ServiceCall(ServiceCallConfig),
    ObjectCall(ObjectCallConfig),
    WorkflowCall(WorkflowCallConfig),
    SendMessage(SendMessageConfig),
    DelayedSend(DelayedSendConfig),
    
    // State (3)
    GetState(GetStateConfig),
    SetState(SetStateConfig),
    ClearState(ClearStateConfig),
    
    // Flow (5)
    Condition(ConditionConfig),
    Switch(SwitchConfig),
    Loop(LoopConfig),
    Parallel(ParallelConfig),
    Compensate(CompensateConfig),
    
    // Timing (2)
    Sleep(SleepConfig),
    Timeout(TimeoutConfig),
    
    // Signal (4)
    DurablePromise(DurablePromiseConfig),
    Awakeable(AwakeableConfig),
    ResolvePromise(ResolvePromiseConfig),
    SignalHandler(SignalHandlerConfig),
}
```

### Methods

```rust
impl WorkflowNode {
    /// Returns the category for this node type.
    /// Guaranteed to return a valid NodeCategory, never panics.
    pub fn category(&self) -> NodeCategory;
    
    /// Returns the icon identifier for this node type.
    /// Guaranteed to return a valid non-empty string.
    pub fn icon(&self) -> &'static str;
    
    /// Returns the human-readable label for this node type.
    pub fn label(&self) -> &'static str;
    
    /// Returns the description for this node type.
    pub fn description(&self) -> &'static str;
    
    /// Returns the kebab-case node_type string.
    pub fn node_type(&self) -> &'static str;
}

impl TryFrom<&str> for WorkflowNode {
    type Error = WorkflowNodeError;
    
    fn try_from(s: &str) -> Result<Self, Self::Error>;
}

impl std::fmt::Display for WorkflowNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
```

### Config Structs (Examples)

```rust
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HttpHandlerConfig {
    pub configured: bool,
    pub status: Option<String>,
    pub journal_index: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CronTriggerConfig {
    #[serde(default)]
    pub cron_expression: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ServiceCallConfig {
    #[serde(default)]
    pub durable_step_name: String,
    #[serde(default)]
    pub target_service: String,
    #[serde(default)]
    pub target_handler: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeoutConfig {
    #[serde(default)]
    pub timeout_ms: u64,
}

// ... 20 more config structs
```

### Runtime State (Separate)

```rust
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeRuntimeState {
    pub status: NodeStatus,
    pub configured: bool,
    pub journal_index: Option<u64>,
    pub retry_count: u64,
    pub idempotency_key: Option<String>,
    pub last_output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub executing: bool,
    pub skipped: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum NodeStatus {
    #[default]
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}
```

## Non-goals

- [ ] Validation of config field values (e.g., cron expression syntax) - future bead
- [ ] Migration from existing `Node` struct - separate bead (bd-39r)
- [ ] Updating flow_extender to use enum matching - separate bead (bd-39s)
- [ ] Updating config panel to use typed accessors - separate bead (bd-39t)
- [ ] Generic `NodeConfig` trait - deferred pending use cases
