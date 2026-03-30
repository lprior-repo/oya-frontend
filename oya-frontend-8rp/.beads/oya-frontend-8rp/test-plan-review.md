## VERDICT: REJECTED

### Axis 1 — Contract Parity

[PASS] All public functions have BDD scenarios:
- `MAX_EXPRESSION_DEPTH` → Invariant 10
- `ExpressionDepth::new` → Behaviors 1-6
- `ExpressionDepth::current` → Behaviors 7-9
- `ExpressionDepth::increment` → Behaviors 10-13
- `ExpressionDepth::is_valid` → Behaviors 14-18
- `resolve_expressions` → Behaviors 27-30, 36-42
- `validate_expression_depth` → Behaviors in Section 5
- `calculate_depth` (private) → Behaviors 19-26

[PASS] All Error variants have exact variant assertions:
- `DepthLimitExceeded` → Behaviors 5-6, 12, 29-30, Section 5 tests
- `InvalidExpression` → Behavior 39 (resolve_expressions_fails_with_invalid_expression_syntax)
- `ExpressionNotFound` → Behavior 36 (resolve_expressions_returns_expression_not_found_with_missing_reference)
- `TypeError` → Behavior 37 (resolve_expressions_returns_type_error_with_mismatched_types)
- `RuntimeError` → Behavior 38 (resolve_expressions_returns_runtime_error_with_division_by_zero)

**CRITICAL FAILURE: Error::ExpressionNotFound payload mismatch**

Contract (lines 116-121):
```rust
Error::ExpressionNotFound { reference: String }
```

Test-plan.md Behavior 36 (lines 619-627):
```
Then: Err(Error::ExpressionNotFound) is returned
```

**The test does NOT assert the `reference: String` payload.** The contract explicitly defines a payload field for this error variant. The test must verify the payload matches, not just the variant.

**LETHAL FINDING #1:** `test-plan.md:625` — `resolve_expressions_returns_expression_not_found_with_missing_reference` asserts `Err(Error::ExpressionNotFound)` without verifying `reference: String` payload.

---

### Axis 2 — Assertion Sharpness

[PASS] No `is_ok()` / `is_err()` bare assertions found. All assertions use concrete values.

[PASS] No `> 0` or `Some(_)` vague assertions. All use exact values (0, 1024, 1025, u32::MAX).

**CRITICAL FAILURE: E2E teardown uses unverifiable predicate**

Test-plan.md Section 7, "e2e_test_cleanup_verifies_isolated_state" (lines 759-770):

```
Then: Test completes without timeout errors
And: Application returns to default state:
        - Default value displayed in input field
        - Output section cleared
        - No error toasts visible
```

The assertion "Test completes without timeout errors" is a **predicate** that says nothing about the actual outcome. A test that hangs forever and gets killed by the test runner's timeout would satisfy this assertion.

**MAJOR FINDING #1:** Line 765 — "Test completes without timeout errors" is a hollow assertion. Must be replaced with concrete observable behavior like "Test exits with code 0" or "Test execution time < 5 seconds."

---

### Axis 3 — Trophy Allocation

**Count public functions from contract.md:**
- `MAX_EXPRESSION_DEPTH` (const)
- `ExpressionDepth::new`
- `ExpressionDepth::current`
- `ExpressionDepth::increment`
- `ExpressionDepth::is_valid`
- `resolve_expressions`
- `validate_expression_depth`
- `calculate_depth` (private)

**Total: 8 functions (7 public + 1 private)**

Test-plan.md Summary (lines 14-15):
> Trophy allocation: **35 unit / 7 integration / 4 e2e = 47 tests**

Ratio: 47 / 8 = 5.875×

**[PASS] Ratio ≥ 5× achieved.**

**[PASS] `calculate_depth` has proptest invariants:**
- Invariants 1-5 cover `calculate_depth` (lines 818-838)
- Invariant 2 specifically asserts the recursive formula: `calculate_depth(nested_expression) == 1 + max(calculate_depth(child) for child in children)`

**[PASS] Fuzz targets cover parsers:**
- `parse_expression_depth` (lines 889-905)
- `deserialize_expression_tree` (lines 909-925)

---

### Axis 4 — Boundary Completeness

**For `ExpressionDepth::new`:**
- Minimum valid (0): Behavior 1 ✅
- Maximum valid (1024): Behavior 4 ✅
- One-below-minimum: N/A (u32::MIN = 0 is valid)
- One-above-maximum (1025): Behavior 5 ✅
- Empty/zero: Behavior 1 ✅
- Overflow (u32::MAX): Behavior 6 ✅

**For `ExpressionDepth::increment`:**
- Minimum valid (0): Behavior 10 ✅
- Maximum valid (1023 → 1024): Behavior 11 ✅
- One-above-maximum (1024): Behavior 12 ✅

**For `ExpressionDepth::is_valid`:**
- Accepts 0: Behavior 14 ✅
- Accepts 1023: Behavior 15 ✅
- Accepts 1024: Behavior 16 ✅
- Rejects 1025: Behavior 17 ✅
- Rejects u32::MAX: Behavior 18 ✅

**For `calculate_depth`:**
- Empty (0): Behavior 19 ✅
- Leaf (0): Behavior 20 ✅
- Single child (1): Behavior 21 ✅
- Nested (2): Behavior 22 ✅
- Formula assertion: Behavior 23 ✅
- Max tree (1024): Behavior 24 ✅
- Excessive (1025): Behavior 25 ✅
- Wide tree (1000 siblings): Behavior 26 ✅

**For `resolve_expressions`:**
- Depth 0: Behavior 27 ✅
- Depth 1023: Behavior 28 ✅
- Depth 1024: Behavior 29 ✅
- Depth 1025: Behavior 30 ✅

**For `validate_expression_depth`:**
- Depth 500: Section 5 Behavior 1 ✅
- Depth 1500: Section 5 Behavior 2 ✅
- Depth 0: Section 5 Behavior 3 ✅
- Depth 1024: Section 5 Behavior 4 ✅
- Depth 1025: Section 5 Behavior 5 ✅

**[PASS] All boundaries explicitly covered.**

---

### Axis 5 — Mutation Survivability

**Test the "swap current_depth and max_depth" mutation:**

Mutation at Behavior 29 (lines 522-531):
```
Then: Err(Error::DepthLimitExceeded{current_depth: 1024, max_depth: 1024}) is returned
```

If swapped, both values are 1024, so the mutation is **NOT CAUGHT**.

Test-plan.md explicitly acknowledges this (lines 549-550):
> **MAJOR FIX #8 (continued):** This is the test that catches error message format mutations because current_depth != max_depth. If swapped, the message would be "Expression depth 1024 exceeds maximum of 1025" which fails the assertion.

The plan **correctly identifies Behavior 30** as the mutation-catching test. **PASS.**

**Test the ">= to >" mutation in resolve_expressions:**

Behavior 29 tests depth 1024. If the mutation changes `>=` to `>`, then depth 1024 would pass instead of fail. The test would catch it because it expects `Err(DepthLimitExceeded{1024, 1024})`.

**PASS.**

**Test the "Ok to Err" mutation in ExpressionDepth::new:**

Behavior 4 tests that `ExpressionDepth::new(1024)` returns `Ok(ExpressionDepth(1024))`. If mutated to return `Err`, the test fails.

**PASS.**

**Test the "Err to Ok" mutation in ExpressionDepth::new:**

Behavior 5 tests that `ExpressionDepth::new(1025)` returns `Err(...)`. If mutated to return `Ok`, the test fails.

**PASS.**

**Test the "0 to 1" mutation in calculate_depth base case:**

Behavior 19 tests that `calculate_depth` with empty children returns 0. If mutated to return 1, the test fails.

**PASS.**

**Test MAX_EXPRESSION_DEPTH constant mutation:**

Invariant 10 (lines 864-872):
```
Invariant 10: MAX_EXPRESSION_DEPTH constant value is verified
Strategy: Assert MAX_EXPRESSION_DEPTH == 1024 at compile time
Anti-invariant: If constant is changed to 2048, this invariant fails
Implementation: const ASSERTION: () = assert!(MAX_EXPRESSION_DEPTH == 1024);
```

**PASS.**

**[PASS] All critical mutations have catching tests.**

---

### Axis 6 — Holzmann Plan Audit

**Rule 2 (ceiling on iteration):**
Test-plan.md line 20:
> Recursion ceiling: 1025 levels explicit in calculate_depth

**PASS.**

**Rule 3 (no test depends on execution order):**
Test-plan.md Section 8 (lines 776-811):
> Each E2E test runs with a fresh app instance or state reset between tests

**PASS.**

**Rule 5 (explicit preconditions):**
Most scenarios have "Given:" clauses. However, some are vague:

Behavior 29 (lines 522-531):
```
Given: A valid expression tree
And: A registry with all referenced expressions defined
```

What makes an expression "valid"? What does "defined" mean? These are **implicit preconditions**.

**MAJOR FINDING #2:** Lines 523-524 — "A valid expression tree" and "A registry with all referenced expressions defined" are vague preconditions. Must specify exact structure (e.g., "An Expression with value: Some(\"42\") and children: vec![]").

**Rule 8 (side effects in setup named explicitly):**
Test-plan.md does not explicitly document what side effects occur during setup (e.g., "Registry is initialized with empty map before each test").

**MINOR FINDING #1:** Missing explicit documentation of setup side effects.

---

## LETHAL FINDINGS

1. **Line 625 (Behavior 36)** — `Err(Error::ExpressionNotFound)` does not assert the `reference: String` payload. Contract line 118 defines `reference: String` as required payload.

## MAJOR FINDINGS (2)

1. **Line 765** — "Test completes without timeout errors" is a hollow predicate assertion.
2. **Lines 523-524** — Vague preconditions "A valid expression tree" and "A registry with all referenced expressions defined" without exact structure specification.

## MINOR FINDINGS (1/5 threshold)

1. **Missing** — Explicit documentation of setup side effects (Holzmann Rule 8).

---

## MANDATE

**Before resubmission, fix ALL findings:**

### LETHAL: Fix Behavior 36 payload assertion
Behavior 36 (resolve_expressions_returns_expression_not_found_with_missing_reference) must assert the `reference: String` payload:
```
Then: Err(Error::ExpressionNotFound { reference: "missing_expr" }) is returned
And: The error.reference field equals "missing_expr"
```

### MAJOR: Fix E2E teardown assertion
Replace "Test completes without timeout errors" with concrete assertion:
```
Then: Test exits with code 0
And: Test execution time < 5 seconds
```

### MAJOR: Fix vague preconditions
Behavior 29 and 30 preconditions must specify exact structure:
```
Given: An Expression with value: Some("42") and children: vec![]
And: A registry initialized with { "test_expr" → expression }
```

### MINOR: Document setup side effects
Add Section: "Test Setup Side Effects" documenting:
- Registry initialization (empty map vs. seeded with test expressions)
- Signal state reset (if frontend tests)
- Any global state modifications before each test

---

## RE-TEST INSTRUCTION

After fixes, re-run the full six-axis audit from scratch. Do not assume previous fixes are still present.

**STATUS: REJECTED**

---

*Review completed on: Mon Mar 30 2026*
*Review iteration: 7*
*Previous status: REJECTED (3 LETHAL, 6 MAJOR, 3 MINOR)*
*Current status: REJECTED (1 LETHAL, 2 MAJOR, 1 MINOR)*
