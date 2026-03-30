# Martin Fowler Test Plan: ServiceKind and ContextType

## Overview
This test plan provides executable specifications for the ServiceKind and ContextType enums and their integration with WorkflowNode. Each test follows the Given-When-Then pattern and describes **behavior**, not implementation.

---

## Happy Path Tests

### ServiceKind Enum

```
given_valid_kebab_case_string_when_parsing_service_kind_then_returns_correct_variant
```

**Given**: strings "service", "virtual-object", "workflow"
**When**: parsing with `ServiceKind::from_str()`
**Then**: returns `Ok(ServiceKind::Service)`, `Ok(ServiceKind::VirtualObject)`, `Ok(ServiceKind::Workflow)`

---

```
given_service_kind_variant_when_serializing_then_produces_kebab_case_json
```

**Given**: `ServiceKind::VirtualObject`
**When**: serializing to JSON
**Then**: produces `"virtual-object"`

---

```
given_service_kind_when_roundtrip_through_json_then_preserves_variant
```

**Given**: any `ServiceKind` variant
**When**: serialize to JSON, then deserialize
**Then**: result equals original variant

---

### ContextType Enum

```
given_valid_kebab_case_string_when_parsing_context_type_then_returns_correct_variant
```

**Given**: strings "service", "object-exclusive", "object-shared", "workflow-exclusive", "workflow-shared"
**When**: parsing with `ContextType::from_str()`
**Then**: returns corresponding `Ok(ContextType::* )` variant

---

```
given_context_type_variant_when_serializing_then_produces_kebab_case_json
```

**Given**: `ContextType::ObjectExclusive`
**When**: serializing to JSON
**Then**: produces `"object-exclusive"`

---

```
given_context_type_when_roundtrip_through_json_then_preserves_variant
```

**Given**: any `ContextType` variant
**When**: serialize to JSON, then deserialize
**Then**: result equals original variant

---

```
given_context_type_when_getting_service_kind_then_returns_owning_kind
```

**Given**: 
  - `ContextType::Service`
  - `ContextType::ObjectExclusive` or `ObjectShared`
  - `ContextType::WorkflowExclusive` or `WorkflowShared`
**When**: calling `.service_kind()`
**Then**: returns `ServiceKind::Service`, `ServiceKind::VirtualObject`, `ServiceKind::Workflow` respectively

---

### WorkflowNode::compatible_service_kinds()

```
given_entry_node_when_getting_compatible_service_kinds_then_returns_all_three
```

**Given**: `WorkflowNode::HttpHandler`, `KafkaHandler`, `CronTrigger`, or `WorkflowSubmit`
**When**: calling `compatible_service_kinds()`
**Then**: returns slice containing `[ServiceKind::Service, ServiceKind::VirtualObject, ServiceKind::Workflow]`

---

```
given_state_node_when_getting_compatible_service_kinds_then_returns_virtual_object_and_workflow
```

**Given**: `WorkflowNode::GetState`, `SetState`, or `ClearState`
**When**: calling `compatible_service_kinds()`
**Then**: returns slice containing `[ServiceKind::VirtualObject, ServiceKind::Workflow]`
**And**: slice does NOT contain `ServiceKind::Service`

---

```
given_promise_node_when_getting_compatible_service_kinds_then_returns_workflow_only
```

**Given**: `WorkflowNode::DurablePromise`, `Awakeable`, or `ResolvePromise`
**When**: calling `compatible_service_kinds()`
**Then**: returns slice containing ONLY `[ServiceKind::Workflow]`

---

```
given_generic_node_when_getting_compatible_service_kinds_then_returns_all_three
```

**Given**: `WorkflowNode::Run`, `ServiceCall`, `Condition`, `Sleep`, or `SignalHandler`
**When**: calling `compatible_service_kinds()`
**Then**: returns slice containing all three ServiceKind variants

---

### WorkflowNode::required_context_types()

```
given_state_node_when_getting_required_context_types_then_returns_exclusive_only
```

**Given**: `WorkflowNode::GetState`, `SetState`, or `ClearState`
**When**: calling `required_context_types()`
**Then**: returns slice containing `[ContextType::ObjectExclusive, ContextType::WorkflowExclusive]`
**And**: slice does NOT contain `Service`, `ObjectShared`, or `WorkflowShared`

---

```
given_promise_node_when_getting_required_context_types_then_returns_workflow_exclusive_only
```

**Given**: `WorkflowNode::DurablePromise`, `Awakeable`, or `ResolvePromise`
**When**: calling `required_context_types()`
**Then**: returns slice containing ONLY `[ContextType::WorkflowExclusive]`

---

```
given_generic_node_when_getting_required_context_types_then_returns_all_five
```

**Given**: `WorkflowNode::HttpHandler`, `Run`, `Condition`, or `Sleep`
**When**: calling `required_context_types()`
**Then**: returns slice containing all five ContextType variants

---

## Error Path Tests

### ServiceKind Parsing Errors

```
given_empty_string_when_parsing_service_kind_then_returns_parse_error
```

**Given**: empty string `""`
**When**: parsing with `ServiceKind::from_str()`
**Then**: returns `Err(ParseServiceKindError(""))`

---

```
given_unknown_string_when_parsing_service_kind_then_returns_parse_error
```

**Given**: string `"unknown"`
**When**: parsing with `ServiceKind::from_str()`
**Then**: returns `Err(ParseServiceKindError("unknown"))`

---

```
given_wrong_case_string_when_parsing_service_kind_then_returns_parse_error
```

**Given**: strings `"SERVICE"`, `"VirtualObject"`, `"Workflow"`
**When**: parsing with `ServiceKind::from_str()`
**Then**: returns `Err(ParseServiceKindError)` for each

---

### ContextType Parsing Errors

```
given_invalid_string_when_parsing_context_type_then_returns_parse_error
```

**Given**: strings `""`, `"invalid"`, `"objectexclusive"`, `"WORKFLOW-EXCLUSIVE"`
**When**: parsing with `ContextType::from_str()`
**Then**: returns `Err(ParseContextTypeError)` for each

---

### JSON Deserialization Errors

```
given_json_with_unknown_service_kind_when_deserializing_then_returns_error
```

**Given**: JSON string `"\"unknown-service\""`
**When**: deserializing to `ServiceKind`
**Then**: returns deserialization error

---

```
given_json_with_unknown_context_type_when_deserializing_then_returns_error
```

**Given**: JSON string `"\"unknown-context\""`
**When**: deserializing to `ContextType`
**Then**: returns deserialization error

---

## Edge Case Tests

### Enum Cardinality

```
given_service_kind_when_counting_variants_then_equals_three
```

**Given**: `ServiceKind::all()` method
**When**: counting returned slice length
**Then**: equals 3

---

```
given_context_type_when_counting_variants_then_equals_five
```

**Given**: `ContextType::all()` method
**When**: counting returned slice length
**Then**: equals 5

---

### Method Idempotency

```
given_workflow_node_when_calling_compatible_kinds_twice_then_returns_identical_slices
```

**Given**: any `WorkflowNode` variant
**When**: calling `compatible_service_kinds()` twice
**Then**: both results have identical content and order

---

```
given_workflow_node_when_calling_required_contexts_twice_then_returns_identical_slices
```

**Given**: any `WorkflowNode` variant
**When**: calling `required_context_types()` twice
**Then**: both results have identical content and order

---

### Non-Empty Contracts

```
given_all_24_workflow_nodes_when_getting_compatible_kinds_then_none_returns_empty
```

**Given**: all 24 `WorkflowNode` variants
**When**: calling `compatible_service_kinds()` on each
**Then**: every result is a non-empty slice

---

```
given_all_24_workflow_nodes_when_getting_required_contexts_then_none_returns_empty
```

**Given**: all 24 `WorkflowNode` variants
**When**: calling `required_context_types()` on each
**Then**: every result is a non-empty slice

---

## Contract Verification Tests

### Display/FromStr Roundtrip

```
given_service_kind_when_displaying_then_output_is_parseable
```

**Given**: any `ServiceKind` variant
**When**: format with `Display`, then parse with `FromStr`
**Then**: returns the original variant

---

```
given_context_type_when_displaying_then_output_is_parseable
```

**Given**: any `ContextType` variant
**When**: format with `Display`, then parse with `FromStr`
**Then**: returns the original variant

---

### Hash/Eq Consistency

```
given_equal_service_kinds_when_hashing_then_produces_same_hash
```

**Given**: two `ServiceKind::Workflow` values
**When**: hashing both
**Then**: hashes are equal

---

```
given_equal_context_types_when_hashing_then_produces_same_hash
```

**Given**: two `ContextType::ObjectExclusive` values
**When**: hashing both
**Then**: hashes are equal

---

### Copy Trait

```
given_service_kind_when_copying_then_original_unchanged
```

**Given**: `let original = ServiceKind::VirtualObject`
**When**: `let copy = original` (copy, not move)
**Then**: both `original` and `copy` are usable and equal

---

```
given_context_type_when_copying_then_original_unchanged
```

**Given**: `let original = ContextType::WorkflowShared`
**When**: `let copy = original` (copy, not move)
**Then**: both `original` and `copy` are usable and equal

---

### Module Reorganization

```
given_moved_service_kind_when_importing_in_flow_extender_then_compiles
```

**Given**: `ServiceKind` defined in `src/graph/restate_types.rs`
**When**: `src/flow_extender/mod.rs` imports `use crate::graph::ServiceKind`
**Then**: code compiles successfully

---

```
given_existing_restate_service_kind_references_when_renamed_then_uses_new_name
```

**Given**: existing code using `RestateServiceKind`
**When**: replacing with `ServiceKind`
**Then**: all usages compile and behave identically

---

## Test File Organization

```rust
// src/graph/restate_types.rs

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    mod service_kind {
        mod happy_path { /* 4 tests */ }
        mod error_path { /* 4 tests */ }
        mod edge_case { /* 2 tests */ }
        mod contract { /* 3 tests */ }
    }
    
    mod context_type {
        mod happy_path { /* 5 tests */ }
        mod error_path { /* 4 tests */ }
        mod edge_case { /* 2 tests */ }
        mod contract { /* 3 tests */ }
    }
}

// src/graph/workflow_node.rs - additions to existing tests module

#[cfg(test)]
mod tests {
    // ... existing tests ...
    
    mod restate_compatibility {
        mod compatible_service_kinds {
            mod happy_path { /* 7 tests by category */ }
            mod edge_case { /* 2 tests */ }
        }
        
        mod required_context_types {
            mod happy_path { /* 5 tests by category */ }
            mod edge_case { /* 2 tests */ }
        }
    }
}
```

## Test Count Summary

| Category | ServiceKind | ContextType | WorkflowNode Methods | Total |
|----------|-------------|-------------|---------------------|-------|
| Happy Path | 4 | 5 | 12 | 21 |
| Error Path | 4 | 4 | 0 | 8 |
| Edge Case | 2 | 2 | 4 | 8 |
| Contract | 3 | 3 | 0 | 6 |
| **Total** | **13** | **14** | **16** | **43** |

---

## Acceptance Criteria

- [ ] All 43 tests pass
- [ ] `moon run :clippy` passes with no warnings
- [ ] `moon run :fmt -- --check` passes
- [ ] ServiceKind moved from flow_extender to graph/restate_types.rs
- [ ] All existing flow_extender tests still pass
