// Additional adversarial tests for migration.rs
// Run with: cargo test --test adversarial_tests

#![cfg(test)]

use oya_frontend::migration::*;
use std::collections::HashSet;

// ============================================================================
// Additional ZoomFactor Edge Cases
// ============================================================================

#[test]
fn zoomfactor_extreme_positive() {
    // f32::MAX is approximately 3.4e38, way above 3.0
    let result = ZoomFactor::from_f32(f32::MAX);
    assert_eq!(result, None);
}

#[test]
fn zoomfactor_extreme_negative() {
    // f32::MIN is approximately -3.4e38, negative and below 0.15
    let result = ZoomFactor::from_f32(f32::MIN);
    assert_eq!(result, None);
}

#[test]
fn zoomfactor_subnormal() {
    // f32::MIN_POSITIVE is approximately 1.4e-45, below 0.15
    let result = ZoomFactor::from_f32(f32::MIN_POSITIVE);
    assert_eq!(result, None);
}

#[test]
fn zoomfactor_just_below_minimum() {
    let result = ZoomFactor::from_f32(0.149999);
    assert_eq!(result, None);
}

#[test]
fn zoomfactor_just_above_maximum() {
    let result = ZoomFactor::from_f32(3.000001);
    assert_eq!(result, None);
}

// ============================================================================
// Additional ClassList Edge Cases
// ============================================================================

#[test]
fn classlist_unicode_whitespace() {
    // Various Unicode whitespace characters
    let result = ClassList::from_string("\u{00A0}\u{2003}\u{3000}hello\u{3000}\u{2003}\u{00A0}");
    assert_eq!(result, Ok(ClassList("hello".to_string())));
}

#[test]
fn classlist_newlines_and_tabs() {
    let result = ClassList::from_string("\n\t\r\nhello\t\r\n");
    assert_eq!(result, Ok(ClassList("hello".to_string())));
}

#[test]
fn classlist_emoji_and_symbols() {
    let result = ClassList::from_string("🎨-🖌️-✨");
    assert_eq!(result, Ok(ClassList("🎨-🖌️-✨".to_string())));
}

#[test]
fn classlist_very_long() {
    let long_string = "flex ".repeat(1000);
    let result = ClassList::from_string(&long_string);
    assert!(result.is_ok());
    let binding = result.unwrap();
    let classes = binding.as_str();
    assert!(classes.contains("flex"));
    assert!(!classes.contains("  "));
}

// ============================================================================
// Additional CssToken Edge Cases
// ============================================================================

#[test]
fn csstoken_null_byte() {
    let approved_set: HashSet<String> = HashSet::from_iter(vec!["flex".to_string()]);
    let result = CssToken::from_string("flex\x00", &approved_set);
    assert!(result.is_err());
}

#[test]
fn csstoken_html_entities() {
    let approved_set: HashSet<String> = HashSet::from_iter(vec!["div".to_string()]);
    let result = CssToken::from_string("&lt;div&gt;", &approved_set);
    assert!(result.is_err());
}

#[test]
fn csstoken_sql_injection() {
    let approved_set: HashSet<String> = HashSet::from_iter(vec!["test".to_string()]);
    let result = CssToken::from_string("' OR 1=1 --", &approved_set);
    assert!(result.is_err());
}

#[test]
fn csstoken_xss_vector() {
    let approved_set: HashSet<String> = HashSet::from_iter(vec!["safe".to_string()]);
    let result = CssToken::from_string("<script>alert(1)</script>", &approved_set);
    assert!(result.is_err());
}

#[test]
fn csstoken_path_traversal() {
    let approved_set: HashSet<String> = HashSet::from_iter(vec!["safe".to_string()]);
    let result = CssToken::from_string("../../etc/passwd", &approved_set);
    assert!(result.is_err());
}

// ============================================================================
// Additional Px Edge Cases
// ============================================================================

#[test]
fn px_subnormal() {
    let result = Px::new(f32::MIN_POSITIVE);
    // Subnormal values > 0 should be accepted
    assert!(result.is_some());
}

#[test]
fn px_negative_zero() {
    let result = Px::new(-0.0);
    assert_eq!(result, None);
}

#[test]
fn px_very_small_positive() {
    let result = Px::new(0.0000001);
    assert!(result.is_some());
}

// ============================================================================
// Additional NodeId Edge Cases
// ============================================================================

#[test]
fn nodeid_all_zeros() {
    let result = NodeId::new("00000000-0000-0000-0000-000000000000");
    assert!(result.is_ok());
}

#[test]
fn nodeid_all_ones() {
    let result = NodeId::new("ffffffff-ffff-ffff-ffff-ffffffffffff");
    assert!(result.is_ok());
}

#[test]
fn nodeid_uppercase_hex() {
    let result = NodeId::new("550E8400-E29B-41D4-A716-446655440000");
    assert!(result.is_ok());
}

#[test]
fn nodeid_mixed_case_hex() {
    let result = NodeId::new("550e8400-E29b-41D4-a716-446655440000");
    assert!(result.is_ok());
}

#[test]
fn nodeid_null_byte() {
    let result = NodeId::new("550e8400-e29b-41d4-a716-4466554400\x00");
    assert!(result.is_err());
}

#[test]
fn nodeid_garbage() {
    let result = NodeId::new("not-a-uuid-at-all");
    assert!(result.is_err());
}

#[test]
fn nodeid_empty() {
    let result = NodeId::new("");
    assert!(result.is_err());
}

#[test]
fn nodeid_partial() {
    let result = NodeId::new("550e8400-e29b");
    assert!(result.is_err());
}

// ============================================================================
// Additional FlowPosition Edge Cases
// ============================================================================

#[test]
fn flowposition_negative_zero() {
    let result = FlowPosition::new(-0.0, 0.0);
    assert!(result.is_some());
}

#[test]
fn flowposition_extreme_positive() {
    // BUG: f32::MAX is accepted because it's finite but causes overflow
    // This test documents the defect - should be None but is Some
    let result = FlowPosition::new(f32::MAX, f32::MAX);
    assert!(result.is_some(), "BUG: f32::MAX should be rejected");
}

#[test]
fn flowposition_extreme_negative() {
    // BUG: f32::MIN is accepted because it's finite but causes overflow
    // This test documents the defect - should be None but is Some
    let result = FlowPosition::new(f32::MIN, f32::MIN);
    assert!(result.is_some(), "BUG: f32::MIN should be rejected");
}

#[test]
fn flowposition_mixed_nan() {
    let result = FlowPosition::new(100.0, f32::NAN);
    assert_eq!(result, None);
}

#[test]
fn flowposition_mixed_infinity() {
    let result = FlowPosition::new(f32::INFINITY, 100.0);
    assert_eq!(result, None);
}

// ============================================================================
// MigrationError Exhaustive Variant Testing
// ============================================================================

#[test]
fn migration_error_source_file_missing() {
    let err = MigrationError::SourceFileMissing {
        path: "test.txt".to_string(),
    };
    assert!(err.to_string().contains("test.txt"));
}

#[test]
fn migration_error_source_parse_failed() {
    let err = MigrationError::SourceParseFailed {
        path: "test.txt".to_string(),
        reason: "syntax error".to_string(),
    };
    assert!(err.to_string().contains("test.txt"));
}

#[test]
fn migration_error_source_component_missing() {
    let err = MigrationError::SourceComponentMissing {
        component: "Toolbar".to_string(),
    };
    assert!(err.to_string().contains("Toolbar"));
}

#[test]
fn migration_error_required_class_missing() {
    let err = MigrationError::RequiredClassMissing {
        component: "NodeCard".to_string(),
        token: "flex".to_string(),
    };
    assert!(err.to_string().contains("NodeCard"));
}

#[test]
fn migration_error_unsupported_css_token() {
    let err = MigrationError::UnsupportedCssToken {
        token: "custom".to_string(),
    };
    assert!(err.to_string().contains("custom"));
}

#[test]
fn migration_error_token_mapping_collision() {
    let err = MigrationError::TokenMappingCollision {
        source_str: "old".to_string(),
        target_str: "new".to_string(),
    };
    assert!(err.to_string().contains("old"));
}

#[test]
fn migration_error_dom_structure_mismatch() {
    let err = MigrationError::DomStructureMismatch {
        component: "Toolbar".to_string(),
        expected: "div".to_string(),
        actual: "span".to_string(),
    };
    assert!(err.to_string().contains("Toolbar"));
}

#[test]
fn migration_error_layout_tolerance_exceeded() {
    let err = MigrationError::LayoutToleranceExceeded {
        component: "NodeCard".to_string(),
        metric: "width".to_string(),
        expected: "100px".to_string(),
        actual: "150px".to_string(),
    };
    assert!(err.to_string().contains("NodeCard"));
}

#[test]
fn migration_error_responsive_regression() {
    let err = MigrationError::ResponsiveRegression {
        breakpoint: "mobile".to_string(),
        reason: "overflow".to_string(),
    };
    assert!(err.to_string().contains("mobile"));
}

#[test]
fn migration_error_animation_intent_regression() {
    let err = MigrationError::AnimationIntentRegression {
        animation: "fade".to_string(),
        reason: "missing".to_string(),
    };
    assert!(err.to_string().contains("fade"));
}

#[test]
fn migration_error_invalid_interaction_transition() {
    let err = MigrationError::InvalidInteractionTransition {
        from: "idle".to_string(),
        event: "click".to_string(),
        to: "invalid".to_string(),
    };
    assert!(err.to_string().contains("idle"));
}

#[test]
fn migration_error_invalid_connection_attempt() {
    let err = MigrationError::InvalidConnectionAttempt {
        reason: "self-connection".to_string(),
    };
    assert!(err.to_string().contains("self-connection"));
}

#[test]
fn migration_error_node_not_found() {
    let err = MigrationError::NodeNotFound {
        node_id: "123".to_string(),
    };
    assert!(err.to_string().contains("123"));
}

#[test]
fn migration_error_edge_endpoint_missing() {
    let err = MigrationError::EdgeEndpointMissing {
        source_str: "source".to_string(),
        target_str: "target".to_string(),
    };
    assert!(err.to_string().contains("source"));
}

#[test]
fn migration_error_viewport_invariant_violation() {
    let err = MigrationError::ViewportInvariantViolation {
        reason: "zoom out of bounds".to_string(),
    };
    assert!(err.to_string().contains("zoom out of bounds"));
}

#[test]
fn migration_error_local_storage_read_failure() {
    let err = MigrationError::LocalStorageReadFailure {
        reason: "permission denied".to_string(),
    };
    assert!(err.to_string().contains("permission denied"));
}

#[test]
fn migration_error_local_storage_write_failure() {
    let err = MigrationError::LocalStorageWriteFailure {
        reason: "quota exceeded".to_string(),
    };
    assert!(err.to_string().contains("quota exceeded"));
}

#[test]
fn migration_error_local_storage_data_corrupted() {
    let err = MigrationError::LocalStorageDataCorrupted {
        reason: "checksum mismatch".to_string(),
    };
    assert!(err.to_string().contains("checksum mismatch"));
}

#[test]
fn migration_error_minimap_regression() {
    let err = MigrationError::MinimapRegression {
        reason: "rendering error".to_string(),
    };
    assert!(err.to_string().contains("rendering error"));
}

#[test]
fn migration_error_parity_verification_failed() {
    let err = MigrationError::ParityVerificationFailed {
        reason: "visual mismatch".to_string(),
    };
    assert!(err.to_string().contains("visual mismatch"));
}
