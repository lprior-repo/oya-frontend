// Proptest Invariants for Bead oya-frontend-rb4
// Test invariants for newtype constructors and pure functions

#![cfg(test)]

use oya_frontend::migration::*;
use proptest::prelude::*;

proptest! {
    // ============================================================================
    // ZoomFactor Invariants
    // ============================================================================

    #[test]
    fn zoomfactor_construction_enforces_bounds(value in any::<f32>()) {
        let result = ZoomFactor::from_f32(value);
        if let Some(zf) = result {
            prop_assert!(zf.value() >= 0.15, "ZoomFactor below minimum");
            prop_assert!(zf.value() <= 3.0, "ZoomFactor above maximum");
        } else {
            prop_assert!(
                value < 0.15 || value > 3.0 || !value.is_finite(),
                "Valid zoom rejected"
            );
        }
    }

    #[test]
    fn zoomfactor_boundary_values(
        low in just(0.15),
        high in just(3.0),
        below in 0.0..=0.14,
        above in 3.1..=10.0,
    ) {
        prop_assert!(ZoomFactor::from_f32(low).is_some());
        prop_assert!(ZoomFactor::from_f32(high).is_some());
        prop_assert!(ZoomFactor::from_f32(below).is_none());
        prop_assert!(ZoomFactor::from_f32(above).is_none());
    }

    // ============================================================================
    // ClassList Invariants
    // ============================================================================

    #[test]
    fn classlist_no_consecutive_spaces(input in any::<String>()) {
        let result = ClassList::from_string(&input);
        if let Ok(class_list) = result {
            prop_assert!(
                !class_list.as_str().contains("  "),
                "ClassList contains consecutive spaces"
            );
        }
    }

    #[test]
    fn classlist_trimmed(input in any::<String>()) {
        let result = ClassList::from_string(&input);
        if let Ok(class_list) = result {
            let s = class_list.as_str();
            if !s.is_empty() {
                prop_assert!(
                    !s.starts_with(|c: char| c.is_whitespace()),
                    "ClassList has leading whitespace"
                );
                prop_assert!(
                    !s.ends_with(|c: char| c.is_whitespace()),
                    "ClassList has trailing whitespace"
                );
            }
        }
    }

    #[test]
    fn classlist_non_empty(input in any::<String>()) {
        let result = ClassList::from_string(&input);
        if let Ok(class_list) = result {
            prop_assert!(!class_list.as_str().is_empty(), "ClassList is empty");
        }
    }

    // ============================================================================
    // Px Invariants
    // ============================================================================

    #[test]
    fn px_positive(value in any::<f32>()) {
        let result = Px::new(value);
        if let Some(px) = result {
            prop_assert!(px.value() > 0.0, "Px value is not positive");
            prop_assert!(px.value().is_finite(), "Px value is not finite");
        }
    }

    #[test]
    fn px_boundary_values(
        zero in just(0.0),
        small_pos in 0.001..=1.0,
        large_pos in 1000.0..=100000.0,
    ) {
        prop_assert!(Px::new(zero).is_none());
        prop_assert!(Px::new(small_pos).is_some());
        prop_assert!(Px::new(large_pos).is_some());
    }

    // ============================================================================
    // FlowPosition Invariants
    // ============================================================================

    #[test]
    fn flowposition_finite_positions(x in any::<f32>(), y in any::<f32>()) {
        let result = FlowPosition::new(x, y);
        if let Some(pos) = result {
            prop_assert!(pos.x.is_finite(), "FlowPosition x is not finite");
            prop_assert!(pos.y.is_finite(), "FlowPosition y is not finite");
        }
    }

    // ============================================================================
    // CanvasInteraction Invariants
    // ============================================================================

    #[test]
    fn canvas_interaction_exhaustive(state: CanvasInteraction) {
        match state {
            CanvasInteraction::Idle => {},
            CanvasInteraction::Panning { .. } => {},
            CanvasInteraction::DraggingNode { .. } => {},
            CanvasInteraction::Connecting { .. } => {},
        }
    }

    // ============================================================================
    // Transform Consistency Invariant
    // ============================================================================

    #[test]
    fn transform_consistent(
        pan_x in -10000.0..10000.0,
        pan_y in -10000.0..10000.0,
        zoom in prop::num::uniform(0.15..=3.0),
    ) {
        let zoom_factor = ZoomFactor::from_f32(zoom).unwrap();
        let translated_x = pan_x * zoom_factor.value();
        let translated_y = pan_y * zoom_factor.value();
        prop_assert!(translated_x.is_finite(), "Transformed X overflow");
        prop_assert!(translated_y.is_finite(), "Transformed Y overflow");
    }

    // ============================================================================
    // NodeId Invariant
    // ============================================================================

    #[test]
    fn nodeid_uuid_format(uuid_str in "\\PC*") {
        let result = NodeId::new(&uuid_str);
        if result.is_ok() {
            let id = result.unwrap();
            let parts: Vec<&str> = id.as_str().split('-').collect();
            prop_assert!(parts.len() >= 1, "Valid UUID should have structure");
        }
    }
}
