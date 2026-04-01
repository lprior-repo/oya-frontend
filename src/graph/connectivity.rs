use uuid::Uuid;

use super::{Connection, NodeId, PortName, Workflow};
use crate::graph::restate_types::PortType;
use crate::graph::restate_types::{types_compatible, ParsePortTypeError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionError {
    SelfConnection,
    MissingSourceNode(NodeId),
    MissingTargetNode(NodeId),
    WouldCreateCycle,
    Duplicate,
    TypeMismatch {
        source_type: PortType,
        target_type: PortType,
    },
    ParseError(ParsePortTypeError),
}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelfConnection => write!(f, "Cannot connect node to itself"),
            Self::MissingSourceNode(node_id) => {
                write!(f, "Source node not found: {node_id}")
            }
            Self::MissingTargetNode(node_id) => {
                write!(f, "Target node not found: {node_id}")
            }
            Self::WouldCreateCycle => write!(f, "Connection would create a cycle"),
            Self::Duplicate => write!(f, "Connection already exists"),
            Self::TypeMismatch {
                source_type,
                target_type,
            } => write!(
                f,
                "Type mismatch: {source_type} is not compatible with {target_type}"
            ),
            Self::ParseError(err) => write!(f, "Parse error: {err}"),
        }
    }
}

impl std::error::Error for ConnectionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionResult {
    Created,
}

/// Internal state representing a validated connection ready for mutation.
#[derive(Debug, Clone)]
struct ValidationState {
    source: NodeId,
    target: NodeId,
    source_port: PortName,
    target_port: PortName,
}

impl Workflow {
    /// Adds a connection with full type checking.
    ///
    /// Returns `true` if the connection was successfully added, `false` otherwise.
    ///
    /// # See Also
    ///
    /// Use `add_connection_checked` for detailed error information.
    pub fn add_connection(
        &mut self,
        source: NodeId,
        target: NodeId,
        source_port: &PortName,
        target_port: &PortName,
    ) -> bool {
        self.add_connection_checked(source, target, source_port, target_port)
            .is_ok()
    }

    /// Adds a connection with full type checking.
    ///
    /// # Errors
    ///
    /// Returns `ConnectionError` if:
    /// - `source` and `target` are the same node
    /// - Either endpoint does not exist in the workflow
    /// - The connection would create a cycle
    /// - An identical connection already exists
    /// - Source and target port types are incompatible
    pub fn add_connection_checked(
        &mut self,
        source: NodeId,
        target: NodeId,
        source_port: &PortName,
        target_port: &PortName,
    ) -> Result<ConnectionResult, ConnectionError> {
        let validation = Self::validate_connection(
            &self.nodes,
            &self.connections,
            source,
            target,
            source_port,
            target_port,
        )?;
        Self::commit_connection(&mut self.connections, validation);
        Ok(ConnectionResult::Created)
    }

    /// Validates that a connection can be added without mutating state.
    ///
    /// # Errors
    ///
    /// Returns `ConnectionError` if validation fails:
    /// - Self-connection detected
    /// - Source or target node missing
    /// - Cycle would be created
    /// - Connection already exists
    /// - Port types incompatible
    fn validate_connection(
        nodes: &[super::Node],
        connections: &[Connection],
        source: NodeId,
        target: NodeId,
        source_port: &PortName,
        target_port: &PortName,
    ) -> Result<ValidationState, ConnectionError> {
        if source == target {
            return Err(ConnectionError::SelfConnection);
        }

        // Single-pass O(n) lookup instead of two separate .iter().any() scans
        let source_found = nodes.iter().any(|node| node.id == source);
        let target_found = nodes.iter().any(|node| node.id == target);

        if !source_found {
            return Err(ConnectionError::MissingSourceNode(source));
        }
        if !target_found {
            return Err(ConnectionError::MissingTargetNode(target));
        }

        if Self::path_exists(connections, target, source) {
            return Err(ConnectionError::WouldCreateCycle);
        }

        if connections.iter().any(|c| {
            c.source == source
                && c.target == target
                && c.source_port == *source_port
                && c.target_port == *target_port
        }) {
            return Err(ConnectionError::Duplicate);
        }

        Self::check_port_type_compatibility(nodes, source, target)?;

        Ok(ValidationState {
            source,
            target,
            source_port: source_port.clone(),
            target_port: target_port.clone(),
        })
    }

    /// Commits a validated connection to the graph.
    ///
    /// # Safety
    ///
    /// Only call this after `validate_connection` has succeeded.
    fn commit_connection(connections: &mut Vec<Connection>, validation: ValidationState) {
        connections.push(Connection {
            id: Uuid::new_v4(),
            source: validation.source,
            target: validation.target,
            source_port: validation.source_port,
            target_port: validation.target_port,
        });
    }

    /// Returns the validated connection state for mutation.
    #[must_use]
    fn new_connection(validation: &ValidationState) -> Connection {
        Connection {
            id: Uuid::new_v4(),
            source: validation.source,
            target: validation.target,
            source_port: validation.source_port.clone(),
            target_port: validation.target_port.clone(),
        }
    }

    /// Single-pass node lookup: finds both source and target in one iteration
    /// instead of two separate O(n) scans.
    fn find_source_and_target_nodes(
        nodes: &[super::Node],
        source: NodeId,
        target: NodeId,
    ) -> Result<(&super::Node, &super::Node), ConnectionError> {
        let mut source_node = None;
        let mut target_node = None;

        for node in nodes {
            if node.id == source {
                source_node = Some(node);
            }
            if node.id == target {
                target_node = Some(node);
            }
            // Early exit once both are found
            if source_node.is_some() && target_node.is_some() {
                break;
            }
        }

        let source = source_node.ok_or(ConnectionError::MissingSourceNode(source))?;
        let target = target_node.ok_or(ConnectionError::MissingTargetNode(target))?;
        Ok((source, target))
    }

    fn check_port_type_compatibility(
        nodes: &[super::Node],
        source: NodeId,
        target: NodeId,
    ) -> Result<(), ConnectionError> {
        let (source_node, target_node) =
            Self::find_source_and_target_nodes(nodes, source, target)?;

        let source_type = Self::get_node_output_port_type(source_node)?;
        let target_type = Self::get_node_input_port_type(target_node)?;

        if !types_compatible(source_type, target_type) {
            return Err(ConnectionError::TypeMismatch {
                source_type,
                target_type,
            });
        }

        Ok(())
    }

    fn get_node_output_port_type(node: &super::Node) -> Result<PortType, ConnectionError> {
        node.node_type
            .parse::<super::workflow_node::WorkflowNode>()
            .map_err(|_| ConnectionError::ParseError(ParsePortTypeError(node.node_type.clone())))
            .map(|workflow_node| workflow_node.output_port_type())
    }

    fn get_node_input_port_type(node: &super::Node) -> Result<PortType, ConnectionError> {
        node.node_type
            .parse::<super::workflow_node::WorkflowNode>()
            .map_err(|_| ConnectionError::ParseError(ParsePortTypeError(node.node_type.clone())))
            .map(|workflow_node| workflow_node.input_port_type())
    }

    #[must_use]
    pub fn path_exists(connections: &[Connection], from: NodeId, to: NodeId) -> bool {
        // If from == to, only return true if there's a self-loop
        if from == to {
            return connections
                .iter()
                .any(|conn| conn.source == from && conn.target == from);
        }

        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![from];

        while let Some(current) = stack.pop() {
            if current == to {
                return true;
            }

            if visited.insert(current) {
                connections
                    .iter()
                    .filter(|connection| connection.source == current)
                    .for_each(|connection| stack.push(connection.target));
            }
        }

        false
    }
}

/// Helper function for tests to access `path_exists` directly
#[must_use]
pub fn path_exists_internal(connections: &[Connection], from: NodeId, to: NodeId) -> bool {
    Workflow::path_exists(connections, from, to)
}

/// Helper function for tests to access `check_port_type_compatibility` directly
///
/// # Errors
///
/// Returns `ConnectionError::MissingSourceNode` if source not found.
/// Returns `ConnectionError::MissingTargetNode` if target not found.
/// Returns `ConnectionError::TypeMismatch` if types incompatible.
pub fn check_port_type_compatibility_internal(
    nodes: &[super::Node],
    source: NodeId,
    target: NodeId,
) -> Result<(), ConnectionError> {
    Workflow::check_port_type_compatibility(nodes, source, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn given_self_connection_when_adding_checked_connection_then_self_connection_error_is_returned()
    {
        let mut workflow = Workflow::new();
        let node_id = workflow.add_node("http-handler", 0.0, 0.0);
        let main = PortName("main".to_string());

        let result = workflow.add_connection_checked(node_id, node_id, &main, &main);

        assert_eq!(result, Err(ConnectionError::SelfConnection));
    }

    #[test]
    fn given_duplicate_connection_when_adding_checked_connection_then_duplicate_error_is_returned()
    {
        let mut workflow = Workflow::new();
        let source = workflow.add_node("http-handler", 0.0, 0.0);
        let target = workflow.add_node("run", 100.0, 0.0);
        let main = PortName("main".to_string());

        let first = workflow.add_connection_checked(source, target, &main, &main);
        assert!(matches!(first, Ok(ConnectionResult::Created)));

        let duplicate = workflow.add_connection_checked(source, target, &main, &main);

        assert_eq!(duplicate, Err(ConnectionError::Duplicate));
    }

    #[test]
    fn given_back_edge_when_adding_checked_connection_then_cycle_error_is_returned() {
        let mut workflow = Workflow::new();
        let first = workflow.add_node("http-handler", 0.0, 0.0);
        let second = workflow.add_node("run", 100.0, 0.0);
        let main = PortName("main".to_string());

        let created = workflow.add_connection_checked(first, second, &main, &main);
        assert!(matches!(created, Ok(ConnectionResult::Created)));

        let cycle = workflow.add_connection_checked(second, first, &main, &main);

        assert_eq!(cycle, Err(ConnectionError::WouldCreateCycle));
    }

    #[test]
    fn given_type_mismatch_ports_when_adding_checked_connection_then_type_mismatch_error_is_returned(
    ) {
        let mut workflow = Workflow::new();
        let source = workflow.add_node("condition", 0.0, 0.0);
        let target = workflow.add_node("signal-handler", 100.0, 0.0);
        let main = PortName("main".to_string());

        let result = workflow.add_connection_checked(source, target, &main, &main);

        assert_eq!(
            result,
            Err(ConnectionError::TypeMismatch {
                source_type: PortType::FlowControl,
                target_type: PortType::Signal,
            })
        );
    }

    #[test]
    fn given_missing_source_when_adding_checked_connection_then_source_not_found_error_is_returned()
    {
        let mut workflow = Workflow::new();
        let target = workflow.add_node("run", 100.0, 0.0);
        let main = PortName("main".to_string());

        let missing_source = NodeId(Uuid::new_v4());
        let result = workflow.add_connection_checked(missing_source, target, &main, &main);

        assert_eq!(
            result,
            Err(ConnectionError::MissingSourceNode(missing_source))
        );
    }

    #[test]
    fn given_missing_target_when_adding_checked_connection_then_target_not_found_error_is_returned()
    {
        let mut workflow = Workflow::new();
        let source = workflow.add_node("http-handler", 0.0, 0.0);
        let main = PortName("main".to_string());

        let missing_target = NodeId(Uuid::new_v4());
        let result = workflow.add_connection_checked(source, missing_target, &main, &main);

        assert_eq!(
            result,
            Err(ConnectionError::MissingTargetNode(missing_target))
        );
    }

    #[test]
    fn given_compatible_ports_when_adding_checked_connection_then_connection_is_created() {
        let mut workflow = Workflow::new();
        let source = workflow.add_node("http-handler", 0.0, 0.0);
        let target = workflow.add_node("run", 100.0, 0.0);
        let main = PortName("main".to_string());

        let result = workflow.add_connection_checked(source, target, &main, &main);

        assert_eq!(result, Ok(ConnectionResult::Created));
        assert_eq!(workflow.connections.len(), 1);
    }

    // ---------------------------------------------------------------------------
    // Additional edge-case tests for validate_connection
    // ---------------------------------------------------------------------------

    #[test]
    fn given_both_nodes_missing_when_adding_checked_connection_then_source_not_found_error_is_returned()
    {
        let mut workflow = Workflow::new();
        let ghost_source = NodeId(Uuid::new_v4());
        let ghost_target = NodeId(Uuid::new_v4());
        let main = PortName("main".to_string());

        // Source is checked first, so MissingSourceNode should be returned
        let result =
            workflow.add_connection_checked(ghost_source, ghost_target, &main, &main);

        assert_eq!(
            result,
            Err(ConnectionError::MissingSourceNode(ghost_source))
        );
    }

    #[test]
    fn given_missing_source_and_existing_target_when_adding_checked_connection_then_source_not_found_error_is_returned(
    ) {
        let mut workflow = Workflow::new();
        let existing_target = workflow.add_node("run", 0.0, 0.0);
        let ghost_source = NodeId(Uuid::new_v4());
        let main = PortName("main".to_string());

        let result =
            workflow.add_connection_checked(ghost_source, existing_target, &main, &main);

        assert_eq!(
            result,
            Err(ConnectionError::MissingSourceNode(ghost_source))
        );
    }

    #[test]
    fn given_existing_source_and_missing_target_when_adding_checked_connection_then_target_not_found_error_is_returned(
    ) {
        let mut workflow = Workflow::new();
        let existing_source = workflow.add_node("http-handler", 0.0, 0.0);
        let ghost_target = NodeId(Uuid::new_v4());
        let main = PortName("main".to_string());

        let result =
            workflow.add_connection_checked(existing_source, ghost_target, &main, &main);

        assert_eq!(
            result,
            Err(ConnectionError::MissingTargetNode(ghost_target))
        );
    }

    #[test]
    fn given_self_loop_on_different_port_names_when_adding_checked_connection_then_self_connection_error_is_returned(
    ) {
        let mut workflow = Workflow::new();
        let node = workflow.add_node("run", 0.0, 0.0);
        let port_a = PortName("output_a".to_string());
        let port_b = PortName("input_b".to_string());

        let result = workflow.add_connection_checked(node, node, &port_a, &port_b);

        assert_eq!(result, Err(ConnectionError::SelfConnection));
    }

    #[test]
    fn given_same_endpoints_different_ports_when_adding_checked_connection_then_both_connections_are_created(
    ) {
        let mut workflow = Workflow::new();
        let source = workflow.add_node("condition", 0.0, 0.0);
        let target = workflow.add_node("run", 100.0, 0.0);
        let main_port = PortName("main".to_string());
        let alt_port = PortName("alt".to_string());

        // First connection with "main" ports
        let first = workflow.add_connection_checked(source, target, &main_port, &main_port);
        assert_eq!(first, Ok(ConnectionResult::Created));

        // The condition node outputs FlowControl and run accepts Plain, so this
        // will fail with TypeMismatch — use same type nodes instead.
        let mut wf2 = Workflow::new();
        let s = wf2.add_node("run", 0.0, 0.0);
        let t = wf2.add_node("run", 100.0, 0.0);

        let c1 = wf2.add_connection_checked(s, t, &main_port, &main_port);
        assert_eq!(c1, Ok(ConnectionResult::Created));

        // Different port names are treated as a different connection,
        // but type check still applies to the same port types
        let c2 = wf2.add_connection_checked(s, t, &alt_port, &main_port);
        assert_eq!(c2, Ok(ConnectionResult::Created));
        assert_eq!(wf2.connections.len(), 2);
    }

    #[test]
    fn given_indirect_cycle_when_adding_checked_connection_then_cycle_error_is_returned() {
        // A -> B -> C, then try C -> A
        let mut workflow = Workflow::new();
        let a = workflow.add_node("http-handler", 0.0, 0.0);
        let b = workflow.add_node("run", 100.0, 0.0);
        let c = workflow.add_node("run", 200.0, 0.0);
        let main = PortName("main".to_string());

        let _ = workflow.add_connection_checked(a, b, &main, &main);
        let _ = workflow.add_connection_checked(b, c, &main, &main);

        // Trying C -> A should detect the indirect cycle
        let result = workflow.add_connection_checked(c, a, &main, &main);
        assert_eq!(result, Err(ConnectionError::WouldCreateCycle));
    }

    // ---------------------------------------------------------------------------
    // path_exists
    // ---------------------------------------------------------------------------

    #[test]
    fn given_disconnected_nodes_when_checking_path_exists_then_false_is_returned() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("run", 0.0, 0.0);
        let b = workflow.add_node("run", 100.0, 0.0);
        // No connections

        assert!(!Workflow::path_exists(&workflow.connections, a, b));
    }

    #[test]
    fn given_direct_edge_when_checking_path_exists_then_true_is_returned() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("http-handler", 0.0, 0.0);
        let b = workflow.add_node("run", 100.0, 0.0);
        let main = PortName("main".to_string());

        let _ = workflow.add_connection_checked(a, b, &main, &main);

        assert!(Workflow::path_exists(&workflow.connections, a, b));
    }

    #[test]
    fn given_transitive_path_when_checking_path_exists_then_true_is_returned() {
        let mut workflow = Workflow::new();
        let a = workflow.add_node("http-handler", 0.0, 0.0);
        let b = workflow.add_node("run", 100.0, 0.0);
        let c = workflow.add_node("run", 200.0, 0.0);
        let main = PortName("main".to_string());

        let _ = workflow.add_connection_checked(a, b, &main, &main);
        let _ = workflow.add_connection_checked(b, c, &main, &main);

        assert!(Workflow::path_exists(&workflow.connections, a, c));
    }

    #[test]
    fn given_same_node_no_self_loop_when_checking_path_exists_then_false_is_returned() {
        let connections: Vec<Connection> = Vec::new();
        let node = NodeId(Uuid::new_v4());
        assert!(!Workflow::path_exists(&connections, node, node));
    }

    #[test]
    fn given_self_loop_connection_when_checking_path_exists_from_node_to_itself_then_true_is_returned()
    {
        let node = NodeId(Uuid::new_v4());
        let connections = vec![Connection {
            id: Uuid::new_v4(),
            source: node,
            target: node,
            source_port: PortName("main".to_string()),
            target_port: PortName("main".to_string()),
        }];

        assert!(Workflow::path_exists(&connections, node, node));
    }

    #[test]
    fn given_empty_connections_when_checking_path_exists_then_false_is_returned() {
        let connections: Vec<Connection> = Vec::new();
        let a = NodeId(Uuid::new_v4());
        let b = NodeId(Uuid::new_v4());

        assert!(!Workflow::path_exists(&connections, a, b));
    }

    // ---------------------------------------------------------------------------
    // add_connection (bool-returning wrapper)
    // ---------------------------------------------------------------------------

    #[test]
    fn given_valid_connection_when_adding_unchecked_then_true_is_returned() {
        let mut workflow = Workflow::new();
        let source = workflow.add_node("http-handler", 0.0, 0.0);
        let target = workflow.add_node("run", 100.0, 0.0);
        let main = PortName("main".to_string());

        let added = workflow.add_connection(source, target, &main, &main);

        assert!(added);
        assert_eq!(workflow.connections.len(), 1);
    }

    #[test]
    fn given_self_connection_when_adding_unchecked_then_false_is_returned() {
        let mut workflow = Workflow::new();
        let node = workflow.add_node("run", 0.0, 0.0);
        let main = PortName("main".to_string());

        assert!(!workflow.add_connection(node, node, &main, &main));
        assert!(workflow.connections.is_empty());
    }

    // ---------------------------------------------------------------------------
    // find_source_and_target_nodes — single-pass optimization
    // ---------------------------------------------------------------------------

    #[test]
    fn given_single_node_list_when_finding_source_and_target_then_both_resolve_to_same_node() {
        let mut workflow = Workflow::new();
        let node_id = workflow.add_node("run", 0.0, 0.0);

        let result = Workflow::find_source_and_target_nodes(
            &workflow.nodes,
            node_id,
            node_id,
        );

        assert!(result.is_ok());
        let (src, tgt) = result.expect("both refer to same node");
        assert_eq!(src.id, node_id);
        assert_eq!(tgt.id, node_id);
    }

    #[test]
    fn given_empty_node_list_when_finding_source_and_target_then_missing_source_error_is_returned() {
        let nodes: Vec<super::super::Node> = Vec::new();
        let source = NodeId(Uuid::new_v4());
        let target = NodeId(Uuid::new_v4());

        let result = Workflow::find_source_and_target_nodes(&nodes, source, target);

        assert_eq!(result, Err(ConnectionError::MissingSourceNode(source)));
    }
}
