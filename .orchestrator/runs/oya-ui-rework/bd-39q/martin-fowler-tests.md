# Martin Fowler Test Plan: WorkflowNode Enum

> Tests are executable specifications. Each test name describes behavior, not implementation.

## Test File Structure

```
src/graph/
├── workflow_node.rs          # Production code
├── workflow_node_test.rs     # Unit tests
├── workflow_node_serde_test.rs  # Serialization tests
└── workflow_node_migrate_test.rs # Migration tests (optional, may be separate bead)
```

---

## Happy Path Tests

### Core Variant Tests

```
test_given_valid_node_type_string_when_parsing_then_returns_correct_variant
```
- Given: A valid kebab-case node_type string
- When: The string is parsed via `TryFrom<&str>`
- Then: Returns `Ok(WorkflowNode)` with correct variant

```
test_given_all_24_node_types_when_parsing_each_then_all_succeed
```
- Given: All 24 node_type strings from NODE_TEMPLATES
- When: Each string is parsed
- Then: All 24 return `Ok(WorkflowNode)`, no errors

```
test_given_workflow_node_variant_when_displaying_then_outputs_kebab_case
```
- Given: Any WorkflowNode variant
- When: `format!("{}", variant)` is called
- Then: Output is kebab-case string (e.g., "http-handler")

```
test_given_workflow_node_variant_when_getting_node_type_then_matches_display
```
- Given: Any WorkflowNode variant
- When: `variant.node_type()` is called
- Then: Returns same string as `format!("{}", variant)`

### Category Derivation Tests

```
test_given_entry_variants_when_getting_category_then_returns_entry
```
- Given: HttpHandler, KafkaHandler, CronTrigger, WorkflowSubmit variants
- When: `category()` is called on each
- Then: All return `NodeCategory::Entry`

```
test_given_durable_variants_when_getting_category_then_returns_durable
```
- Given: Run, ServiceCall, ObjectCall, WorkflowCall, SendMessage, DelayedSend variants
- When: `category()` is called on each
- Then: All return `NodeCategory::Durable`

```
test_given_state_variants_when_getting_category_then_returns_state
```
- Given: GetState, SetState, ClearState variants
- When: `category()` is called on each
- Then: All return `NodeCategory::State`

```
test_given_flow_variants_when_getting_category_then_returns_flow
```
- Given: Condition, Switch, Loop, Parallel, Compensate variants
- When: `category()` is called on each
- Then: All return `NodeCategory::Flow`

```
test_given_timing_variants_when_getting_category_then_returns_timing
```
- Given: Sleep, Timeout variants
- When: `category()` is called on each
- Then: All return `NodeCategory::Timing`

```
test_given_signal_variants_when_getting_category_then_returns_signal
```
- Given: DurablePromise, Awakeable, ResolvePromise, SignalHandler variants
- When: `category()` is called on each
- Then: All return `NodeCategory::Signal`

### Icon Derivation Tests

```
test_given_http_handler_variant_when_getting_icon_then_returns_globe
```
- Given: `WorkflowNode::HttpHandler(_)`
- When: `icon()` is called
- Then: Returns `"globe"`

```
test_given_kafka_handler_variant_when_getting_icon_then_returns_kafka
```
- Given: `WorkflowNode::KafkaHandler(_)`
- When: `icon()` is called
- Then: Returns `"kafka"`

```
test_given_cron_trigger_variant_when_getting_icon_then_returns_clock
```
- Given: `WorkflowNode::CronTrigger(_)`
- When: `icon()` is called
- Then: Returns `"clock"`

```
test_given_all_variants_when_getting_icon_then_all_return_non_empty_string
```
- Given: All 24 WorkflowNode variants
- When: `icon()` is called on each
- Then: All return non-empty `&'static str`

---

## Error Path Tests

### Parsing Errors

```
test_given_empty_string_when_parsing_then_returns_unknown_type_error
```
- Given: Empty string `""`
- When: `WorkflowNode::try_from("")` is called
- Then: Returns `Err(WorkflowNodeError::UnknownNodeType)` with empty string in message

```
test_given_unknown_node_type_when_parsing_then_returns_unknown_type_error
```
- Given: String `"foo-bar"`
- When: `WorkflowNode::try_from("foo-bar")` is called
- Then: Returns `Err(WorkflowNodeError::UnknownNodeType)` with `"foo-bar"` in message

```
test_given_case_mismatched_node_type_when_parsing_then_returns_error
```
- Given: String `"HTTP-HANDLER"` (uppercase)
- When: `WorkflowNode::try_from()` is called
- Then: Returns `Err(WorkflowNodeError::UnknownNodeType)`

```
test_given_whitespace_only_string_when_parsing_then_returns_error
```
- Given: String `"   "`
- When: `WorkflowNode::try_from()` is called
- Then: Returns `Err(WorkflowNodeError::UnknownNodeType)`

### Deserialization Errors

```
test_given_json_without_type_field_when_deserializing_then_returns_error
```
- Given: JSON `{"config": {}}`
- When: Deserialized into `WorkflowNode`
- Then: Returns `Err` with message about missing type field

```
test_given_json_with_unknown_type_when_deserializing_then_returns_error
```
- Given: JSON `{"type": "unknown-type"}`
- When: Deserialized into `WorkflowNode`
- Then: Returns `Err` with message about unknown type

```
test_given_invalid_json_syntax_when_deserializing_then_returns_error
```
- Given: Invalid JSON `{"type": "http-handler"` (missing closing brace)
- When: Deserialized into `WorkflowNode`
- Then: Returns `Err` containing the parse error

---

## Edge Case Tests

### Config Defaults

```
test_given_default_config_for_http_handler_when_serializing_then_produces_valid_json
```
- Given: `HttpHandlerConfig::default()`
- When: Serialized to JSON
- Then: Produces valid JSON `{"type":"http-handler",...}`

```
test_given_empty_config_struct_when_creating_variant_then_succeeds
```
- Given: Any config struct with all default values
- When: Used to create a WorkflowNode variant
- Then: Variant is created successfully

```
test_given_all_config_structs_when_checking_default_then_all_implement_default
```
- Given: All 24 config struct types
- When: `Default::default()` is called
- Then: All compile and return valid defaults

### Unicode and Special Characters

```
test_given_config_with_unicode_string_when_roundtripping_then_preserves_unicode
```
- Given: Config with unicode `"日本語"`
- When: Serialized and deserialized
- Then: Unicode is preserved exactly

```
test_given_config_with_emoji_when_roundtripping_then_preserves_emoji
```
- Given: Config with emoji `"🎉"`
- When: Serialized and deserialized
- Then: Emoji is preserved exactly

### Boundary Values

```
test_given_config_with_max_u64_value_when_roundtripping_then_preserves_value
```
- Given: `timeout_ms: u64::MAX`
- When: Serialized and deserialized
- Then: Value is preserved exactly

```
test_given_config_with_empty_string_field_when_roundtripping_then_preserves_empty
```
- Given: Empty string field `cron_expression: ""`
- When: Serialized and deserialized
- Then: Empty string is preserved

```
test_given_config_with_very_long_string_when_serializing_then_succeeds
```
- Given: String field with 64KB of text
- When: Serialized
- Then: Serialization succeeds without error

---

## Contract Verification Tests

### Round-Trip Serialization

```
test_given_http_handler_with_full_config_when_roundtripping_then_preserves_all_data
```
- Given: `HttpHandler` with all config fields populated
- When: Serialized to JSON then deserialized back
- Then: All fields match original exactly

```
test_given_all_24_variants_when_roundtripping_each_then_all_preserve_data
```
- Given: All 24 variants with representative config
- When: Each is round-tripped through JSON
- Then: All variants preserve their config data

### Invariant Tests

```
test_given_variant_count_when_comparing_to_templates_then_count_is_24
```
- Given: WorkflowNode enum definition
- When: Variant count is checked
- Then: Count equals 24 (number of NODE_TEMPLATES)

```
test_given_any_variant_when_calling_category_then_never_panics
```
- Given: Any valid WorkflowNode variant
- When: `category()` is called
- Then: Returns valid NodeCategory, never panics

```
test_given_any_variant_when_calling_icon_then_never_panics
```
- Given: Any valid WorkflowNode variant
- When: `icon()` is called
- Then: Returns valid `&'static str`, never panics

### Serde Format Tests

```
test_given_any_variant_when_serializing_then_json_has_type_field
```
- Given: Any WorkflowNode variant
- When: Serialized to JSON
- Then: JSON contains `"type"` field (not `"node_type"`)

```
test_given_any_variant_when_serializing_then_type_is_kebab_case
```
- Given: Any WorkflowNode variant
- When: Serialized to JSON
- Then: `"type"` value is kebab-case (e.g., `"http-handler"`)

---

## Migration Tests (for localStorage Compatibility)

```
test_given_legacy_json_node_when_migrating_then_produces_workflow_node
```
- Given: Old format JSON `{"node_type":"http-handler","config":{...}}`
- When: Migration function is called
- Then: Produces new format with `WorkflowNode` variant

```
test_given_workflow_with_multiple_nodes_when_migrating_then_all_nodes_preserved
```
- Given: Old workflow JSON with 10 nodes
- When: Migrated to new format
- Then: All 10 nodes are present and valid

```
test_given_migrated_workflow_when_serializing_then_can_deserialize_again
```
- Given: A migrated workflow
- When: Serialized and deserialized
- Then: Round-trip succeeds with no data loss

---

## Given-When-Then Scenarios

### Scenario 1: Creating a Cron Trigger Node

```
Given: A user wants to create a cron trigger node
When: 
  - The node_type string "cron-trigger" is parsed
  - A default CronTriggerConfig is created
  - The variant is wrapped in WorkflowNode::CronTrigger(config)
Then:
  - The variant's category() returns NodeCategory::Entry
  - The variant's icon() returns "clock"
  - The variant's node_type() returns "cron-trigger"
  - Serialization produces {"type":"cron-trigger","cron_expression":""}
```

### Scenario 2: Parsing Invalid Input

```
Given: A string "invalid-type" is received (e.g., from corrupted localStorage)
When: WorkflowNode::try_from("invalid-type") is called
Then:
  - Returns Err(WorkflowNodeError::UnknownNodeType)
  - Error message contains "invalid-type"
  - Error message lists valid types or provides hint
  - No panic occurs
```

### Scenario 3: Round-Trip Through JSON

```
Given: A ServiceCall node with config:
  - durable_step_name: "fetch-user"
  - target_service: "UserService"
  - target_handler: "getUser"
When:
  - Node is serialized to JSON
  - JSON is deserialized back
Then:
  - Deserialized node equals original
  - All config fields match exactly
  - type field is "service-call"
```

### Scenario 4: Category Derivation

```
Given: All 24 WorkflowNode variants exist
When: category() is called on each variant
Then:
  - Entry category: 4 variants
  - Durable category: 6 variants
  - State category: 3 variants
  - Flow category: 5 variants
  - Timing category: 2 variants
  - Signal category: 4 variants
  - Total: 24 variants, all categorized
```

### Scenario 5: Icon Derivation

```
Given: The HttpHandler variant
When: icon() is called
Then:
  - Returns "globe" (matching NODE_TEMPLATES)
  - Returns &'static str (no allocation)
  - Same value on every call
```

---

## Test Coverage Requirements

| Category | Minimum Tests |
|----------|---------------|
| Happy Path | 10+ |
| Error Path | 5+ |
| Edge Cases | 5+ |
| Contract Verification | 8+ |
| **Total** | **28+** |

## Quality Gates

1. **All tests must pass** before implementation is considered complete
2. **No panics** - every error path returns `Err`, never panics
3. **100% variant coverage** - all 24 variants have at least one test each
4. **Round-trip proven** - serialization tests for all variants
5. **Error messages are useful** - errors contain context for debugging
