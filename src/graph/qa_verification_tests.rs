// QA verification test for bead 1af
#[cfg(test)]
mod qa_tests {
    use oya_frontend::graph::*;

    #[test]
    fn service_kind_available_clients_impl_handler_exact_service() {
        // Test Handler
        let clients = ServiceKind::Handler.available_clients();
        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0], ClientType::Service);
    }

    #[test]
    fn service_kind_available_clients_impl_actor_exact_service_object() {
        // Test Actor
        let clients = ServiceKind::Actor.available_clients();
        assert_eq!(clients.len(), 2);
        assert_eq!(clients[0], ClientType::Service);
        assert_eq!(clients[1], ClientType::Object);
    }

    #[test]
    fn service_kind_available_clients_impl_workflow_exact_service_object_workflow() {
        // Test Workflow
        let clients = ServiceKind::Workflow.available_clients();
        assert_eq!(clients.len(), 3);
        assert_eq!(clients[0], ClientType::Service);
        assert_eq!(clients[1], ClientType::Object);
        assert_eq!(clients[2], ClientType::Workflow);
    }

    #[test]
    fn context_type_available_traits_impl_synchronous_exact_6_traits() {
        // Test Synchronous (6 traits)
        let traits = ContextType::Synchronous.available_traits();
        assert_eq!(traits.len(), 6);
    }

    #[test]
    fn context_type_available_traits_impl_asynchronous_exact_7_traits() {
        // Test Asynchronous (7 traits)
        let traits = ContextType::Asynchronous.available_traits();
        assert_eq!(traits.len(), 7);
    }

    #[test]
    fn http_handler_has_handler_service_kind() {
        let node = WorkflowNode::HttpHandler(HttpHandlerConfig::default());
        assert_eq!(node.service_kind(), ServiceKind::Handler);
    }

    #[test]
    fn object_call_has_actor_service_kind() {
        let node = WorkflowNode::ObjectCall(ObjectCallConfig::default());
        assert_eq!(node.service_kind(), ServiceKind::Actor);
    }

    #[test]
    fn workflow_call_has_workflow_service_kind() {
        let node = WorkflowNode::WorkflowCall(WorkflowCallConfig::default());
        assert_eq!(node.service_kind(), ServiceKind::Workflow);
    }

    #[test]
    fn check_connection_impl_happy_path_http_handler_to_http_call() {
        let source = WorkflowNode::HttpHandler(HttpHandlerConfig::default());
        let target = WorkflowNode::HttpCall(HttpCallConfig::default());
        assert!(check_connection(&source, &target).is_ok());
    }

    #[test]
    fn get_node_by_id_find_existing_node() {
        let node1 = Node::new(
            NodeId::new("test-001"),
            WorkflowNode::HttpHandler(HttpHandlerConfig::default()),
        );
        let node2 = Node::new(
            NodeId::new("test-002"),
            WorkflowNode::Run(RunConfig::default()),
        );
        let nodes = vec![node1, node2];

        let id = NodeId::new("test-001");
        assert!(get_node_by_id(id, &nodes).is_ok());
    }

    #[test]
    fn get_node_by_id_rejects_nonexistent() {
        let node1 = Node::new(
            NodeId::new("test-001"),
            WorkflowNode::HttpHandler(HttpHandlerConfig::default()),
        );
        let nodes = vec![node1];

        let id = NodeId::new("nonexistent");
        assert!(matches!(
            get_node_by_id(id, &nodes),
            Err(ConnectionError::NodeNotFound { .. })
        ));
    }

    #[test]
    fn service_kind_from_str_rejects_invalid_string() {
        let result: Result<ServiceKind, _> = "invalid".parse();
        assert!(result.is_err());
    }

    #[test]
    fn context_type_from_str_rejects_invalid_string() {
        let result: Result<ContextType, _> = "invalid".parse();
        assert!(result.is_err());
    }

    #[test]
    fn types_compatible_impl_any_universal_source_event() {
        assert!(types_compatible(PortType::Any, PortType::Event));
    }

    #[test]
    fn types_compatible_impl_json_universal_source_state() {
        assert!(types_compatible(PortType::Json, PortType::State));
    }

    #[test]
    fn types_compatible_impl_incompatible_event_state() {
        assert!(!types_compatible(PortType::Event, PortType::State));
    }
}
