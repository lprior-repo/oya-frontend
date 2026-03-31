// Migration Unit Tests for Bead oya-frontend-rb4
// Test all newtype constructors, sum types, and pure functions

#![cfg(test)]

use oya_frontend::migration::*;
use std::collections::HashSet;

// ============================================================================
// ZoomFactor Tests (8 tests)
// ============================================================================

#[test]
fn zoomfactor_from_f32_valid_minimum() {
    let result = ZoomFactor::from_f32(0.15);
    assert_eq!(result, Some(ZoomFactor(0.15)));
}

#[test]
fn zoomfactor_from_f32_valid_maximum() {
    let result = ZoomFactor::from_f32(3.0);
    assert_eq!(result, Some(ZoomFactor(3.0)));
}

#[test]
fn zoomfactor_from_f32_below_minimum() {
    let result = ZoomFactor::from_f32(0.14);
    assert_eq!(result, None);
}

#[test]
fn zoomfactor_from_f32_above_maximum() {
    let result = ZoomFactor::from_f32(3.1);
    assert_eq!(result, None);
}

#[test]
fn zoomfactor_from_f32_nan() {
    let result = ZoomFactor::from_f32(f32::NAN);
    assert_eq!(result, None);
}

#[test]
fn zoomfactor_from_f32_infinity() {
    let result = ZoomFactor::from_f32(f32::INFINITY);
    assert_eq!(result, None);
}

#[test]
fn zoomfactor_from_f32_neg_infinity() {
    let result = ZoomFactor::from_f32(f32::NEG_INFINITY);
    assert_eq!(result, None);
}

#[test]
fn zoomfactor_from_f32_normal() {
    let result = ZoomFactor::from_f32(1.0);
    assert_eq!(result, Some(ZoomFactor(1.0)));
}

// ============================================================================
// ClassList Tests (7 tests)
// ============================================================================

#[test]
fn classlist_from_string_single_token() {
    let result = ClassList::from_string("hello");
    assert_eq!(result, Ok(ClassList("hello".to_string())));
}

#[test]
fn classlist_from_string_multiple_tokens() {
    let result = ClassList::from_string("flex items-center justify-between");
    assert_eq!(
        result,
        Ok(ClassList("flex items-center justify-between".to_string()))
    );
}

#[test]
fn classlist_from_string_normalize_consecutive_spaces() {
    let result = ClassList::from_string("flex   items-center   justify-between");
    assert_eq!(
        result,
        Ok(ClassList("flex items-center justify-between".to_string()))
    );
}

#[test]
fn classlist_from_string_trim_leading_trailing() {
    let result = ClassList::from_string("  hello world  ");
    assert_eq!(result, Ok(ClassList("hello world".to_string())));
}

#[test]
fn classlist_from_string_empty() {
    let result = ClassList::from_string("");
    assert_eq!(result, Err(ClassListError::Empty));
}

#[test]
fn classlist_from_string_whitespace_only() {
    let result = ClassList::from_string("   ");
    assert_eq!(result, Err(ClassListError::Empty));
}

#[test]
fn classlist_from_string_complex() {
    let result = ClassList::from_string("flex items-center justify-between");
    assert_eq!(
        result,
        Ok(ClassList("flex items-center justify-between".to_string()))
    );
}

// ============================================================================
// CssToken Tests (2 tests)
// ============================================================================

#[test]
fn css_token_from_string_valid() {
    let approved_set: HashSet<String> = HashSet::from_iter(vec![
        "flex".to_string(),
        "items-center".to_string(),
        "h-screen".to_string(),
    ]);
    let result = CssToken::from_string("flex", &approved_set);
    assert_eq!(result, Ok(CssToken("flex".to_string())));
}

#[test]
fn css_token_from_string_invalid() {
    let approved_set: HashSet<String> =
        HashSet::from_iter(vec!["flex".to_string(), "items-center".to_string()]);
    let result = CssToken::from_string("nonexistent", &approved_set);
    assert_eq!(
        result,
        Err(MigrationError::UnsupportedCssToken {
            token: "nonexistent".to_string()
        })
    );
}

// ============================================================================
// Px Tests (6 tests)
// ============================================================================

#[test]
fn px_new_valid() {
    let result = Px::new(100.0);
    assert_eq!(result, Some(Px(100.0)));
}

#[test]
fn px_new_zero() {
    let result = Px::new(0.0);
    assert_eq!(result, None);
}

#[test]
fn px_new_negative() {
    let result = Px::new(-1.0);
    assert_eq!(result, None);
}

#[test]
fn px_new_nan() {
    let result = Px::new(f32::NAN);
    assert_eq!(result, None);
}

#[test]
fn px_new_infinity() {
    let result = Px::new(f32::INFINITY);
    assert_eq!(result, None);
}

#[test]
fn px_new_small_positive() {
    let result = Px::new(0.001);
    assert_eq!(result, Some(Px(0.001)));
}

// ============================================================================
// ComponentId Tests (2 tests - compile-time static)
// ============================================================================

#[test]
fn component_id_new_toolbar() {
    let result = ComponentId::new("toolbar");
    assert_eq!(result.as_str(), "toolbar");
}

#[test]
fn component_id_new_node_card() {
    let result = ComponentId::new("node-card");
    assert_eq!(result.as_str(), "node-card");
}

// ============================================================================
// TestSelector Tests (2 tests - compile-time static)
// ============================================================================

#[test]
fn test_selector_new_toolbar() {
    let result = TestSelector::new("data-testid=toolbar");
    assert_eq!(result.as_str(), "data-testid=toolbar");
}

#[test]
fn test_selector_new_selector() {
    let result = TestSelector::new(".flex.h-screen");
    assert_eq!(result.as_str(), ".flex.h-screen");
}

// ============================================================================
// NodeId Tests (2 tests)
// ============================================================================

#[test]
fn node_id_new_valid_uuid() {
    let result = NodeId::new("550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(
        result,
        Ok(NodeId("550e8400-e29b-41d4-a716-446655440000".to_string()))
    );
}

#[test]
fn node_id_new_invalid_uuid() {
    let result = NodeId::new("invalid-uuid");
    assert_eq!(result, Err(NodeIdError::InvalidUuid));
}

// ============================================================================
// FlowPosition Tests (4 tests)
// ============================================================================

#[test]
fn flow_position_new_origin() {
    let result = FlowPosition::new(0.0, 0.0);
    assert_eq!(result, Some(FlowPosition { x: 0.0, y: 0.0 }));
}

#[test]
fn flow_position_new_negative_x() {
    let result = FlowPosition::new(-100.0, 50.0);
    assert_eq!(result, Some(FlowPosition { x: -100.0, y: 50.0 }));
}

#[test]
fn flow_position_new_nan_x() {
    let result = FlowPosition::new(f32::NAN, 0.0);
    assert_eq!(result, None);
}

#[test]
fn flow_position_new_infinity_y() {
    let result = FlowPosition::new(0.0, f32::INFINITY);
    assert_eq!(result, None);
}

// ============================================================================
// HandleType Sum Type Tests (3 tests)
// ============================================================================

#[test]
fn handle_type_source_variant() {
    let handle = HandleType::Source;
    match handle {
        HandleType::Source => {}
        HandleType::Target => panic!("Expected Source variant"),
    }
}

#[test]
fn handle_type_target_variant() {
    let handle = HandleType::Target;
    match handle {
        HandleType::Source => panic!("Expected Target variant"),
        HandleType::Target => {}
    }
}

#[test]
fn handle_type_exhaustive_match_compiles() {
    let handle = HandleType::Source;
    let result = match handle {
        HandleType::Source => "source",
        HandleType::Target => "target",
    };
    assert_eq!(result, "source");
}

// ============================================================================
// SelectionState Sum Type Tests (3 tests)
// ============================================================================

#[test]
fn selection_state_none_variant() {
    let state = SelectionState::None;
    match state {
        SelectionState::None => {}
        SelectionState::NodeSelected { .. } => panic!("Expected None variant"),
    }
}

#[test]
fn selection_state_node_selected_variant() {
    let state = SelectionState::NodeSelected {
        node_id: NodeId::new("550e8400-e29b-41d4-a716-446655440000").unwrap(),
    };
    match state {
        SelectionState::None => panic!("Expected NodeSelected variant"),
        SelectionState::NodeSelected { node_id } => {
            assert_eq!(node_id.as_str(), "550e8400-e29b-41d4-a716-446655440000");
        }
    }
}

#[test]
fn selection_state_exhaustive_match_compiles() {
    let state = SelectionState::None;
    let result = match state {
        SelectionState::None => "none",
        SelectionState::NodeSelected { .. } => "selected",
    };
    assert_eq!(result, "none");
}

// ============================================================================
// CanvasInteraction Sum Type Tests (5 tests)
// ============================================================================

#[test]
fn canvas_interaction_idle_variant() {
    let state = CanvasInteraction::Idle;
    match state {
        CanvasInteraction::Idle => {}
        _ => panic!("Expected Idle variant"),
    }
}

#[test]
fn canvas_interaction_panning_variant() {
    let state = CanvasInteraction::Panning {
        start: FlowPosition::new(0.0, 0.0).unwrap(),
        origin: FlowPosition::new(10.0, 20.0).unwrap(),
    };
    match state {
        CanvasInteraction::Panning { start, origin } => {
            assert_eq!(start.x, 0.0);
            assert_eq!(start.y, 0.0);
            assert_eq!(origin.x, 10.0);
            assert_eq!(origin.y, 20.0);
        }
        _ => panic!("Expected Panning variant"),
    }
}

#[test]
fn canvas_interaction_dragging_node_variant() {
    let node_id = NodeId::new("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let state = CanvasInteraction::DraggingNode {
        node_id: node_id.clone(),
        start: FlowPosition::new(0.0, 0.0).unwrap(),
        origin: FlowPosition::new(10.0, 20.0).unwrap(),
    };
    match state {
        CanvasInteraction::DraggingNode { node_id: nid, .. } => {
            assert_eq!(nid.as_str(), "550e8400-e29b-41d4-a716-446655440000");
        }
        _ => panic!("Expected DraggingNode variant"),
    }
}

#[test]
fn canvas_interaction_connecting_variant() {
    let node_id = NodeId::new("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let state = CanvasInteraction::Connecting {
        from: node_id.clone(),
        handle: HandleType::Source,
        cursor: FlowPosition::new(100.0, 200.0).unwrap(),
    };
    match state {
        CanvasInteraction::Connecting {
            from,
            handle,
            cursor,
        } => {
            assert_eq!(from.as_str(), "550e8400-e29b-41d4-a716-446655440000");
            assert_eq!(handle, HandleType::Source);
            assert_eq!(cursor.x, 100.0);
            assert_eq!(cursor.y, 200.0);
        }
        _ => panic!("Expected Connecting variant"),
    }
}

#[test]
fn canvas_interaction_exhaustive_match_compiles() {
    let state = CanvasInteraction::Idle;
    let result = match state {
        CanvasInteraction::Idle => "idle",
        CanvasInteraction::Panning { .. } => "panning",
        CanvasInteraction::DraggingNode { .. } => "dragging",
        CanvasInteraction::Connecting { .. } => "connecting",
    };
    assert_eq!(result, "idle");
}

// ============================================================================
// ParityLevel Sum Type Tests (3 tests)
// ============================================================================

#[test]
fn parity_level_exact_variant() {
    let level = ParityLevel::Exact;
    match level {
        ParityLevel::Exact => {}
        ParityLevel::EquivalentFallback { .. } => panic!("Expected Exact variant"),
    }
}

#[test]
fn parity_level_equivalent_fallback_variant() {
    let level = ParityLevel::EquivalentFallback {
        reason: "Using fallback".to_string(),
    };
    match level {
        ParityLevel::Exact => panic!("Expected EquivalentFallback variant"),
        ParityLevel::EquivalentFallback { reason } => {
            assert_eq!(reason, "Using fallback");
        }
    }
}

#[test]
fn parity_level_exhaustive_match_compiles() {
    let level = ParityLevel::Exact;
    let result = match level {
        ParityLevel::Exact => "exact",
        ParityLevel::EquivalentFallback { .. } => "fallback",
    };
    assert_eq!(result, "exact");
}
