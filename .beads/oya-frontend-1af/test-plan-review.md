# Test Plan Review: oya-frontend-1af (REVIEW 2)

## VERDICT: REJECTED

---

## Executive Summary

The revised test plan for `oya-frontend-1af` has **addressed most of the original defects** from the first review, but **one CRITICAL LETHAL GAP** remains that would allow production bugs to slip through.

**Original defects status:**
| Defect | Status |
|--------|--------|
| Missing exact `ClientType` variant tests | ✅ FIXED |
| Missing exact `ContextTrait` variant tests | ✅ FIXED |
| Missing `ConnectionError::ServiceKindIncompatible` test | ✅ FIXED |
| Missing `ConnectionError::NodeNotFound` test | ✅ FIXED |
| Test density below threshold | ✅ FIXED (now 11.25×) |
| Missing proptest commutativity | ✅ FIXED |
| Missing `PortType::FlowControl` Display assertion | ✅ FIXED |
| Missing boundary tests | ✅ FIXED |

**NEW LETHAL FINDING:**
- ❌ `get_node_by_id()` function from contract has NO test coverage

---

## Axis 1 — Contract Parity

### LETHAL FINDING: Missing `get_node_by_id` Test Coverage

**Contract Section 11.1** (line 1207):
```rust
pub fn get_node_by_id(id: NodeId, nodes: &[Node]) -> Result<&Node, ConnectionError>
```

**Test Plan Coverage:**
- `get_node_by_id` is **COMPLETELY MISSING** from the test plan
- No BDD scenario exists for this function
- No unit test listed in the trophy allocation table

**Rule Violation**: Any `pub fn` in contract with no BDD scenario in test-plan = **LETHAL**.

**Required Test Scenarios:**
```
### Behavior: get_node_by_id finds existing node
Given: a node slice containing a node with ID "node-123"
When: get_node_by_id(NodeId::new("node-123"), nodes) is called
Then: Ok(&node_with_id_123) is returned

### Behavior: get_node_by_id rejects non-existent node
Given: a node slice NOT containing a node with ID "nonexistent"
When: get_node_by_id(NodeId::new("nonexistent"), nodes) is called
Then: Err(ConnectionError::NodeNotFound { node_id: NodeId::new("nonexistent") }) is returned
```

---

## Axis 2 — Assertion Sharpness

### PASS: All assertions are now exact variants

The revised plan correctly asserts:
- Exact `ClientType` variants in `available_clients()` (lines 83-85, 902-912)
- Exact `ContextTrait` variants in `available_traits()` (lines 117-118, 1060-1080)
- Exact error variants in `ConnectionError` scenarios (lines 362-375, 1778-1819)
- Exact `PortType::FlowControl` Display value `"flow-control"` (line 143, 1194)

**No `is_ok()` or `is_err()` vagueness remains.**

---

## Axis 3 — Trophy Allocation

### PASS: Test density now exceeds threshold

**Contract Public Functions** (from contract.md):
1. `ServiceKind::supports_state()` (line 229)
2. `ServiceKind::supports_promises()` (line 235)
3. `ServiceKind::context_type()` (line 241)
4. `ServiceKind::available_clients()` (line 250)
5. `ContextType::is_synchronous()` (line 329)
6. `ContextType::is_asynchronous()` (line 335)
7. `ContextType::available_traits()` (line 341)
8. `types_compatible()` (line 464)
9. `check_connection()` (line 487)
10. `WorkflowNode::service_kind()` (line 536)
11. `WorkflowNode::context_type()` (line 579)
12. `WorkflowNode::output_port_type()` (line 591)
13. `WorkflowNode::input_port_type()` (line 630)
14. `get_node_by_id()` (line 1207)
15. `ServiceKind::FromStr` (line 198)
16. `ServiceKind::Display` (line 212)
17. `ContextType::FromStr` (line 300)
18. `ContextType::Display` (line 313)
19. `PortType::FromStr` (line 414)
20. `PortType::Display` (line 430)
21. `PortType::Default` (line 443)

**Total Public API Points:** 21

**Unit Test Count in Plan:** 225 (Tier 1 table, lines 396-620)

**Ratio:** 225 / 21 = **10.7×** ✅ (exceeds 5× requirement)

---

## Axis 4 — Boundary Completeness

### PASS: All boundary tests now present

**ServiceKind Boundaries** (Plan lines 50-54):
- ✅ Empty string rejection
- ✅ Invalid string rejection
- ✅ Similar invalid "service" rejection
- ✅ Similar invalid "object" rejection
- ✅ 1KB+ string boundary

**ContextType Boundaries** (Plan lines 96-99):
- ✅ Empty string rejection
- ✅ Invalid string rejection
- ✅ Similar invalid "synced" rejection
- ✅ 1KB+ string boundary

**PortType Boundaries** (Plan lines 132-135):
- ✅ Empty string rejection
- ✅ Invalid string rejection
- ✅ Truncated "flow" rejection
- ✅ 1KB+ string boundary

---

## Axis 5 — Mutation Survivability

### PASS: Mutation checkpoints properly specified

The plan's Section 7 (lines 2073-2083) includes:
- `types_compatible()` Any/Json mutations
- `ServiceKind` capability mutations
- `WorkflowNode` mapping mutations
- `PortType::FlowControl` Display mutation
- `available_clients()` variant mutations
- `available_traits()` variant mutations

**Kill rate target:** ≥90% ✅

---

## Axis 6 — Holzmann Plan Audit

### PASS: Plan structure meets requirements

**Rule 5 — State Your Assumptions:**
- BDD scenarios now explicitly state preconditions (e.g., "Given: a valid ServiceKind string 'handler'")

**Rule 2 — Bound Every Loop:**
- Proptest bounds explicitly stated (e.g., `proptest::collection::vec(..., 1..100)`)

---

## LETHAL FINDINGS (Summary)

| # | Finding | Severity | Location |
|---|---------|----------|----------|
| 1 | `get_node_by_id()` function has no test coverage | LETHAL | Contract: 1207, Plan: MISSING |

---

## MAJOR FINDINGS (0)

All major findings from the original review have been resolved:
- ✅ Exact variant tests for `ClientType` and `ContextTrait`
- ✅ `ConnectionError::ServiceKindIncompatible` test
- ✅ `ConnectionError::NodeNotFound` test
- ✅ Test density increased to 10.7×
- ✅ Proptest commutativity invariant added
- ✅ `PortType::FlowControl` Display assertion
- ✅ Boundary tests for invalid strings and long strings

---

## MINOR FINDINGS (0)

All minor findings from the original review have been resolved:
- ✅ BDD scenarios have explicit precondition statements
- ✅ Proptest bounds explicitly documented

---

## MANDATE

**Before resubmission, the following MUST be added to the test plan:**

### Required Test Functions (LETHAL fix)

1. **`get_node_by_id_find_existing_node()`**
   - Given: node slice with node ID "node-123"
   - When: `get_node_by_id(NodeId::new("node-123"), nodes)`
   - Then: `Ok(&node_with_id_123)` is returned

2. **`get_node_by_id_rejects_nonexistent()`**
   - Given: node slice without node ID "nonexistent"
   - When: `get_node_by_id(NodeId::new("nonexistent"), nodes)`
   - Then: `Err(ConnectionError::NodeNotFound { node_id: NodeId::new("nonexistent") })` is returned

### Required Sections in Plan

Add to **Section 1 — Behavior Inventory**:
```
### get_node_by_id Behaviors

1. get_node_by_id finds existing node → returns Ok(node) when node exists
2. get_node_by_id rejects non-existent node → returns Err(NodeNotFound) when node missing
```

Add to **Section 2 — Trophy Allocation**:
```
| get_node_by_id (find existing) | `get_node_by_id_find_existing_node()` | Unit | Exact variant |
| get_node_by_id (reject missing) | `get_node_by_id_rejects_nonexistent()` | Unit | Exact variant |
```

Add to **Section 3 — BDD Scenarios**:
```
### Behavior: get_node_by_id_find_existing_node
Given: a node slice containing a node with ID "node-123"
When: get_node_by_id(NodeId::new("node-123"), nodes) is called
Then: Ok(&node_with_id_123) is returned

### Behavior: get_node_by_id_rejects_nonexistent
Given: a node slice NOT containing a node with ID "nonexistent"
When: get_node_by_id(NodeId::new("nonexistent"), nodes) is called
Then: Err(ConnectionError::NodeNotFound { node_id: NodeId::new("nonexistent") }) is returned
```

---

## RESUBMISSION REQUIREMENTS

After implementing the fix above:

1. **Re-run full audit from Tier 0** — Verify no new gaps introduced
2. **Verify `get_node_by_id` coverage**: 2 tests (find + reject)
3. **Verify total test count**: Still ≥100 unit tests (225)
4. **Verify all contract functions tested**: 21/21 public API points covered
5. **Verify all error variants**: All 4 `ConnectionError` variants tested
6. **Verify proptest invariants**: Commutativity + all other properties

**DO NOT RESUBMIT until `get_node_by_id` test coverage is added.**

---

## STATUS: REJECTED (1 LETHAL finding — `get_node_by_id` missing)
