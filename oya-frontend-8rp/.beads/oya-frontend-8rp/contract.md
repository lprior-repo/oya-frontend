# Contract Specification: Expression Depth Limiting

## Context

- **Feature**: Add expression depth limit to `resolve_expressions()` to prevent stack overflow
- **Bead ID**: oya-frontend-8rp
- **Domain terms**:
  - Expression: A nested computation unit that can contain sub-expressions
  - Depth: The nesting level of expressions (root = 0, first child = 1, etc.)
  - Stack overflow: Runtime panic caused by exceeding system stack limits
- **Assumptions**:
  - Expressions are represented as recursive data structures
  - The current implementation traverses expressions recursively
  - Maximum safe depth is approximately 1000-2000 (platform-dependent)
- **Open questions**:
  - What is the exact maximum depth threshold? (Default: 1024)
  - Should the depth limit be configurable or fixed?
  - Do we need to track depth across multiple resolve calls or per-call only?

## Maximum Depth Constant

```rust
pub const MAX_EXPRESSION_DEPTH: u32 = 1024;
```

## Type Definitions

```rust
/// Represents the depth of a nested expression
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExpressionDepth(u32);

impl ExpressionDepth {
    pub fn new(depth: u32) -> Result<Self, Error>;
    pub fn current(&self) -> u32;
    pub fn increment(&self) -> Result<Self, Error>;
    pub fn is_valid(&self) -> bool;
}

/// Error taxonomy for expression resolution
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Exceeded maximum allowed expression depth
    DepthLimitExceeded { current_depth: u32, max_depth: u32 },
    
    /// Input expression contains invalid syntax or structure
    InvalidExpression,
    
    /// Expression reference not found
    ExpressionNotFound,
    
    /// Type mismatch in expression evaluation
    TypeError { expected: &'static str, actual: &'static str },
    
    /// Runtime error during expression evaluation
    RuntimeError { message: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl std::error::Error for Error {}
```

## Preconditions

- [ ] `resolve_expressions()` must be called with a valid expression tree
- [ ] Initial depth must be 0 (root level)
- [ ] Expression tree must not be corrupted or contain circular references
- [ ] Maximum depth constant must be defined and accessible
- [ ] All sub-expressions must be well-formed before processing

## Postconditions

### On Success
- [ ] Expression is fully resolved without stack overflow
- [ ] Result contains the evaluated expression value
- [ ] No intermediate expressions remain in unresolved state
- [ ] Memory is properly managed (no leaks from deep recursion)

### On Depth Limit Violation
- [ ] Returns `Error::DepthLimitExceeded` with current and max depth
- [ ] No partial resolution state is left behind
- [ ] All owned resources are properly released
- [ ] Error message includes actionable information (depth values)

### On Other Errors
- [ ] Returns appropriate `Error` variant from taxonomy
- [ ] Expression tree remains in original state (no mutation on error)
- [ ] No side effects from failed resolution attempt

## Invariants

- [ ] **Depth Monotonicity**: Expression depth can only increase during traversal
- [ ] **Depth Bound**: Current depth never exceeds `MAX_EXPRESSION_DEPTH`
- [ ] **Type Safety**: All expressions maintain type consistency throughout resolution
- [ ] **No Dangling References**: All expression references are valid during resolution
- [ ] **Resource Safety**: No memory leaks or resource exhaustion from deep nesting
- [ ] **Error Completeness**: Every failure mode has a corresponding error variant

## Error Taxonomy

### Error::DepthLimitExceeded
- **When**: Current depth exceeds `MAX_EXPRESSION_DEPTH` during traversal
- **Payload**: `current_depth: u32`, `max_depth: u32`
- **Recovery**: User must refactor expression to reduce nesting depth
- **Example**: `"Expression depth 1025 exceeds maximum of 1024"`

### Error::InvalidExpression
- **When**: Expression syntax or structure is malformed
- **Payload**: None (detailed context via logging/debug info)
- **Recovery**: User must correct expression syntax
- **Example**: `"Unexpected token at position 42"`

### Error::ExpressionNotFound
- **When**: Referenced expression cannot be found in registry
- **Payload**: `reference: String`
- **Recovery**: User must define or reference correct expression
- **Example**: `"Expression 'user.input' not found"`

### Error::TypeError
- **When**: Type mismatch during expression evaluation
- **Payload**: `expected: &'static str`, `actual: &'static str`
- **Recovery**: User must ensure type compatibility
- **Example**: `"Expected i32, found String"`

### Error::RuntimeError
- **When**: Unexpected runtime error during resolution
- **Payload**: `message: String`
- **Recovery**: Investigate root cause, may be system issue
- **Example**: `"Division by zero in expression evaluation"`

## Contract Signatures

```rust
/// Maximum allowed expression nesting depth
pub const MAX_EXPRESSION_DEPTH: u32 = 1024;

/// Resolve an expression tree with depth limiting
/// 
/// # Arguments
/// * `expression` - The root expression to resolve
/// * `registry` - Context containing referenced expressions
/// * `current_depth` - Current nesting depth (starts at 0)
/// 
/// # Returns
/// * `Result<ResolvedExpression, Error>` - Either resolved value or error
/// 
/// # Preconditions
/// - `current_depth < MAX_EXPRESSION_DEPTH`
/// - `expression` is well-formed
/// - `registry` contains all referenced expressions
/// 
/// # Postconditions
/// - If Ok: Expression fully resolved, no side effects
/// - If Err: Original state preserved, error describes failure
pub fn resolve_expressions<T>(
    expression: &Expression,
    registry: &ExpressionRegistry,
    current_depth: ExpressionDepth,
) -> Result<ResolvedExpression, Error> {
    // Contract enforcement: check depth before processing
    if !current_depth.is_valid() {
        return Err(Error::DepthLimitExceeded {
            current_depth: current_depth.current(),
            max_depth: MAX_EXPRESSION_DEPTH,
        });
    }
    
    // ... resolution logic ...
}

/// Validate expression depth before resolution
pub fn validate_expression_depth(expression: &Expression) -> Result<ExpressionDepth, Error> {
    let depth = calculate_depth(expression);
    ExpressionDepth::new(depth)
}

/// Calculate maximum nesting depth of expression tree
fn calculate_depth(expression: &Expression) -> u32 {
    // ... depth calculation logic ...
}
```

## Non-goals

- [ ] Implementing expression language semantics (assumes existing evaluator)
- [ ] Configurable depth limits per-resolve (uses global constant)
- [ ] Caching or memoization of expression results
- [ ] Async/await support for expression resolution
- [ ] Performance optimization for deep expressions (prevention is the solution)
- [ ] Supporting iterative deepening or incremental resolution

## Implementation Notes

1. **Depth Tracking**: Pass depth as explicit parameter through recursive calls
2. **Early Exit**: Check depth at entry point, before any processing
3. **Error Context**: Include depth values in error messages for debugging
4. **No Mutation on Error**: Expression tree must remain immutable during failed resolution
5. **Type Safety**: Use `ExpressionDepth` wrapper to prevent invalid depths at type level

## Verification Criteria

- [ ] Code compiles with `rustc` without warnings
- [ ] All error variants are exhaustive (no `unreachable!()`)
- [ ] Depth check occurs before any recursive calls
- [ ] Error messages are human-readable and actionable
- [ ] Contract signatures use `Result<T, Error>` for all fallible operations
- [ ] `MAX_EXPRESSION_DEPTH` is defined as `const` (not `let` or `lazy_static`)
