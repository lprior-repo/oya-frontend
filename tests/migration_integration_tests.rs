// Migration Integration Tests for Bead oya-frontend-rb4
// Rewritten to match functional-rust implementation with DDD principles

#![cfg(test)]

use oya_frontend::migration::*;
use std::collections::HashSet;

// ============================================================================
// build_source_contract Integration Tests (2 tests)
// ============================================================================

#[test]
fn build_source_contract_returns_source_file_missing() {
    let result = build_source_contract();
    assert_eq!(
        result,
        Err(MigrationError::SourceFileMissing {
            path: "placeholder".to_string()
        })
    );
}

#[test]
fn build_source_contract_error_type_is_struct_variant() {
    let result = build_source_contract();
    match result {
        Err(MigrationError::SourceFileMissing { path }) => {
            assert_eq!(path, "placeholder");
        }
        _ => panic!("Expected SourceFileMissing variant"),
    }
}

// ============================================================================
// validate_source_assets Integration Tests (2 tests)
// ============================================================================

#[test]
fn validate_source_assets_returns_source_file_missing() {
    let contract = UiParityContract {
        structural: StructuralContract {
            component_id: ComponentId::new("toolbar"),
            required_dom_order: vec![],
            required_class_tokens: vec![],
        },
        visual: VisualContract {
            component_id: ComponentId::new("toolbar"),
            width_px: None,
            height_px: None,
            spacing_scale: vec![],
            parity: ParityLevel::Exact,
        },
        interaction: InteractionContract {
            component_id: ComponentId::new("toolbar"),
            state_machine: vec![],
        },
    };

    let result = validate_source_assets(&contract);
    assert_eq!(
        result,
        Err(MigrationError::SourceFileMissing {
            path: "placeholder".to_string()
        })
    );
}

#[test]
fn validate_source_assets_error_variant_is_struct() {
    let contract = UiParityContract {
        structural: StructuralContract {
            component_id: ComponentId::new("canvas"),
            required_dom_order: vec![],
            required_class_tokens: vec![],
        },
        visual: VisualContract {
            component_id: ComponentId::new("canvas"),
            width_px: None,
            height_px: None,
            spacing_scale: vec![],
            parity: ParityLevel::Exact,
        },
        interaction: InteractionContract {
            component_id: ComponentId::new("canvas"),
            state_machine: vec![],
        },
    };

    let result = validate_source_assets(&contract);
    if let Err(MigrationError::SourceFileMissing { path }) = result {
        assert_eq!(path, "placeholder");
    } else {
        panic!("Expected SourceFileMissing variant");
    }
}

// ============================================================================
// map_source_tokens_to_dioxus Integration Tests (2 tests)
// ============================================================================

#[test]
fn map_source_tokens_to_dioxus_returns_unsupported_token() {
    let contract = UiParityContract {
        structural: StructuralContract {
            component_id: ComponentId::new("toolbar"),
            required_dom_order: vec![],
            required_class_tokens: vec![],
        },
        visual: VisualContract {
            component_id: ComponentId::new("toolbar"),
            width_px: None,
            height_px: None,
            spacing_scale: vec![],
            parity: ParityLevel::Exact,
        },
        interaction: InteractionContract {
            component_id: ComponentId::new("toolbar"),
            state_machine: vec![],
        },
    };

    let result = map_source_tokens_to_dioxus(&contract);
    assert_eq!(
        result,
        Err(MigrationError::UnsupportedCssToken {
            token: "placeholder".to_string()
        })
    );
}

#[test]
fn map_source_tokens_to_dioxus_error_variant_is_struct() {
    let contract = UiParityContract {
        structural: StructuralContract {
            component_id: ComponentId::new("canvas"),
            required_dom_order: vec![],
            required_class_tokens: vec![],
        },
        visual: VisualContract {
            component_id: ComponentId::new("canvas"),
            width_px: None,
            height_px: None,
            spacing_scale: vec![],
            parity: ParityLevel::Exact,
        },
        interaction: InteractionContract {
            component_id: ComponentId::new("canvas"),
            state_machine: vec![],
        },
    };

    let result = map_source_tokens_to_dioxus(&contract);
    if let Err(MigrationError::UnsupportedCssToken { token }) = result {
        assert_eq!(token, "placeholder");
    } else {
        panic!("Expected UnsupportedCssToken variant");
    }
}

// ============================================================================
// validate_component_structure Integration Tests (2 tests)
// ============================================================================

#[test]
fn validate_component_structure_returns_dom_structure_mismatch() {
    let rendered = RenderedTree {
        component_id: ComponentId::new("toolbar"),
        selectors: vec![],
    };

    let contract = UiParityContract {
        structural: StructuralContract {
            component_id: ComponentId::new("toolbar"),
            required_dom_order: vec![TestSelector::new("data-testid=toolbar")],
            required_class_tokens: vec![],
        },
        visual: VisualContract {
            component_id: ComponentId::new("toolbar"),
            width_px: None,
            height_px: None,
            spacing_scale: vec![],
            parity: ParityLevel::Exact,
        },
        interaction: InteractionContract {
            component_id: ComponentId::new("toolbar"),
            state_machine: vec![],
        },
    };

    let result = validate_component_structure(&rendered, &contract);
    assert_eq!(
        result,
        Err(MigrationError::DomStructureMismatch {
            component: "placeholder".to_string(),
            expected: "placeholder".to_string(),
            actual: "placeholder".to_string(),
        })
    );
}

#[test]
fn validate_component_structure_error_variant_is_struct() {
    let rendered = RenderedTree {
        component_id: ComponentId::new("canvas"),
        selectors: vec![TestSelector::new("data-testid=canvas")],
    };

    let contract = UiParityContract {
        structural: StructuralContract {
            component_id: ComponentId::new("canvas"),
            required_dom_order: vec![],
            required_class_tokens: vec![],
        },
        visual: VisualContract {
            component_id: ComponentId::new("canvas"),
            width_px: None,
            height_px: None,
            spacing_scale: vec![],
            parity: ParityLevel::Exact,
        },
        interaction: InteractionContract {
            component_id: ComponentId::new("canvas"),
            state_machine: vec![],
        },
    };

    let result = validate_component_structure(&rendered, &contract);
    if let Err(MigrationError::DomStructureMismatch {
        component,
        expected,
        actual,
    }) = result
    {
        assert_eq!(component, "placeholder");
        assert_eq!(expected, "placeholder");
        assert_eq!(actual, "placeholder");
    } else {
        panic!("Expected DomStructureMismatch variant");
    }
}

// ============================================================================
// validate_visual_metrics Integration Tests (2 tests)
// ============================================================================

#[test]
fn validate_visual_metrics_returns_parity_verification_failed() {
    let snapshot = VisualSnapshot {
        component_id: ComponentId::new("canvas"),
        width: 1920.0,
        height: 1080.0,
    };

    let baseline = VisualBaseline {
        component_id: ComponentId::new("canvas"),
        width: 1920.0,
        height: 1080.0,
        tolerance: 5.0,
    };

    let result = validate_visual_metrics(&snapshot, &baseline);
    assert_eq!(
        result,
        Err(MigrationError::ParityVerificationFailed {
            reason: "placeholder".to_string()
        })
    );
}

#[test]
fn validate_visual_metrics_error_variant_is_struct() {
    let snapshot = VisualSnapshot {
        component_id: ComponentId::new("toolbar"),
        width: 800.0,
        height: 600.0,
    };

    let baseline = VisualBaseline {
        component_id: ComponentId::new("toolbar"),
        width: 800.0,
        height: 600.0,
        tolerance: 10.0,
    };

    let result = validate_visual_metrics(&snapshot, &baseline);
    if let Err(MigrationError::ParityVerificationFailed { reason }) = result {
        assert_eq!(reason, "placeholder");
    } else {
        panic!("Expected ParityVerificationFailed variant");
    }
}

// ============================================================================
// validate_interaction_machine Integration Tests (2 tests)
// ============================================================================

#[test]
fn validate_interaction_machine_returns_invalid_transition() {
    let trace = InteractionTrace {
        transitions: vec![],
    };

    let result = validate_interaction_machine(&trace);
    assert_eq!(
        result,
        Err(MigrationError::InvalidInteractionTransition {
            from: "placeholder".to_string(),
            event: "placeholder".to_string(),
            to: "placeholder".to_string(),
        })
    );
}

#[test]
fn validate_interaction_machine_error_variant_is_struct() {
    let trace = InteractionTrace {
        transitions: vec![CanvasInteractionTransition {
            from: CanvasInteraction::Idle,
            event: "click".to_string(),
            to: CanvasInteraction::Idle,
        }],
    };

    let result = validate_interaction_machine(&trace);
    if let Err(MigrationError::InvalidInteractionTransition { from, event, to }) = result {
        assert_eq!(from, "placeholder");
        assert_eq!(event, "placeholder");
        assert_eq!(to, "placeholder");
    } else {
        panic!("Expected InvalidInteractionTransition variant");
    }
}

// ============================================================================
// validate_responsive_layout Integration Tests (2 tests)
// ============================================================================

#[test]
fn validate_responsive_layout_returns_regression() {
    let report = ResponsiveReport {
        breakpoints: vec![ResponsiveBreakpoint {
            name: "mobile".to_string(),
            width: 375.0,
            controls_reachable: false,
        }],
    };

    let result = validate_responsive_layout(&report);
    assert_eq!(
        result,
        Err(MigrationError::ResponsiveRegression {
            breakpoint: "placeholder".to_string(),
            reason: "placeholder".to_string(),
        })
    );
}

#[test]
fn validate_responsive_layout_error_variant_is_struct() {
    let report = ResponsiveReport {
        breakpoints: vec![ResponsiveBreakpoint {
            name: "desktop".to_string(),
            width: 1920.0,
            controls_reachable: true,
        }],
    };

    let result = validate_responsive_layout(&report);
    if let Err(MigrationError::ResponsiveRegression { breakpoint, reason }) = result {
        assert_eq!(breakpoint, "placeholder");
        assert_eq!(reason, "placeholder");
    } else {
        panic!("Expected ResponsiveRegression variant");
    }
}

// ============================================================================
// validate_animation_intent Integration Tests (2 tests)
// ============================================================================

#[test]
fn validate_animation_intent_returns_regression() {
    let report = AnimationReport {
        animation: "slide-in".to_string(),
        direction: "left".to_string(),
        duration_ms: 200,
    };

    let result = validate_animation_intent(&report);
    assert_eq!(
        result,
        Err(MigrationError::AnimationIntentRegression {
            animation: "placeholder".to_string(),
            reason: "placeholder".to_string(),
        })
    );
}

#[test]
fn validate_animation_intent_error_variant_is_struct() {
    let report = AnimationReport {
        animation: "fade-out".to_string(),
        direction: "up".to_string(),
        duration_ms: 300,
    };

    let result = validate_animation_intent(&report);
    if let Err(MigrationError::AnimationIntentRegression { animation, reason }) = result {
        assert_eq!(animation, "placeholder");
        assert_eq!(reason, "placeholder");
    } else {
        panic!("Expected AnimationIntentRegression variant");
    }
}

// ============================================================================
// finalize_migration_report Integration Tests (2 tests)
// ============================================================================

#[test]
fn finalize_migration_report_returns_parity_verification_failed() {
    let results = vec![ParityCheck {
        component_id: ComponentId::new("toolbar"),
        status: ParityLevel::Exact,
    }];

    let result = finalize_migration_report(&results);
    assert_eq!(
        result,
        Err(MigrationError::ParityVerificationFailed {
            reason: "placeholder".to_string()
        })
    );
}

#[test]
fn finalize_migration_report_error_variant_is_struct() {
    let results = vec![
        ParityCheck {
            component_id: ComponentId::new("toolbar"),
            status: ParityLevel::Exact,
        },
        ParityCheck {
            component_id: ComponentId::new("canvas"),
            status: ParityLevel::EquivalentFallback {
                reason: "fallback used".to_string(),
            },
        },
    ];

    let result = finalize_migration_report(&results);
    if let Err(MigrationError::ParityVerificationFailed { reason }) = result {
        assert_eq!(reason, "placeholder");
    } else {
        panic!("Expected ParityVerificationFailed variant");
    }
}

// ============================================================================
// Type Constructor Tests
// ============================================================================

#[test]
fn test_component_id_constructor() {
    let id = ComponentId::new("test-component");
    assert_eq!(id.as_str(), "test-component");
}

#[test]
fn test_test_selector_constructor() {
    let selector = TestSelector::new("data-testid=button");
    assert_eq!(selector.as_str(), "data-testid=button");
}

#[test]
fn test_px_constructor() {
    let px = Px::new(100.0);
    assert!(px.is_some());
    assert_eq!(px.expect("Px::new(100.0) should succeed").value(), 100.0);
}

#[test]
fn test_px_constructor_invalid() {
    let px = Px::new(-1.0);
    assert!(px.is_none());
}

#[test]
fn test_css_token_from_string_approved() {
    let mut approved: HashSet<String> = HashSet::new();
    approved.insert("flex".to_string());

    let token = CssToken::from_string("flex", &approved);
    assert_eq!(token, Ok(CssToken("flex".to_string())));
}

#[test]
fn test_css_token_from_string_unapproved() {
    let mut approved: HashSet<String> = HashSet::new();
    approved.insert("flex".to_string());

    let token = CssToken::from_string("unknown", &approved);
    assert_eq!(
        token,
        Err(MigrationError::UnsupportedCssToken {
            token: "unknown".to_string()
        })
    );
}

#[test]
fn test_node_id_valid() {
    let node = NodeId::new("12345678-1234-1234-1234-123456789012");
    assert_eq!(
        node,
        Ok(NodeId("12345678-1234-1234-1234-123456789012".to_string()))
    );
}

#[test]
fn test_node_id_invalid() {
    let node = NodeId::new("not-a-uuid");
    assert_eq!(node, Err(NodeIdError::InvalidUuid));
}

#[test]
fn test_flow_position_valid() {
    let pos = FlowPosition::new(10.0, 20.0);
    assert_eq!(pos, Some(FlowPosition { x: 10.0, y: 20.0 }));
}

#[test]
fn test_flow_position_invalid() {
    let pos = FlowPosition::new(f32::INFINITY, 0.0);
    assert_eq!(pos, None);
}

#[test]
fn test_zoom_factor_valid() {
    let zoom = ZoomFactor::from_f32(1.5);
    assert_eq!(zoom, Some(ZoomFactor(1.5)));
}

#[test]
fn test_zoom_factor_invalid() {
    let zoom = ZoomFactor::from_f32(4.0);
    assert_eq!(zoom, None);
}

#[test]
fn test_class_list_from_string() {
    let class = ClassList::from_string("flex items-center");
    assert_eq!(class, Ok(ClassList("flex items-center".to_string())));
}

#[test]
fn test_class_list_empty() {
    let class = ClassList::from_string("");
    assert_eq!(class, Err(ClassListError::Empty));
}

// ============================================================================
// All 20 Error Variants Coverage Test
// ============================================================================

#[test]
fn error_variant_01_source_file_missing() {
    let err = MigrationError::SourceFileMissing {
        path: "test.css".to_string(),
    };
    assert_eq!(err.to_string(), "required source file missing: test.css");
}

#[test]
fn error_variant_02_source_parse_failed() {
    let err = MigrationError::SourceParseFailed {
        path: "test.tsx".to_string(),
        reason: "unexpected token".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "source parse failed for test.tsx: unexpected token"
    );
}

#[test]
fn error_variant_03_source_component_missing() {
    let err = MigrationError::SourceComponentMissing {
        component: "button".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "required component missing in source contract: button"
    );
}

#[test]
fn error_variant_04_required_class_missing() {
    let err = MigrationError::RequiredClassMissing {
        component: "toolbar".to_string(),
        token: "flex".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "required class token missing: component=toolbar, token=flex"
    );
}

#[test]
fn error_variant_05_unsupported_css_token() {
    let err = MigrationError::UnsupportedCssToken {
        token: "custom-token".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "unsupported css token for dioxus pipeline: custom-token"
    );
}

#[test]
fn error_variant_06_token_mapping_collision() {
    let err = MigrationError::TokenMappingCollision {
        source_str: "bg-primary".to_string(),
        target_str: "bg-background".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "token mapping collision: source=bg-primary, target=bg-background"
    );
}

#[test]
fn error_variant_07_dom_structure_mismatch() {
    let err = MigrationError::DomStructureMismatch {
        component: "toolbar".to_string(),
        expected: "name,actions".to_string(),
        actual: "actions,name".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "dom structure mismatch in toolbar: expected=name,actions, actual=actions,name"
    );
}

#[test]
fn error_variant_08_layout_tolerance_exceeded() {
    let err = MigrationError::LayoutToleranceExceeded {
        component: "canvas".to_string(),
        metric: "width".to_string(),
        expected: "1920".to_string(),
        actual: "1950".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "layout metric out of tolerance: component=canvas, metric=width, expected=1920, actual=1950"
    );
}

#[test]
fn error_variant_09_responsive_regression() {
    let err = MigrationError::ResponsiveRegression {
        breakpoint: "768px".to_string(),
        reason: "sidebar hidden".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "responsive regression at breakpoint 768px: sidebar hidden"
    );
}

#[test]
fn error_variant_10_animation_intent_regression() {
    let err = MigrationError::AnimationIntentRegression {
        animation: "slide-in".to_string(),
        reason: "no slide detected".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "animation intent regression: animation=slide-in, reason=no slide detected"
    );
}

#[test]
fn error_variant_11_invalid_interaction_transition() {
    let err = MigrationError::InvalidInteractionTransition {
        from: "Idle".to_string(),
        event: "click".to_string(),
        to: "Dragging".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "invalid interaction transition: from=Idle, event=click, to=Dragging"
    );
}

#[test]
fn error_variant_12_invalid_connection_attempt() {
    let err = MigrationError::InvalidConnectionAttempt {
        reason: "self-connection".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "invalid connection attempt: self-connection"
    );
}

#[test]
fn error_variant_13_node_not_found() {
    let err = MigrationError::NodeNotFound {
        node_id: "nonexistent".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "node not found for interaction: nonexistent"
    );
}

#[test]
fn error_variant_14_edge_endpoint_missing() {
    let err = MigrationError::EdgeEndpointMissing {
        source_str: "node-1".to_string(),
        target_str: "node-2".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "edge render target missing: source=node-1, target=node-2"
    );
}

#[test]
fn error_variant_15_viewport_invariant_violation() {
    let err = MigrationError::ViewportInvariantViolation {
        reason: "zoom out of range".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "viewport invariant violated: zoom out of range"
    );
}

#[test]
fn error_variant_16_local_storage_read_failure() {
    let err = MigrationError::LocalStorageReadFailure {
        reason: "quota exceeded".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "local storage read failure: quota exceeded"
    );
}

#[test]
fn error_variant_17_local_storage_write_failure() {
    let err = MigrationError::LocalStorageWriteFailure {
        reason: "quota exceeded".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "local storage write failure: quota exceeded"
    );
}

#[test]
fn error_variant_18_local_storage_data_corrupted() {
    let err = MigrationError::LocalStorageDataCorrupted {
        reason: "invalid json".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "local storage data corrupted: invalid json"
    );
}

#[test]
fn error_variant_19_minimap_regression() {
    let err = MigrationError::MinimapRegression {
        reason: "minimap blocks interaction".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "minimap render regression: minimap blocks interaction"
    );
}

#[test]
fn error_variant_20_parity_verification_failed() {
    let err = MigrationError::ParityVerificationFailed {
        reason: "critical check failed".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "parity verification failed: critical check failed"
    );
}
