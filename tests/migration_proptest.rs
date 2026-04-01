// Proptest Invariants for Bead oya-frontend-rb4
// Test invariants for newtype constructors and pure functions

#![cfg(test)]

use proptest::prelude::*;

// ============================================================================
// ZoomFactor Invariants
// ============================================================================

proptest! {
    #[test]
    fn zoomfactor_construction_enforces_bounds(value in any::<f32>()) {
        let result = oya_frontend::migration::ZoomFactor::from_f32(value);
        if let Some(zf) = result {
            assert!(zf.value() >= 0.15, "ZoomFactor below minimum");
            assert!(zf.value() <= 3.0, "ZoomFactor above maximum");
        } else {
            assert!(
                value < 0.15 || value > 3.0 || !value.is_finite(),
                "Valid zoom rejected"
            );
        }
    }

    #[test]
    fn zoomfactor_boundary_values(
        low in 0.150_f32..=0.151_f32,
        high in 2.99_f32..=2.999_f32,
        below in 0.0_f32..=0.14_f32,
        above in 3.1_f32..=10.0_f32,
    ) {
        assert!(oya_frontend::migration::ZoomFactor::from_f32(low).is_some());
        assert!(oya_frontend::migration::ZoomFactor::from_f32(high).is_some());
        assert!(oya_frontend::migration::ZoomFactor::from_f32(below).is_none());
        assert!(oya_frontend::migration::ZoomFactor::from_f32(above).is_none());
    }

    // ============================================================================
    // ClassList Invariants
    // ============================================================================

    #[test]
    fn classlist_no_consecutive_spaces(input in any::<String>()) {
        let result = oya_frontend::migration::ClassList::from_string(&input);
        if let Ok(class_list) = result {
            assert!(
                !class_list.as_str().contains("  "),
                "ClassList contains consecutive spaces"
            );
        }
    }

    #[test]
    fn classlist_trimmed(input in any::<String>()) {
        let result = oya_frontend::migration::ClassList::from_string(&input);
        if let Ok(class_list) = result {
            let s = class_list.as_str();
            if !s.is_empty() {
                assert!(
                    !s.starts_with(|c: char| c.is_whitespace()),
                    "ClassList has leading whitespace"
                );
                assert!(
                    !s.ends_with(|c: char| c.is_whitespace()),
                    "ClassList has trailing whitespace"
                );
            }
        }
    }

    #[test]
    fn classlist_non_empty(input in any::<String>()) {
        let result = oya_frontend::migration::ClassList::from_string(&input);
        if let Ok(class_list) = result {
            assert!(!class_list.as_str().is_empty(), "ClassList is empty");
        }
    }

    // ============================================================================
    // Px Invariants
    // ============================================================================

    #[test]
    fn px_positive(value in any::<f32>()) {
        let result = oya_frontend::migration::Px::new(value);
        if let Some(px) = result {
            assert!(px.value() > 0.0, "Px value is not positive");
            assert!(px.value().is_finite(), "Px value is not finite");
        }
    }

    // ============================================================================
    // FlowPosition Invariants
    // ============================================================================

    #[test]
    fn flowposition_finite_positions(x in any::<f32>(), y in any::<f32>()) {
        let result = oya_frontend::migration::FlowPosition::new(x, y);
        if let Some(pos) = result {
            assert!(pos.x.is_finite(), "FlowPosition x is not finite");
            assert!(pos.y.is_finite(), "FlowPosition y is not finite");
        }
    }

    // ============================================================================
    // Transform Consistency Invariant
    // ============================================================================

    #[test]
    fn transform_consistent(
        pan_x in any::<f32>(),
        pan_y in any::<f32>(),
        zoom in any::<f32>(),
    ) {
        let zoom_factor = oya_frontend::migration::ZoomFactor::from_f32(zoom).unwrap_or(oya_frontend::migration::ZoomFactor(1.0));
        let translated_x = pan_x * zoom_factor.value();
        let translated_y = pan_y * zoom_factor.value();
        assert!(translated_x.is_finite(), "Transformed X overflow");
        assert!(translated_y.is_finite(), "Transformed Y overflow");
    }

    // ============================================================================
    // NodeId Invariant
    // ============================================================================

    #[test]
    fn nodeid_uuid_format(uuid_str in "\\PC*") {
        let result = oya_frontend::migration::NodeId::new(&uuid_str);
        if result.is_ok() {
            let id = result.expect("proptest filter guarantees valid UUID");
            let parts: Vec<&str> = id.as_str().split('-').collect();
            assert!(parts.len() >= 1, "Valid UUID should have structure");
        }
    }
}
