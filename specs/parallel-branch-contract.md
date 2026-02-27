# Contract Specification: Parallel Branch Grouping

## Context
- Feature: Parallel branch grouping for flow diagram edge rendering
- Domain: Workflow visualization with multiple outgoing connections from a single source node
- Assumptions:
  - Nodes have `x`, `y` coordinates and fixed dimensions (`NODE_WIDTH = 220.0`, `NODE_HEIGHT = 68.0`)
  - Connections connect source `NodeId` to target `NodeId`
  - Parallel branches = multiple connections from the same source to different targets
  - Offset calculation uses lexicographic ordering of target node IDs for consistent positioning
- Open questions: None

## Preconditions
- [P1] `nodes` slice must contain all nodes referenced in `connections`
- [P2] `connections` slice may be empty (no parallel branches case)
- [P3] `target_nodes` in `calculate_parallel_offset` must not be empty
- [P4] `target_id` must exist in the `targets` slice for `calculate_parallel_offset`

## Postconditions
- [Q1] `find_parallel_branches` returns groups with exactly those sources having 2+ targets
- [Q2] Each `ParallelGroup` has `target_nodes` sorted by `NodeId` (lexicographic order)
- [Q3] Each `ParallelGroup` has `bounds` that tightly enclose all target nodes with 8px padding
- [Q4] `calculate_parallel_offset` returns 0.0 for single-target groups (not called per precondition)
- [Q5] `calculate_parallel_offset` returns evenly distributed offsets centered around 0
- [Q6] `resolve_edge_anchors_with_parallel` returns all edge anchors with parallel-offset targets

## Invariants
- [I1] Parallel groups only contain sources with 2+ connections
- [I2] Target nodes in each group are sorted by ID for deterministic offset calculation
- [I3] Bounds always have positive width and height
- [I4] Offset calculation is symmetric: reversing target order negates offsets

## Error Taxonomy
No runtime errors expected - all operations are pure functions with valid input assumptions.
Preconditions are enforced by design (type system or caller responsibility).

## Contract Signatures
```rust
fn find_parallel_branches(nodes: &[Node], connections: &[Connection]) -> Vec<ParallelGroup>
fn calculate_parallel_offset(target_id: &NodeId, targets: &[Node], node_height: f32) -> f32
fn resolve_edge_anchors_with_parallel(
    edges: &[Connection],
    nodes: &[Node],
    parallel_groups: &[ParallelGroup],
) -> HashMap<String, EdgeAnchor>
```

## Type Encoding
| Precondition | Enforcement Level | Type / Pattern |
|---|---|---|
| P1: All connection node IDs exist in nodes | Caller responsibility | Debug-only `debug_assert!()` |
| P2: Empty connections allowed | Compile-time (slice) | `&[T]` accepts empty slices |
| P3: Non-empty targets for offset | Compile-time (type) | Should use `NonEmpty<T>` or similar |
| P4: Target ID exists in targets | Runtime-checked | `unwrap_or(0)` fallback in implementation |

## Violation Examples

### P3 Violation: Empty targets for offset calculation
```rust
// Given: empty targets slice
let targets: Vec<Node> = vec![];
let target_id = NodeId::new();

// When: calculate_parallel_offset called
calculate_parallel_offset(&target_id, &targets, NODE_HEIGHT);

// Then: panic due to unwrap_or(0) on empty vector
// OR: should return 0.0 as sensible default
```

### P4 Violation: Target ID not in targets
```rust
// Given: target_id not in targets slice
let target_id = NodeId::new(); // Different from all in targets
let targets = vec![/* nodes with different IDs */];

// When: calculate_parallel_offset called
calculate_parallel_offset(&target_id, &targets, NODE_HEIGHT);

// Then: unwrap_or(0) returns 0.0 (not the expected offset)
```

### P1 Violation: Missing node in nodes slice
```rust
// Given: connection references NodeId not in nodes slice
let source_id = NodeId::new();
let target_id = NodeId::new();
let connections = vec![Connection { source: source_id, target: target_id, ... }];
let nodes = vec![]; // Empty - missing both source and target

// When: find_parallel_branches called
find_parallel_branches(&nodes, &connections);

// Then: source_node is None, target_nodes filtered to empty
// Result: group not created (correct behavior per filter_map)
```

## Ownership Contracts
- `find_parallel_branches`: Takes `&[Node]` (shared borrow), clones nodes for groups
- `calculate_parallel_offset`: Takes `&[Node]` (shared borrow), no mutation
- `resolve_edge_anchors_with_parallel`: Takes `&[Node]` and `&[ParallelGroup]`, no mutation

Clone policy: Nodes are cloned into `ParallelGroup` structs. This is intentional for ownership isolation.

## Non-goals
- Dynamic node positioning (uses fixed NODE_WIDTH/NODE_HEIGHT)
- Overlapping parallel groups (each group is independent)
- Performance optimization ( clarity over speed for this feature)
