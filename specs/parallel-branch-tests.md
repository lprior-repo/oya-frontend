# Martin Fowler Test Plan: Parallel Branch Grouping

## Test Organization
- **Happy Path**: Correctly identifies parallel branches and calculates offsets
- **Error Path**: Handles edge cases gracefully
- **Edge Cases**: Empty inputs, boundary values, single targets
- **Contract Verification**: Validates preconditions and postconditions

---

## Happy Path Tests

### Test: `given_source_with_two_targets_when_find_parallel_then_returns_one_group`
```
Given: source node at (100, 100)
And: target node A at (300, 100)
And: target node B at (300, 200)
And: connection from source to A
And: connection from source to B

When: find_parallel_branches is called

Then: returns Vec with exactly 1 ParallelGroup
And: group.source_node is Some(source node)
And: group.target_nodes contains A and B (sorted by ID)
And: group.bounds.x = min(A.x, B.x) - 8.0 = 292.0
And: group.bounds.y = min(A.y, B.y) - 8.0 = 92.0
And: group.bounds.width = (max(A.x, B.x) - min(A.x, B.x)) + 16.0 = 16.0
And: group.bounds.height = (max(A.y, B.y) - min(A.y, B.y)) + 16.0 = 116.0
```

### Test: `given_source_with_three_targets_when_find_parallel_then_returns_one_group_with_three_targets`
```
Given: source node at (100, 100)
And: target nodes A, B, C at y-positions 100, 200, 300
And: connections from source to each target

When: find_parallel_branches is called

Then: returns Vec with exactly 1 ParallelGroup
And: group.target_nodes contains A, B, C in lexicographic order
And: bounds tightly enclose all three targets with 8px padding
```

### Test: `given_source_with_many_targets_when_find_parallel_then_returns_one_group`
```
Given: source node at (100, 100)
And: 5 target nodes at various positions
And: connections from source to each target

When: find_parallel_branches is called

Then: returns Vec with exactly 1 ParallelGroup containing all 5 targets
And: bounds encompass all targets with 8px padding
```

### Test: `given_single_connection_when_find_parallel_then_returns_empty_vec`
```
Given: source node at (100, 100)
And: single target node at (300, 100)
And: one connection from source to target

When: find_parallel_branches is called

Then: returns empty Vec (no parallel branches)
```

### Test: `given_source_with_two_targets_when_calculate_offset_then_returns_symmetric_values`
```
Given: two target nodes A and B at same x-position
And: NODE_HEIGHT = 68.0
And: spacing = NODE_HEIGHT / 2.5 = 27.2

When: calculate_parallel_offset called for A
Then: returns -13.6 (negative offset)

When: calculate_parallel_offset called for B
Then: returns +13.6 (positive offset)
```

### Test: `given_three_targets_when_calculate_offset_then_returns_centered_values`
```
Given: three target nodes A, B, C at same x-position
And: NODE_HEIGHT = 68.0
And: spacing = 27.2

When: calculate_parallel_offset called for A (first)
Then: returns -27.2

When: calculate_parallel_offset called for B (middle)
Then: returns 0.0

When: calculate_parallel_offset called for C (last)
Then: returns +27.2
```

### Test: `given_four_targets_when_calculate_offset_then_returns_symmetric_values`
```
Given: four target nodes A, B, C, D at same x-position
And: NODE_HEIGHT = 68.0
And: spacing = 27.2

When: calculate_parallel_offset called for A
Then: returns -40.8

When: calculate_parallel_offset called for B
Then: returns -13.6

When: calculate_parallel_offset called for C
Then: returns +13.6

When: calculate_parallel_offset called for D
Then: returns +40.8
```

### Test: `given_parallel_groups_when_resolve_anchors_then_offsets_applied_to_targets`
```
Given: source node at (100, 100)
And: two target nodes A (y=100) and B (y=200)
And: parallel group with bounds enclosing both targets
And: offset for A = -13.6, offset for B = +13.6

When: resolve_edge_anchors_with_parallel is called

Then: edge to A has anchor.to.y = 100 + (-13.6) = 86.4
And: edge to B has anchor.to.y = 200 + 13.6 = 213.6
And: other edges (non-parallel) have unmodified anchors
```

---

## Error Path Tests

### Test: `given_duplicate_connections_when_find_parallel_then_treats_as_single_connection`
```
Given: source node
And: same target node connected twice
And: duplicate connections in the slice

When: find_parallel_branches is called

Then: duplicate is counted as one connection
And: returns empty Vec (only 1 unique target)
```

### Test: `given_non_parallel_connections_when_find_parallel_then_ignores_them`
```
Given: source A with 2 targets
And: source B with 1 target
And: source C with 3 targets

When: find_parallel_branches is called

Then: returns 2 ParallelGroups (A and C only)
And: source B is excluded (only 1 connection)
```

---

## Edge Case Tests

### Test: `given_empty_connections_when_find_parallel_then_returns_empty_vec`
```
Given: nodes slice with any nodes
And: empty connections slice

When: find_parallel_branches is called

Then: returns empty Vec
```

### Test: `given_empty_nodes_when_find_parallel_then_returns_empty_vec`
```
Given: empty nodes slice
And: any connections

When: find_parallel_branches is called

Then: returns empty Vec (all filter_map lookups fail)
```

### Test: `given_single_node_with_no_connections_when_find_parallel_then_returns_empty_vec`
```
Given: one node with no connections

When: find_parallel_branches is called

Then: returns empty Vec
```

### Test: `given_many_non_parallel_sources_when_find_parallel_then_returns_empty_vec`
```
Given: 10 source nodes, each with exactly 1 connection

When: find_parallel_branches is called

Then: returns empty Vec (no parallel branches)
```

### Test: `given_target_id_not_in_targets_when_calculate_offset_then_returns_zero`
```
Given: target_id that doesn't match any node in targets slice
And: non-empty targets slice

When: calculate_parallel_offset is called

Then: returns 0.0 (from unwrap_or fallback)
```

### Test: `given_single_target_when_calculate_offset_then_returns_zero`
```
Given: single target node
And: NODE_HEIGHT = 68.0

When: calculate_parallel_offset is called

Then: returns 0.0 (single node, no offset needed)
```

### Test: `given_nodes_at_same_position_when_find_parallel_then_bounds_have_zero_dimensions`
```
Given: source node at (100, 100)
And: two target nodes at same position (300, 100)
And: connections from source to both targets

When: find_parallel_branches is called

Then: bounds.width = 16.0 (padding only)
And: bounds.height = 16.0 (padding only)
```

### Test: `given_targets_at_varying_y_positions_when_calculate_offset_then_respects_order`
```
Given: target nodes with IDs that sort as A < B < C
And: A is at y=300, B is at y=100, C is at y=200
(Note: positions don't affect offset calculation)

When: calculate_parallel_offset for B (middle in sorted order)

Then: returns 0.0 (middle position in sorted list, not by y-coordinate)
```

---

## Contract Verification Tests

### Test: `invariant_parallel_groups_only_have_2_plus_targets`
```
Given: any nodes and connections

When: find_parallel_branches returns groups

Then: for each group, group.target_nodes.len() >= 2
```

### Test: `invariant_target_nodes_are_sorted_by_id`
```
Given: any nodes and connections with parallel branches

When: find_parallel_branches returns groups

Then: for each group, target_nodes are in lexicographic ID order
```

### Test: `invariant_bounds_have_positive_dimensions`
```
Given: parallel groups with valid nodes

When: bounds are calculated

Then: bounds.width > 0.0
And: bounds.height > 0.0
```

### Test: `postcondition_bounds_tightly_enclose_targets_with_padding`
```
Given: parallel group with target nodes

When: bounds are calculated

Then: bounds.x <= min(target.x) - 8.0
And: bounds.y <= min(target.y) - 8.0
And: bounds.x + bounds.width >= max(target.x + NODE_WIDTH) + 8.0
And: bounds.y + bounds.height >= max(target.y + NODE_HEIGHT) + 8.0
```

### Test: `postcondition_offset_is_symmetric_with_respect_to_target_order`
```
Given: two target nodes A and B
And: offset_A = calculate_parallel_offset(&A.id, &[A, B], NODE_HEIGHT)
And: offset_B = calculate_parallel_offset(&B.id, &[A, B], NODE_HEIGHT)

When: reverse target order
And: offset_B_rev = calculate_parallel_offset(&B.id, &[B, A], NODE_HEIGHT)
And: offset_A_rev = calculate_parallel_offset(&A.id, &[B, A], NODE_HEIGHT)

Then: offset_A == -offset_A_rev
And: offset_B == -offset_B_rev
```

---

## Given-When-Then Scenarios

### Scenario 1: Simple Parallel Branch
```
Given: workflow with 1 source node and 2 target nodes
And: source at (100, 100), targets at (300, 100) and (300, 200)
And: connections from source to both targets

When: rendering edges with parallel grouping

Then: one ParallelGroup is created
And: container rect appears around both targets
And: edges are vertically offset (-13.6 and +13.6)
And: badge shows "2 branches"
```

### Scenario 2: Multiple Parallel Groups
```
Given: workflow with 2 sources, each having 2 targets
And: source A at (100, 100) with targets A1, A2
And: source B at (100, 300) with targets B1, B2

When: find_parallel_branches is called

Then: returns 2 ParallelGroups (one per source)
And: each group has its own container and offset calculations
And: no interference between groups
```

### Scenario 3: Large Parallel Group
```
Given: source with 5 target nodes in a vertical line
And: y-positions: 100, 200, 300, 400, 500

When: calculate_parallel_offset for each target

Then: offsets are: -54.4, -27.2, 0.0, +27.2, +54.4
And: evenly distributed around center
And: total span = 4 * spacing = 108.8
```

### Scenario 4: Mixed Parallel and Non-Parallel Edges
```
Given: source with 2 parallel targets AND 1 non-parallel target
And: connections to A, B (parallel) and C (single)

When: resolve_edge_anchors_with_parallel is called

Then: edges to A and B have parallel offsets
And: edge to C has no offset (not in parallel group)
And: all anchors are present in result HashMap
```

---

## Test Data Builders

```rust
// Builder pattern for test nodes
fn build_node(id: NodeId, x: f32, y: f32) -> Node {
    Node {
        id,
        name: format!("Node {}", id),
        description: String::new(),
        node_type: "task".to_string(),
        category: NodeCategory::Durable,
        icon: "fa fa-cog".to_string(),
        x,
        y,
        config: serde_json::Value::Object(serde_json::Map::new()),
        last_output: None,
        selected: false,
        executing: false,
        skipped: false,
        error: None,
        execution_state: ExecutionState::Pending,
    }
}

fn build_connection(id: Uuid, source: NodeId, target: NodeId) -> Connection {
    Connection {
        id,
        source,
        target,
        source_port: PortName::from("out"),
        target_port: PortName::from("in"),
    }
}
```

---

## Running Tests

```bash
cargo test --lib --parallel-branch
```

Or run all tests:
```bash
cargo test --lib
```
