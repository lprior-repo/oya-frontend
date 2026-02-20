use oya_frontend::graph::{Connection, PortName, Workflow};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

fn snapshot_positions(workflow: &Workflow) -> HashMap<String, (f32, f32)> {
    workflow
        .nodes
        .iter()
        .map(|node| (node.id.to_string(), (node.x, node.y)))
        .collect::<HashMap<_, _>>()
}

fn same_positions(left: &HashMap<String, (f32, f32)>, right: &HashMap<String, (f32, f32)>) -> bool {
    left.len() == right.len()
        && left.iter().all(|(id, (lx, ly))| {
            right.get(id).is_some_and(|(rx, ry)| {
                (lx - rx).abs() < 0.000_1_f32 && (ly - ry).abs() < 0.000_1_f32
            })
        })
}

fn build_branching_workflow() -> Workflow {
    let mut workflow = Workflow::new();
    let start = workflow.add_node("http-handler", 20.0, 20.0);
    let branch = workflow.add_node("condition", 40.0, 120.0);
    let left = workflow.add_node("custom-left", 10.0, 260.0);
    let right = workflow.add_node("custom-right", 260.0, 260.0);
    let join = workflow.add_node("custom-join", 140.0, 420.0);

    let main = PortName("main".to_string());
    let _ = workflow.add_connection(start, branch, &main, &main);
    let _ = workflow.add_connection(branch, left, &main, &main);
    let _ = workflow.add_connection(branch, right, &main, &main);
    let _ = workflow.add_connection(left, join, &main, &main);
    let _ = workflow.add_connection(right, join, &main, &main);
    workflow
}

#[test]
fn auto_layout_is_idempotent_for_branching_graph() {
    let mut workflow = build_branching_workflow();

    workflow.apply_layout();
    let first = snapshot_positions(&workflow);
    workflow.apply_layout();
    let second = snapshot_positions(&workflow);

    assert!(same_positions(&first, &second));
}

#[test]
fn auto_layout_is_idempotent_for_disconnected_components() {
    let mut workflow = Workflow::new();

    let a1 = workflow.add_node("a1", 0.0, 0.0);
    let a2 = workflow.add_node("a2", 0.0, 0.0);
    let b1 = workflow.add_node("b1", 0.0, 0.0);
    let b2 = workflow.add_node("b2", 0.0, 0.0);
    let isolated = workflow.add_node("isolated", 0.0, 0.0);

    let main = PortName("main".to_string());
    let _ = workflow.add_connection(a1, a2, &main, &main);
    let _ = workflow.add_connection(b1, b2, &main, &main);
    let _ = workflow.add_connection(a1, isolated, &main, &main);

    workflow.apply_layout();
    let first = snapshot_positions(&workflow);
    workflow.apply_layout();
    let second = snapshot_positions(&workflow);

    assert!(same_positions(&first, &second));
}

#[test]
fn auto_layout_keeps_nodes_in_positive_canvas_space() {
    let mut workflow = build_branching_workflow();
    workflow.apply_layout();

    assert!(workflow
        .nodes
        .iter()
        .all(|node| node.x >= 100.0_f32 && node.y >= 70.0_f32));
}

#[test]
fn prepare_run_resets_runtime_state_to_pending() {
    let mut workflow = Workflow::new();
    let n1 = workflow.add_node("step-one", 20.0, 20.0);
    let n2 = workflow.add_node("step-two", 60.0, 120.0);

    if let Some(node) = workflow.nodes.iter_mut().find(|node| node.id == n1) {
        node.error = Some("boom".to_string());
        node.executing = true;
        node.skipped = true;
        node.last_output = Some(json!({"old": true}));
        node.config = json!({"status": "failed"});
    }

    let main = PortName("main".to_string());
    let _ = workflow.add_connection(n1, n2, &main, &main);

    workflow.prepare_run();

    assert!(workflow.nodes.iter().all(|node| !node.executing));
    assert!(workflow.nodes.iter().all(|node| !node.skipped));
    assert!(workflow.nodes.iter().all(|node| node.error.is_none()));
    assert!(workflow.nodes.iter().all(|node| node.last_output.is_none()));
    assert!(workflow.nodes.iter().all(|node| {
        node.config
            .get("status")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|status| status == "pending")
    }));
}

#[tokio::test]
async fn run_marks_nodes_completed_and_records_history() {
    let mut workflow = Workflow::new();
    let start = workflow.add_node("start-custom", 20.0, 20.0);
    let next = workflow.add_node("next-custom", 50.0, 110.0);
    let main = PortName("main".to_string());
    let _ = workflow.add_connection(start, next, &main, &main);

    workflow.run().await;

    assert_eq!(workflow.current_step, workflow.execution_queue.len());
    assert_eq!(workflow.history.len(), 1);
    assert!(workflow.nodes.iter().all(|node| !node.executing));
    assert!(workflow.nodes.iter().all(|node| node.error.is_none()));
    assert!(workflow.nodes.iter().all(|node| {
        node.config
            .get("status")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|status| status == "completed")
    }));
}

#[tokio::test]
async fn condition_step_marks_false_branch_as_skipped() {
    let mut workflow = Workflow::new();
    let trigger = workflow.add_node("trigger", 0.0, 0.0);
    let condition = workflow.add_node("condition", 0.0, 0.0);
    let true_branch = workflow.add_node("true-branch", 0.0, 0.0);
    let false_branch = workflow.add_node("false-branch", 0.0, 0.0);

    if let Some(node) = workflow.nodes.iter_mut().find(|node| node.id == condition) {
        node.config = json!({"condition": "true"});
    }

    let main = PortName("main".to_string());
    let _ = workflow.add_connection(trigger, condition, &main, &main);
    workflow.connections.push(Connection {
        id: Uuid::new_v4(),
        source: condition,
        target: true_branch,
        source_port: PortName("true".to_string()),
        target_port: PortName("main".to_string()),
    });
    workflow.connections.push(Connection {
        id: Uuid::new_v4(),
        source: condition,
        target: false_branch,
        source_port: PortName("false".to_string()),
        target_port: PortName("main".to_string()),
    });

    workflow.run().await;

    let false_node = workflow
        .nodes
        .iter()
        .find(|node| node.id == false_branch)
        .cloned();
    let true_node = workflow
        .nodes
        .iter()
        .find(|node| node.id == true_branch)
        .cloned();

    assert!(false_node.as_ref().is_some_and(|node| node.skipped));
    assert!(false_node.as_ref().is_some_and(|node| {
        node.config
            .get("status")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|status| status == "skipped")
    }));
    assert!(true_node.as_ref().is_some_and(|node| !node.skipped));
}

#[test]
fn fit_view_is_idempotent_for_same_bounds() {
    let mut workflow = build_branching_workflow();
    workflow.apply_layout();

    workflow.fit_view(1200.0, 720.0, 180.0);
    let first = workflow.viewport.clone();
    workflow.fit_view(1200.0, 720.0, 180.0);
    let second = workflow.viewport.clone();

    assert!((first.x - second.x).abs() < 0.000_1_f32);
    assert!((first.y - second.y).abs() < 0.000_1_f32);
    assert!((first.zoom - second.zoom).abs() < 0.000_1_f32);
}

#[test]
fn add_connection_rejects_invalid_and_duplicate_edges() {
    let mut workflow = Workflow::new();
    let a = workflow.add_node("a", 0.0, 0.0);
    let b = workflow.add_node("b", 0.0, 0.0);
    let main = PortName("main".to_string());

    let self_edge = workflow.add_connection(a, a, &main, &main);
    let first = workflow.add_connection(a, b, &main, &main);
    let duplicate = workflow.add_connection(a, b, &main, &main);

    assert!(!self_edge);
    assert!(first);
    assert!(!duplicate);
    assert_eq!(workflow.connections.len(), 1);
}

#[test]
fn remove_node_also_removes_incident_connections() {
    let mut workflow = Workflow::new();
    let a = workflow.add_node("a", 0.0, 0.0);
    let b = workflow.add_node("b", 0.0, 0.0);
    let c = workflow.add_node("c", 0.0, 0.0);
    let main = PortName("main".to_string());

    let _ = workflow.add_connection(a, b, &main, &main);
    let _ = workflow.add_connection(b, c, &main, &main);
    let _ = workflow.add_connection(a, c, &main, &main);

    workflow.remove_node(b);

    assert_eq!(workflow.nodes.len(), 2);
    assert!(workflow.nodes.iter().all(|node| node.id != b));
    assert!(workflow
        .connections
        .iter()
        .all(|conn| conn.source != b && conn.target != b));
}

#[tokio::test]
async fn history_is_capped_to_ten_runs() {
    let mut workflow = Workflow::new();
    let _ = workflow.add_node("single", 0.0, 0.0);

    for _ in 0..12 {
        workflow.run().await;
    }

    assert_eq!(workflow.history.len(), 10);
}

#[tokio::test]
async fn failed_http_request_marks_run_as_unsuccessful() {
    let mut workflow = Workflow::new();
    let node_id = workflow.add_node("http-request", 0.0, 0.0);

    if let Some(node) = workflow.nodes.iter_mut().find(|node| node.id == node_id) {
        node.config = json!({"url": "http://127.0.0.1:0", "method": "GET"});
    }

    workflow.run().await;

    assert_eq!(workflow.history.len(), 1);
    assert!(!workflow.history[0].success);
    assert!(workflow
        .nodes
        .iter()
        .find(|node| node.id == node_id)
        .is_some_and(|node| node.error.is_some()));
}
