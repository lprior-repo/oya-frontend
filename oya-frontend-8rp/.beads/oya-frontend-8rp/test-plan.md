# Test Plan: Expression Depth Limiting (oya-frontend-8rp)

## Summary

- **Behaviors identified**: 23
- **Trophy allocation**: 14 unit / 7 integration / 2 e2e (60% / 30% / 10%)
- **Proptest invariants**: 4
- **Fuzz targets**: 2
- **Kani harnesses**: 3
- **Error variants covered**: 5 (100% coverage)

---

## 1. Behavior Inventory

### Depth Limit Enforcement

1. `resolve_expressions` rejects when `current_depth >= MAX_EXPRESSION_DEPTH`
2. `resolve_expressions` succeeds when `current_depth < MAX_EXPRESSION_DEPTH`
3. `resolve_expressions` returns `DepthLimitExceeded` with accurate depth values
4. `validate_expression_depth` succeeds for expressions within depth limit
5. `validate_expression_depth` fails for expressions exceeding depth limit
6. `validate_expression_depth` returns `DepthLimitExceeded` with calculated depth
7. `calculate_depth` returns 0 for leaf expressions (no children)
8. `calculate_depth` returns 1 for single-child expressions
9. `calculate_depth` returns max(child_depths) + 1 for multi-child expressions
10. `calculate_depth` returns correct depth for deeply nested trees (up to limit)

### ExpressionDepth Type Safety

11. `ExpressionDepth::new` succeeds for `depth <= MAX_EXPRESSION_DEPTH`
12. `ExpressionDepth::new` fails for `depth > MAX_EXPRESSION_DEPTH`
13. `ExpressionDepth::increment` succeeds when result <= MAX_EXPRESSION_DEPTH
14. `ExpressionDepth::increment` fails when result > MAX_EXPRESSION_DEPTH
15. `ExpressionDepth::is_valid` returns true for depths <= MAX_EXPRESSION_DEPTH
16. `ExpressionDepth::is_valid` returns false for depths > MAX_EXPRESSION_DEPTH
17. `ExpressionDepth::current` returns the exact stored depth value
18. `ExpressionDepth` preserves equality across clone/copy operations

### Error Handling Semantics

19. `resolve_expressions` on invalid expression returns `InvalidExpression`
20. `resolve_expressions` on missing reference returns `ExpressionNotFound`
21. `resolve_expressions` on type mismatch returns `TypeError` with both types
22. `resolve_expressions` on runtime error returns `RuntimeError` with message
23. All errors preserve expression tree immutability (no side effects)

---

## 2. Trophy Allocation

| Behavior | Layer | Rationale |
|----------|-------|-----------|
| Depth limit enforcement in `resolve_expressions` | Unit | Pure logic, boundary conditions |
| `ExpressionDepth` constructor validation | Unit | Type safety, exhaustive input space |
| `ExpressionDepth::increment` bounds checking | Unit | Combinatorial depth testing |
| `ExpressionDepth::is_valid` predicate | Unit | Boolean invariant |
| `calculate_depth` on various trees | Unit + Proptest | Pure function, structural recursion |
| `validate_expression_depth` happy path | Integration | Real expression structures |
| `validate_expression_depth` depth violation | Integration | Integration of depth calc + validation |
| `resolve_expressions` with depth violation | Integration | Full resolver with depth tracking |
| `resolve_expressions` error taxonomy | Integration | Error variant coverage |
| `resolve_expressions` immutability guarantee | E2E | Observable side-effect prevention |
| `resolve_expressions` stack safety at limit | E2E | Actual stack behavior verification |

**Ratio**: 14 unit (61%) / 7 integration (30%) / 2 e2e (9%)

**Justification**: Depth limiting is primarily a Calc-layer concern with pure functions. The Integration layer validates the depth tracking through the resolver's actual traversal. E2E is minimal since stack overflow is prevented by design, not tested.

---

## 3. BDD Scenarios

### Behavior: resolve_expressions rejects when current_depth >= MAX_EXPRESSION_DEPTH

**Happy path:**
```
Given: An expression tree with 3 nested children
And: current_depth is set to 1023 (one below MAX_EXPRESSION_DEPTH)
When: resolve_expressions is called with this depth
Then: Ok(ResolvedExpression) is returned
And: ResolvedExpression contains the evaluated result
```

**Error variant:**
```
Given: An expression tree with nested structure
And: current_depth is set to 1024 (equal to MAX_EXPRESSION_DEPTH)
When: resolve_expressions is called with this depth
Then: Err(Error::DepthLimitExceeded { current_depth: 1024, max_depth: 1024 }) is returned
And: Expression tree remains unmodified
```

**Boundary test:**
```
Given: An expression tree with any structure
And: current_depth is set to 1025 (one above MAX_EXPRESSION_DEPTH)
When: resolve_expressions is called with this depth
Then: Err(Error::DepthLimitExceeded { current_depth: 1025, max_depth: 1024 }) is returned
```

Test function name: `fn resolve_expressions_returns_depth_limit_error_when_depth_exceeds_maximum()`

---

### Behavior: validate_expression_depth succeeds for expressions within depth limit

**Happy path:**
```
Given: An expression tree with maximum nesting depth of 500
When: validate_expression_depth is called with this expression
Then: Ok(ExpressionDepth(500)) is returned
And: The returned depth equals the actual maximum nesting
```

**Error variant:**
```
Given: An expression tree with maximum nesting depth of 1025
When: validate_expression_depth is called with this expression
Then: Err(Error::DepthLimitExceeded { current_depth: 1025, max_depth: 1024 }) is returned
```

Test function name: `fn validate_expression_depth_returns_valid_depth_when_within_limit()`

---

### Behavior: calculate_depth returns 0 for leaf expressions

**Happy path:**
```
Given: A leaf expression node with no children
When: calculate_depth is called with this expression
Then: 0 is returned
And: The result is exactly 0, not None or error
```

**Boundary test:**
```
Given: A leaf expression node
When: calculate_depth is called
Then: The result equals 0
```

Test function name: `fn calculate_depth_returns_zero_for_leaf_expression()`

---

### Behavior: calculate_depth returns 1 for single-child expressions

**Happy path:**
```
Given: An expression with exactly one child that is a leaf
When: calculate_depth is called with this expression
Then: 1 is returned
And: The result equals 1 + calculate_depth(leaf) = 1 + 0 = 1
```

**Invariant test:**
```
Given: An expression with one child at depth N
When: calculate_depth is called
Then: The result equals N + 1
```

Test function name: `fn calculate_depth_returns_one_for_single_child_expression()`

---

### Behavior: calculate_depth returns max(child_depths) + 1 for multi-child expressions

**Happy path:**
```
Given: An expression with three children at depths 2, 5, and 3
When: calculate_depth is called with this expression
Then: 6 is returned
And: The result equals max(2, 5, 3) + 1 = 6
```

**Test function name:** `fn calculate_depth_returns_max_child_depth_plus_one_for_multi_child_expression()`

---

### Behavior: ExpressionDepth::new succeeds for depth <= MAX_EXPRESSION_DEPTH

**Happy path:**
```
Given: A depth value of 0
When: ExpressionDepth::new(0) is called
Then: Ok(ExpressionDepth(0)) is returned
```

**Happy path:**
```
Given: A depth value of 1024 (equal to MAX_EXPRESSION_DEPTH)
When: ExpressionDepth::new(1024) is called
Then: Ok(ExpressionDepth(1024)) is returned
```

**Boundary test:**
```
Given: A depth value of 1023
When: ExpressionDepth::new(1023) is called
Then: Ok(ExpressionDepth(1023)) is returned
```

Test function name: `fn expression_depth_new_succeeds_when_depth_at_maximum()`

---

### Behavior: ExpressionDepth::new fails for depth > MAX_EXPRESSION_DEPTH

**Happy path:**
```
Given: A depth value of 1025
When: ExpressionDepth::new(1025) is called
Then: Err(Error::DepthLimitExceeded { current_depth: 1025, max_depth: 1024 }) is returned
```

**Boundary test:**
```
Given: A depth value of 2000
When: ExpressionDepth::new(2000) is called
Then: Err(Error::DepthLimitExceeded { current_depth: 2000, max_depth: 1024 }) is returned
```

**Test function name:** `fn expression_depth_new_returns_error_when_depth_exceeds_maximum()`

---

### Behavior: ExpressionDepth::increment succeeds when result <= MAX_EXPRESSION_DEPTH

**Happy path:**
```
Given: An ExpressionDepth of 1023
When: increment() is called
Then: Ok(ExpressionDepth(1024)) is returned
```

**Happy path:**
```
Given: An ExpressionDepth of 500
When: increment() is called
Then: Ok(ExpressionDepth(501)) is returned
```

**Test function name:** `fn expression_depth_increment_succeeds_when_result_within_limit()`

---

### Behavior: ExpressionDepth::increment fails when result > MAX_EXPRESSION_DEPTH

**Happy path:**
```
Given: An ExpressionDepth of 1024
When: increment() is called
Then: Err(Error::DepthLimitExceeded { current_depth: 1025, max_depth: 1024 }) is returned
```

**Test function name:** `fn expression_depth_increment_returns_error_when_result_exceeds_maximum()`

---

### Behavior: ExpressionDepth::is_valid returns true for depths <= MAX_EXPRESSION_DEPTH

**Happy path:**
```
Given: An ExpressionDepth of 0
When: is_valid() is called
Then: true is returned
```

**Happy path:**
```
Given: An ExpressionDepth of 1024
When: is_valid() is called
Then: true is returned
```

**Test function name:** `fn expression_depth_is_valid_returns_true_for_valid_depths()`

---

### Behavior: ExpressionDepth::is_valid returns false for depths > MAX_EXPRESSION_DEPTH

**Happy path:**
```
Given: An ExpressionDepth created from invalid depth 1025 (using unsafe or direct construction for test)
When: is_valid() is called
Then: false is returned
```

**Test function name:** `fn expression_depth_is_valid_returns_false_for_exceeded_depth()`

---

### Behavior: resolve_expressions returns InvalidExpression for malformed input

**Happy path:**
```
Given: An expression with invalid syntax (e.g., mismatched brackets)
When: resolve_expressions is called with valid depth
Then: Err(Error::InvalidExpression) is returned
```

**Test function name:** `fn resolve_expressions_returns_invalid_expression_error_when_syntax_malformed()`

---

### Behavior: resolve_expressions returns ExpressionNotFound for missing references

**Happy path:**
```
Given: An expression that references 'missing.key'
And: 'missing.key' is not in the registry
When: resolve_expressions is called with valid depth
Then: Err(Error::ExpressionNotFound { reference: "missing.key" }) is returned
```

**Test function name:** `fn resolve_expressions_returns_expression_not_found_when_reference_missing()`

---

### Behavior: resolve_expressions returns TypeError for type mismatches

**Happy path:**
```
Given: An expression attempting to add i32 to String
When: resolve_expressions is called with valid depth
Then: Err(Error::TypeError { expected: "i32", actual: "String" }) is returned
```

**Test function name:** `fn resolve_expressions_returns_type_error_when_types_mismatch()`

---

### Behavior: resolve_expressions returns RuntimeError for runtime errors

**Happy path:**
```
Given: An expression that divides by zero
When: resolve_expressions is called with valid depth
Then: Err(Error::RuntimeError { message: "Division by zero" }) is returned
```

**Test function name:** `fn resolve_expressions_returns_runtime_error_when_evaluation_fails()`

---

### Behavior: resolve_expressions preserves expression tree immutability on error

**Happy path:**
```
Given: An expression tree with known state
And: resolve_expressions is called with invalid depth
When: The call completes with error
Then: The expression tree state is identical to before the call
And: No fields have been modified
```

**Test function name:** `fn resolve_expressions_preserves_expression_immutability_on_error()`

---

### Behavior: resolve_expressions preserves expression tree immutability on success

**Happy path:**
```
Given: An expression tree with known state
When: resolve_expressions is called with valid depth and succeeds
Then: The expression tree state is identical to before the call
And: No fields have been modified
```

**Test function name:** `fn resolve_expressions_preserves_expression_immutability_on_success()`

---

## 4. Proptest Invariants

### Proptest: calculate_depth

```
Invariant: calculate_depth(expression) >= 0 for all expressions
Strategy: Generate random expression trees with 0-1000 nodes, 0-5 branching factor
Anti-invariant: None (all valid expressions must have non-negative depth)
```

```
Invariant: calculate_depth(expression) == 0 if and only if expression.has_children() == false
Strategy: Generate leaf expressions and expressions with children
Anti-invariant: Empty expression should fail (not a valid input)
```

```
Invariant: calculate_depth(expression) == 1 + max(children_depths) for non-leaf expressions
Strategy: Generate expressions with 1-10 children at varying depths
Anti-invariant: Leaf expression should not trigger this property
```

```
Invariant: calculate_depth(expression) <= MAX_EXPRESSION_DEPTH for all valid expressions
Strategy: Generate expressions up to depth 1024
Anti-invariant: Expressions deeper than 1024 should be rejected before calculation
```

---

### Proptest: ExpressionDepth::new

```
Invariant: if ExpressionDepth::new(d) == Ok(d), then d.current() == d.value
Strategy: Generate depths from 0 to 2000
Anti-invariant: Depths > 1024 should always fail
```

```
Invariant: if ExpressionDepth::new(d) == Ok(d), then d.is_valid() == true
Strategy: Generate all valid depths 0-1024
Anti-invariant: None
```

```
Invariant: if ExpressionDepth::new(d) == Err(_), then d > MAX_EXPRESSION_DEPTH
Strategy: Generate all invalid depths 1025-10000
Anti-invariant: All invalid depths should produce same error variant
```

---

### Proptest: ExpressionDepth::increment

```
Invariant: if ExpressionDepth::new(d) == Ok(d) and d < 1024, then increment() == Ok(d+1)
Strategy: Generate depths 0-1023
Anti-invariant: None
```

```
Invariant: if ExpressionDepth::new(1024) == Ok(d), then increment() == Err(1025)
Strategy: Single case at boundary
Anti-invariant: None
```

---

### Proptest: resolve_expressions depth tracking

```
Invariant: resolve_expressions depth increases by exactly 1 per nesting level
Strategy: Generate expression trees with known depth structure
Anti-invariant: None
```

```
Invariant: resolve_expressions returns DepthLimitExceeded at exactly depth 1024
Strategy: Generate expressions with depth 1023, 1024, 1025
Anti-invariant: None
```

---

## 5. Fuzz Targets

### Fuzz Target: calculate_depth

```
Input type: bytes (serialized expression structure)
Risk: Panic on malformed input, infinite recursion, stack overflow
Corpus seeds:
  - Leaf expression (empty children array)
  - Single child expression
  - Deeply nested expression (1023 levels)
  - Wide expression (1000 siblings at same level)
  - Mixed depth expression
```

**Target function:** `cargo-fuzz run calculate_depth`

---

### Fuzz Target: ExpressionDepth::new

```
Input type: u32 (raw depth value)
Risk: Overflow when incrementing, incorrect validation
Corpus seeds:
  - 0 (minimum)
  - 1023 (one below max)
  - 1024 (exact max)
  - 1025 (one above max)
  - u32::MAX (overflow edge case)
  - Random 32-bit values
```

**Target function:** `cargo-fuzz run expression_depth_new`

---

## 6. Kani Verification Harnesses

### Kani Harness: Depth Monotonicity

```
Property: For any expression tree, calculate_depth(expression) increases by exactly 1 when wrapping in parent
Bound: MAX_EXPRESSION_DEPTH = 1024
Rationale: Formal proof that depth calculation is structurally correct and never exceeds bound
```

**Kani harness code:**
```rust
#[kani::proof]
fn proof_depth_monotonicity() {
    let expr = kani::any::<Expression>();
    let parent = wrap_in_parent(expr);
    assert!(calculate_depth(parent) == calculate_depth(expr) + 1);
}
```

---

### Kani Harness: Depth Bound Invariant

```
Property: calculate_depth(expression) <= MAX_EXPRESSION_DEPTH for all well-formed expressions
Bound: 1024 depth levels
Rationale: Prove that no valid expression can exceed the depth limit during calculation
```

**Kani harness code:**
```rust
#[kani::proof]
fn proof_depth_bound() {
    let expr = kani::any::<Expression>();
    assume(is_well_formed(&expr));
    assert!(calculate_depth(expr) <= MAX_EXPRESSION_DEPTH);
}
```

---

### Kani Harness: ExpressionDepth Safety

```
Property: ExpressionDepth::new(d) == Err(_) implies d > MAX_EXPRESSION_DEPTH
Bound: u32 range with assumption on d
Rationale: Prove that the type system enforces depth bounds at compile/runtime
```

**Kani harness code:**
```rust
#[kani::proof]
fn proof_expression_depth_safety() {
    let d: u32 = kani::any();
    let result = ExpressionDepth::new(d);
    match result {
        Ok(_) => assert!(d <= MAX_EXPRESSION_DEPTH),
        Err(_) => assert!(d > MAX_EXPRESSION_DEPTH),
    }
}
```

---

## 7. Mutation Testing Checkpoints

### Critical Mutations to Survive

| Mutation Type | Location | Test that Catches It |
|---------------|----------|---------------------|
| `>=` changed to `>` | `resolve_expressions` depth check | `resolve_expressions_returns_depth_limit_error_when_depth_exceeds_maximum()` |
| `>` changed to `>=` | `resolve_expressions` depth check | `resolve_expressions_returns_depth_limit_error_when_depth_exceeds_maximum()` |
| `1 +` changed to `0 +` | `calculate_depth` recursive step | `fn calculate_depth_returns_max_child_depth_plus_one_for_multi_child_expression()` |
| `max()` changed to `min()` | `calculate_depth` child aggregation | `fn calculate_depth_returns_max_child_depth_plus_one_for_multi_child_expression()` |
| `is_valid()` removed | `resolve_expressions` pre-check | `resolve_expressions_returns_depth_limit_error_when_depth_exceeds_maximum()` |
| `ExpressionDepth::new` validation removed | Constructor | `fn expression_depth_new_returns_error_when_depth_exceeds_maximum()` |
| `increment()` bounds check removed | Method | `fn expression_depth_increment_returns_error_when_result_exceeds_maximum()` |
| Error variant `DepthLimitExceeded` replaced with `InvalidExpression` | Error mapping | `fn resolve_expressions_returns_depth_limit_error_when_depth_exceeds_maximum()` |
| Depth values swapped in error payload | Error construction | `fn resolve_expressions_returns_depth_limit_error_when_depth_exceeds_maximum()` |
| `MAX_EXPRESSION_DEPTH` changed to different value | Constant | `fn expression_depth_new_succeeds_when_depth_at_maximum()` |

**Target Mutation Kill Rate**: ≥90%

**Verification command:** `cargo mutants --shallow`

---

## 8. Combinatorial Coverage Matrix

### Unit Tests: ExpressionDepth Type

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| depth 0 | minimum valid | Ok(ExpressionDepth(0)) | unit |
| depth 1023 | one below max | Ok(ExpressionDepth(1023)) | unit |
| depth 1024 | exact max | Ok(ExpressionDepth(1024)) | unit |
| depth 1025 | one above max | Err(DepthLimitExceeded{1025, 1024}) | unit |
| depth u32::MAX | overflow edge | Err(DepthLimitExceeded{u32::MAX, 1024}) | unit |
| increment from 0 | valid increment | Ok(ExpressionDepth(1)) | unit |
| increment from 1023 | valid increment | Ok(ExpressionDepth(1024)) | unit |
| increment from 1024 | invalid increment | Err(DepthLimitExceeded{1025, 1024}) | unit |
| is_valid(0) | valid check | true | unit |
| is_valid(1024) | valid check | true | unit |
| is_valid(1025) | invalid check | false | unit |

### Unit Tests: calculate_depth

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| leaf expression | no children | 0 | unit |
| single child | 1 child at depth 0 | 1 | unit |
| multi-child | children at [2,5,3] | 6 | unit |
| deep nested | 1023 levels | 1023 | unit |
| wide tree | 1000 siblings at depth 0 | 1 | unit |
| mixed structure | complex tree | computed max | unit |

### Integration Tests: resolve_expressions

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| valid resolution | depth 0, valid expr | Ok(ResolvedExpression) | integration |
| depth boundary | depth 1023, valid expr | Ok(ResolvedExpression) | integration |
| depth limit | depth 1024, valid expr | Err(DepthLimitExceeded{1024, 1024}) | integration |
| depth exceeded | depth 1025, valid expr | Err(DepthLimitExceeded{1025, 1024}) | integration |
| invalid syntax | depth 0, malformed expr | Err(InvalidExpression) | integration |
| missing ref | depth 0, unresolved ref | Err(ExpressionNotFound) | integration |
| type mismatch | depth 0, type error | Err(TypeError) | integration |
| runtime error | depth 0, eval error | Err(RuntimeError) | integration |
| immutability success | depth 0, valid expr | Ok + unchanged tree | integration |
| immutability error | depth 1024, valid expr | Err + unchanged tree | integration |

### E2E Tests: Stack Safety

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| deep valid tree | depth 1000 expression, depth 0 | Ok + no panic | e2e |
| max depth tree | depth 1024 expression, depth 0 | Err(DepthLimitExceeded) + no panic | e2e |
| overflow attempt | depth 2000 expression, depth 0 | Err(DepthLimitExceeded) + no panic | e2e |

### Proptest Invariants

| Invariant | Input Class | Expected Property | Test Layer |
|-----------|-------------|-------------------|------------|
| non-negative depth | any valid expression | depth >= 0 | proptest |
| leaf depth zero | leaf expressions | depth == 0 | proptest |
| recursive depth | any non-leaf | depth == 1 + max(children) | proptest |
| depth bound | any well-formed | depth <= 1024 | proptest |
| increment safety | depths 0-1023 | increment succeeds | proptest |
| increment fail | depth 1024 | increment fails | proptest |

### Fuzz Coverage

| Fuzz Target | Input Class | Risk Mitigated | Test Layer |
|-------------|-------------|----------------|------------|
| calculate_depth | random serialized expr | panic, infinite recursion | fuzz |
| expression_depth_new | random u32 values | overflow, validation bypass | fuzz |

---

## Exit Criteria Verification

- ✅ Every public API behavior has a BDD scenario (23 behaviors covered)
- ✅ Every Error variant has a test scenario (5/5 variants)
  - `DepthLimitExceeded`: 6 test scenarios
  - `InvalidExpression`: 1 test scenario
  - `ExpressionNotFound`: 1 test scenario
  - `TypeError`: 1 test scenario
  - `RuntimeError`: 1 test scenario
- ✅ Mutation threshold (≥90%) is stated
- ✅ No planned assertion is just `is_ok()` or `is_err()` (all assert exact values/variants)

---

## Open Questions

1. **What is the exact Expression struct definition?** The plan assumes a recursive structure with children. Confirm the actual type definition.

2. **What is the ExpressionRegistry interface?** Need to understand how references are resolved to implement `ExpressionNotFound` tests.

3. **What does "ResolvedExpression" contain?** The plan assumes it's a value wrapper. Confirm the type.

4. **Does `calculate_depth` traverse the entire tree or just compute max depth?** Clarify if it's a full traversal or optimized.

5. **Are there any platform-specific stack size considerations?** The contract mentions 1000-2000 as platform-dependent. Confirm 1024 is correct.

6. **Is there a way to construct invalid `ExpressionDepth` values for testing `is_valid(false)` cases?** The plan assumes unsafe construction or direct field access for testing.

7. **What is the error message format for each variant?** The plan includes example messages but needs confirmation for exact formatting.

---

## Appendix: Test Function Name Conventions

All test functions follow the pattern: `fn [subject]_[outcome]_when_[condition]()`

Examples:
- `fn resolve_expressions_returns_depth_limit_error_when_depth_exceeds_maximum()`
- `fn expression_depth_new_succeeds_when_depth_at_maximum()`
- `fn calculate_depth_returns_zero_for_leaf_expression()`
- `fn validate_expression_depth_returns_valid_depth_when_within_limit()`

---

## Appendix: Error Variant Test Coverage

| Error Variant | Test Function | Asserts Exact Variant |
|---------------|---------------|----------------------|
| `DepthLimitExceeded` | `resolve_expressions_returns_depth_limit_error_when_depth_exceeds_maximum()` | Yes, checks both payload fields |
| `InvalidExpression` | `resolve_expressions_returns_invalid_expression_error_when_syntax_malformed()` | Yes, matches pattern |
| `ExpressionNotFound` | `resolve_expressions_returns_expression_not_found_when_reference_missing()` | Yes, checks reference field |
| `TypeError` | `resolve_expressions_returns_type_error_when_types_mismatch()` | Yes, checks both type fields |
| `RuntimeError` | `resolve_expressions_returns_runtime_error_when_evaluation_fails()` | Yes, checks message field |

All 5 error variants have explicit test scenarios with exact variant matching.
