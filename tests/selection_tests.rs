//! Selection state tests.
//!
//! Verifies:
//! 1. Node selected flag lifecycle (set/clear/query)
//! 2. Workflow deselect_all clears all flags
//! 3. Selection state survives workflow operations (drag, remove, undo)
//! 4. Adding nodes does not auto-select
//! 5. Removing a selected node clears its flag implicitly
//!
//! Note: The `Selection` enum and `SelectionState` (hooks/use_selection.rs) are
//! gated behind `#[cfg(target_arch = "wasm32")]` since they use Dioxus signals.
//! Those are tested via WASM unit tests. These tests cover the graph-level
//! selection primitives that the hook layer drives.
//!
//! Run: cargo test --test selection_tests

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use oya_frontend::graph::{NodeId, PortName, Workflow};

fn main_port() -> PortName {
    PortName::from("main")
}

fn workflow_with_three_nodes() -> (Workflow, NodeId, NodeId, NodeId) {
    let mut w = Workflow::new();
    let a = w.add_node("http-handler", 0.0, 0.0);
    let b = w.add_node("run", 100.0, 0.0);
    let c = w.add_node("condition", 200.0, 0.0);
    (w, a, b, c)
}

// ═══════════════════════════════════════════════════════════════════════════
// 1. Node Selected Flag Lifecycle
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn new_nodes_are_not_selected() {
    let mut w = Workflow::new();
    let id = w.add_node("run", 0.0, 0.0);

    let node = w.nodes.iter().find(|n| n.id == id).expect("node exists");
    assert!(!node.selected, "newly added nodes should not be selected");
}

#[test]
fn set_selected_true_marks_node_as_selected() {
    let mut w = Workflow::new();
    let id = w.add_node("run", 0.0, 0.0);

    w.nodes.iter_mut().find(|n| n.id == id).expect("node").set_selected(true);

    assert!(
        w.nodes.iter().find(|n| n.id == id).expect("node").selected,
        "node should be selected after set_selected(true)"
    );
}

#[test]
fn set_selected_false_clears_selection() {
    let mut w = Workflow::new();
    let id = w.add_node("run", 0.0, 0.0);

    w.nodes.iter_mut().find(|n| n.id == id).expect("node").set_selected(true);
    w.nodes.iter_mut().find(|n| n.id == id).expect("node").set_selected(false);

    assert!(
        !w.nodes.iter().find(|n| n.id == id).expect("node").selected,
        "node should not be selected after set_selected(false)"
    );
}

#[test]
fn selection_flag_is_independent_per_node() {
    let mut w = Workflow::new();
    let a = w.add_node("run", 0.0, 0.0);
    let b = w.add_node("run", 100.0, 0.0);
    let c = w.add_node("run", 200.0, 0.0);

    // Select only A and C
    w.nodes.iter_mut().find(|n| n.id == a).expect("a").set_selected(true);
    w.nodes.iter_mut().find(|n| n.id == c).expect("c").set_selected(true);

    assert!(w.nodes.iter().find(|n| n.id == a).expect("a").selected);
    assert!(!w.nodes.iter().find(|n| n.id == b).expect("b").selected);
    assert!(w.nodes.iter().find(|n| n.id == c).expect("c").selected);
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. Workflow deselect_all
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn deselect_all_clears_every_node() {
    let (mut w, _a, _b, _c) = workflow_with_three_nodes();

    // Select all
    for node in &mut w.nodes {
        node.set_selected(true);
    }
    assert!(w.nodes.iter().all(|n| n.selected));

    w.deselect_all();

    assert!(
        w.nodes.iter().all(|n| !n.selected),
        "deselect_all must clear all selected flags"
    );
}

#[test]
fn deselect_all_on_empty_workflow_is_noop() {
    let mut w = Workflow::new();
    w.deselect_all(); // Should not panic
}

#[test]
fn deselect_all_does_not_remove_nodes() {
    let (mut w, a, b, c) = workflow_with_three_nodes();

    for node in &mut w.nodes {
        node.set_selected(true);
    }
    w.deselect_all();

    assert_eq!(w.nodes.len(), 3, "deselect_all must not remove nodes");
    assert!(w.nodes.iter().any(|n| n.id == a));
    assert!(w.nodes.iter().any(|n| n.id == b));
    assert!(w.nodes.iter().any(|n| n.id == c));
}

#[test]
fn deselect_all_on_already_deselected_is_noop() {
    let (mut w, _a, _b, _c) = workflow_with_three_nodes();
    // Nodes are already not selected
    w.deselect_all();
    assert!(w.nodes.iter().all(|n| !n.selected));
}

#[test]
fn deselect_all_does_not_affect_connections() {
    let (mut w, a, b, _c) = workflow_with_three_nodes();
    let mp = main_port();
    let _ = w.add_connection_checked(a, b, &mp, &mp);
    let conn_count = w.connections.len();

    for node in &mut w.nodes {
        node.set_selected(true);
    }
    w.deselect_all();

    assert_eq!(w.connections.len(), conn_count, "connections must be preserved");
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. Selection Survives Workflow Operations
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn selected_flag_survives_node_position_update() {
    let mut w = Workflow::new();
    let a = w.add_node("run", 10.0, 20.0);

    w.nodes.iter_mut().find(|n| n.id == a).expect("a").set_selected(true);

    w.update_node_position(a, 50.0, 60.0);

    let node = w.nodes.iter().find(|n| n.id == a).expect("a");
    assert!(node.selected, "selected flag should survive position update");
    assert_ne!((node.x, node.y), (10.0_f32, 20.0_f32), "position should have changed");
}

#[test]
fn removing_unselected_node_does_not_affect_other_selections() {
    let mut w = Workflow::new();
    let a = w.add_node("run", 0.0, 0.0);
    let b = w.add_node("run", 100.0, 0.0);
    let c = w.add_node("run", 200.0, 0.0);

    w.nodes.iter_mut().find(|n| n.id == a).expect("a").set_selected(true);
    w.nodes.iter_mut().find(|n| n.id == c).expect("c").set_selected(true);

    w.remove_node(b);

    assert!(w.nodes.iter().find(|n| n.id == a).expect("a").selected);
    assert!(w.nodes.iter().find(|n| n.id == c).expect("c").selected);
}

#[test]
fn removing_selected_node_removes_its_flag() {
    let mut w = Workflow::new();
    let a = w.add_node("run", 0.0, 0.0);
    let _b = w.add_node("run", 100.0, 0.0);

    w.nodes.iter_mut().find(|n| n.id == a).expect("a").set_selected(true);

    w.remove_node(a);

    assert!(!w.nodes.iter().any(|n| n.id == a), "node A should be gone");
    assert!(!w.nodes.iter().any(|n| n.selected), "no remaining nodes should be selected");
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. Selection After Add Node (no auto-select)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn adding_node_does_not_auto_select() {
    let mut w = Workflow::new();
    let a = w.add_node("run", 0.0, 0.0);

    // Select node A
    w.nodes.iter_mut().find(|n| n.id == a).expect("a").set_selected(true);

    // Add another node
    let b = w.add_node("run", 100.0, 0.0);

    assert!(w.nodes.iter().find(|n| n.id == a).expect("a").selected, "A stays selected");
    assert!(!w.nodes.iter().find(|n| n.id == b).expect("b").selected, "new node B is not auto-selected");
}

#[test]
fn adding_multiple_nodes_none_are_selected() {
    let mut w = Workflow::new();
    for i in 0..5 {
        w.add_node("run", (i * 100) as f32, 0.0);
    }

    assert!(
        w.nodes.iter().all(|n| !n.selected),
        "no newly added nodes should be selected"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. Selection + Connection Operations
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn adding_connection_does_not_change_selection() {
    let mut w = Workflow::new();
    let a = w.add_node("run", 0.0, 0.0);
    let b = w.add_node("run", 100.0, 0.0);

    w.nodes.iter_mut().find(|n| n.id == a).expect("a").set_selected(true);
    let mp = main_port();
    let _ = w.add_connection_checked(a, b, &mp, &mp);

    assert!(w.nodes.iter().find(|n| n.id == a).expect("a").selected, "A stays selected");
    assert!(!w.nodes.iter().find(|n| n.id == b).expect("b").selected, "B stays unselected");
}

#[test]
fn selection_count_matches_flagged_nodes() {
    let (mut w, _a, _b, _c) = workflow_with_three_nodes();

    // Select two out of three
    let ids: Vec<NodeId> = w.nodes.iter().take(2).map(|n| n.id).collect();
    for id in &ids {
        w.nodes.iter_mut().find(|n| &n.id == id).expect("node").set_selected(true);
    }

    let selected_count = w.nodes.iter().filter(|n| n.selected).count();
    assert_eq!(selected_count, 2, "exactly 2 nodes should be selected");
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. Selection Serialization Round-Trip
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn selected_flag_survives_serde_round_trip() {
    let mut w = Workflow::new();
    let a = w.add_node("run", 0.0, 0.0);
    let b = w.add_node("run", 100.0, 0.0);

    w.nodes.iter_mut().find(|n| n.id == a).expect("a").set_selected(true);

    let json = serde_json::to_string(&w).unwrap();
    let restored: Workflow = serde_json::from_str(&json).unwrap();

    let restored_a = restored.nodes.iter().find(|n| n.id == a).expect("a");
    let restored_b = restored.nodes.iter().find(|n| n.id == b).expect("b");

    assert!(restored_a.selected, "A's selected flag should survive round-trip");
    assert!(!restored_b.selected, "B's selected flag should survive round-trip");
}

#[test]
fn deselected_state_survives_serde_round_trip() {
    let mut w = Workflow::new();
    let id = w.add_node("run", 0.0, 0.0);
    // Node is not selected (default)

    let json = serde_json::to_string(&w).unwrap();
    let restored: Workflow = serde_json::from_str(&json).unwrap();

    assert!(
        !restored.nodes.iter().find(|n| n.id == id).expect("node").selected,
        "deselected state should survive round-trip"
    );
}
