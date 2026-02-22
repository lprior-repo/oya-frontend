use anyhow::{anyhow, Result};
use oya_frontend::flow_extender::{
    apply_extension, preview_extension, suggest_extensions, suggest_extensions_with_analysis,
    RestateCapability, RestateServiceKind,
};
use oya_frontend::graph::PortName;
use oya_frontend::Workflow;

fn has_key(workflow: &Workflow, key: &str) -> bool {
    suggest_extensions(workflow)
        .iter()
        .any(|item| item.key == key)
}

#[test]
fn restate_semantic_tags_contract() -> Result<()> {
    let workflow = Workflow::new();

    let analysis = suggest_extensions_with_analysis(&workflow)
        .into_iter()
        .find(|item| item.key == "add-entry-trigger")
        .ok_or_else(|| anyhow!("entry analysis should exist"))?;

    assert!(analysis
        .semantics
        .compatible_service_kinds
        .contains(&RestateServiceKind::Service));
    assert!(analysis
        .semantics
        .compatible_service_kinds
        .contains(&RestateServiceKind::VirtualObject));
    assert!(analysis
        .semantics
        .compatible_service_kinds
        .contains(&RestateServiceKind::Workflow));
    assert!(analysis
        .semantics
        .provides
        .contains(&RestateCapability::EntryTrigger));
    Ok(())
}

#[test]
fn restate_semantic_guardrails_contract() -> Result<()> {
    let mut workflow = Workflow::new();
    let _ = workflow.add_node("run", 20.0, 20.0);
    let _ = workflow.add_node("condition", 60.0, 20.0);
    let _ = workflow.add_node("awakeable", 100.0, 20.0);

    let keys = suggest_extensions(&workflow)
        .into_iter()
        .map(|item| item.key)
        .collect::<Vec<_>>();

    assert!(!keys.iter().any(|key| key == "add-durable-checkpoint"));
    assert!(!keys.iter().any(|key| key == "add-compensation-branch"));
    assert!(!keys.iter().any(|key| key == "add-signal-resolution"));
    Ok(())
}

#[test]
fn entry_trigger_contract() -> Result<()> {
    let mut workflow = Workflow::new();
    let run_id = workflow.add_node("run", 0.0, 0.0);

    assert!(has_key(&workflow, "add-entry-trigger"));

    let preview = preview_extension(&workflow, "add-entry-trigger")
        .map_err(|err| anyhow!(err))?
        .ok_or_else(|| anyhow!("entry preview should exist"))?;
    assert_eq!(preview.nodes.len(), 1);
    assert_eq!(preview.nodes[0].node_type, "http-handler");

    let initial_connections = workflow.connections.clone();
    let apply =
        apply_extension(&mut workflow, "add-entry-trigger").map_err(|err| anyhow!("{err}"))?;

    assert!(!apply.created_nodes.is_empty());
    assert!(workflow
        .nodes
        .iter()
        .any(|node| node.node_type == "http-handler"));
    assert!(workflow.nodes.iter().any(|node| node.id == run_id));
    assert_eq!(workflow.connections, initial_connections);
    Ok(())
}

#[test]
fn timeout_guard_contract() -> Result<()> {
    let mut workflow = Workflow::new();
    let durable_id = workflow.add_node("run", 10.0, 10.0);

    assert!(has_key(&workflow, "add-reliability-bundle"));

    let preview = preview_extension(&workflow, "add-timeout-guard")
        .map_err(|err| anyhow!(err))?
        .ok_or_else(|| anyhow!("timeout preview should exist"))?;
    assert_eq!(preview.nodes[0].node_type, "timeout");
    assert_eq!(preview.connections.len(), 1);

    let _ = apply_extension(&mut workflow, "add-timeout-guard").map_err(|err| anyhow!("{err}"))?;

    assert!(workflow
        .nodes
        .iter()
        .any(|node| node.node_type == "timeout"));
    assert!(workflow
        .connections
        .iter()
        .any(|connection| connection.source == durable_id && connection.source_port.0 == "out"));
    assert!(workflow
        .connections
        .iter()
        .all(|connection| connection.source != connection.target));
    Ok(())
}

#[test]
fn durable_checkpoint_contract() -> Result<()> {
    let mut workflow = Workflow::new();
    let _ = workflow.add_node("get-state", 0.0, 0.0);
    let durable_id = workflow.add_node("run", 30.0, 30.0);

    assert!(has_key(&workflow, "add-reliability-bundle"));

    let preview = preview_extension(&workflow, "add-durable-checkpoint")
        .map_err(|err| anyhow!(err))?
        .ok_or_else(|| anyhow!("checkpoint preview should exist"))?;
    assert_eq!(preview.nodes[0].node_type, "set-state");

    let initial_nodes = workflow.nodes.clone();
    let _ =
        apply_extension(&mut workflow, "add-durable-checkpoint").map_err(|err| anyhow!("{err}"))?;

    assert!(workflow
        .nodes
        .iter()
        .any(|node| node.node_type == "set-state"));
    assert!(initial_nodes
        .iter()
        .all(|existing| workflow.nodes.iter().any(|node| node.id == existing.id)));
    assert!(workflow
        .connections
        .iter()
        .any(|connection| connection.source == durable_id && connection.target_port.0 == "in"));
    Ok(())
}

#[test]
fn compensation_branch_contract() -> Result<()> {
    let mut workflow = Workflow::new();
    let _ = workflow.add_node("durable-promise", 40.0, 90.0);
    let condition_id = workflow.add_node("condition", 100.0, 100.0);
    let true_branch = workflow.add_node("run", 240.0, 60.0);
    let _ = workflow.add_connection(
        condition_id,
        true_branch,
        &PortName::from("true"),
        &PortName::from("in"),
    );

    assert!(has_key(&workflow, "add-reliability-bundle"));

    let preview = preview_extension(&workflow, "add-compensation-branch")
        .map_err(|err| anyhow!(err))?
        .ok_or_else(|| anyhow!("compensate preview should exist"))?;
    assert_eq!(preview.nodes[0].node_type, "compensate");
    assert_eq!(preview.connections[0].source_port, "false");

    let _ = apply_extension(&mut workflow, "add-compensation-branch")
        .map_err(|err| anyhow!("{err}"))?;

    let compensate_node_id = workflow
        .nodes
        .iter()
        .find(|node| node.node_type == "compensate")
        .map(|node| node.id)
        .ok_or_else(|| anyhow!("compensate node should exist"))?;
    assert!(workflow.connections.iter().any(|connection| {
        connection.source == condition_id
            && connection.target == compensate_node_id
            && connection.source_port.0 == "false"
    }));
    Ok(())
}

#[test]
fn signal_resolution_contract() -> Result<()> {
    let mut workflow = Workflow::new();
    let wait_id = workflow.add_node("durable-promise", 50.0, 75.0);
    let durable_id = workflow.add_node("run", 280.0, 75.0);

    assert!(has_key(&workflow, "add-signal-resolution"));

    let before_order = workflow
        .nodes
        .iter()
        .map(|node| node.id)
        .collect::<Vec<_>>();
    let preview = preview_extension(&workflow, "add-signal-resolution")
        .map_err(|err| anyhow!(err))?
        .ok_or_else(|| anyhow!("resolve preview should exist"))?;
    assert_eq!(preview.nodes[0].node_type, "resolve-promise");

    let _ =
        apply_extension(&mut workflow, "add-signal-resolution").map_err(|err| anyhow!("{err}"))?;

    assert!(workflow
        .nodes
        .iter()
        .any(|node| node.node_type == "resolve-promise"));
    assert!(workflow.connections.iter().any(|connection| {
        connection.source == wait_id
            && connection.source_port.0 == "out"
            && connection.target_port.0 == "in"
    }));
    let after_order = workflow
        .nodes
        .iter()
        .map(|node| node.id)
        .collect::<Vec<_>>();
    assert_eq!(after_order[0], before_order[0]);
    assert_eq!(after_order[1], durable_id);
    Ok(())
}
