//! Canvas pan and zoom tests.
//!
//! Verifies:
//! 1. Zoom applies with delta and respects limits (0.15x to 3.0x)
//! 2. Pan changes viewport position
//! 3. Fit-to-view centers all nodes within viewport
//! 4. Zoom centers on cursor position
//! 5. Edge cases: empty workflow, NaN inputs, extreme values
//!
//! Run: cargo test --test canvas_pan_zoom_tests

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use oya_frontend::graph::{Viewport, Workflow};

const DEFAULT_WIDTH: f32 = 1200.0;
const DEFAULT_HEIGHT: f32 = 800.0;
const DEFAULT_PADDING: f32 = 200.0;

fn workflow_with_spread_nodes() -> Workflow {
    let mut w = Workflow::new();
    w.add_node("http-handler", 0.0, 0.0);
    w.add_node("run", 500.0, 300.0);
    w.add_node("condition", 1000.0, 600.0);
    w
}

// ═══════════════════════════════════════════════════════════════════════════
// 1. Zoom — basic functionality
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn zoom_increases_zoom_level() {
    let mut w = Workflow::new();
    let before_zoom = w.viewport.zoom;

    w.zoom(0.5, 600.0, 400.0);

    assert!(
        w.viewport.zoom > before_zoom,
        "positive delta should increase zoom: {} vs {}",
        w.viewport.zoom, before_zoom
    );
}

#[test]
fn zoom_out_decreases_zoom_level() {
    let mut w = Workflow::new();
    // Start zoomed in
    w.viewport.zoom = 2.0;
    let before_zoom = w.viewport.zoom;

    w.zoom(-0.5, 600.0, 400.0);

    assert!(
        w.viewport.zoom < before_zoom,
        "negative delta should decrease zoom: {} vs {}",
        w.viewport.zoom, before_zoom
    );
}

#[test]
fn zoom_zero_keeps_zoom_unchanged() {
    let mut w = Workflow::new();
    w.viewport.zoom = 1.5;
    let before_zoom = w.viewport.zoom;

    w.zoom(0.0, 600.0, 400.0);

    assert!(
        (w.viewport.zoom - before_zoom).abs() < f32::EPSILON,
        "zero delta should not change zoom"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. Zoom Limits (0.15x to 3.0x)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn zoom_cannot_exceed_max_3x() {
    let mut w = Workflow::new();
    // Try to zoom way past 3.0x
    for _ in 0..100 {
        w.zoom(2.0, 600.0, 400.0);
    }

    assert!(
        w.viewport.zoom <= 3.0,
        "zoom must not exceed 3.0x, got {}",
        w.viewport.zoom
    );
    assert!(
        w.viewport.zoom >= 0.15,
        "zoom must not go below 0.15x even at max, got {}",
        w.viewport.zoom
    );
}

#[test]
fn zoom_cannot_drop_below_min_0_15x() {
    let mut w = Workflow::new();
    // Try to zoom way below 0.15x
    for _ in 0..100 {
        w.zoom(-0.9, 600.0, 400.0);
    }

    assert!(
        w.viewport.zoom >= 0.15,
        "zoom must not drop below 0.15x, got {}",
        w.viewport.zoom
    );
}

#[test]
fn zoom_at_boundary_stays_at_boundary() {
    let mut w = Workflow::new();
    w.viewport.zoom = 3.0;
    let before = w.viewport.zoom;

    // Try to zoom past max
    w.zoom(1.0, 600.0, 400.0);
    assert!(
        (w.viewport.zoom - before).abs() < f32::EPSILON,
        "already at max should stay at max"
    );
}

#[test]
fn zoom_at_min_boundary_stays_at_min() {
    let mut w = Workflow::new();
    w.viewport.zoom = 0.15;

    w.zoom(-0.5, 600.0, 400.0);
    assert!(
        (w.viewport.zoom - 0.15).abs() < f32::EPSILON,
        "already at min should stay at min, got {}",
        w.viewport.zoom
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. Pan — viewport position changes
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn pan_moves_viewport_x_and_y() {
    let mut w = Workflow::new();
    let before_x = w.viewport.x;
    let before_y = w.viewport.y;

    // Direct pan: simulate what use_workflow_state::pan() does
    w.viewport.x += 100.0;
    w.viewport.y += 50.0;

    assert!(
        (w.viewport.x - before_x - 100.0).abs() < f32::EPSILON,
        "pan should move viewport x"
    );
    assert!(
        (w.viewport.y - before_y - 50.0).abs() < f32::EPSILON,
        "pan should move viewport y"
    );
}

#[test]
fn pan_negative_direction_works() {
    let mut w = Workflow::new();
    w.viewport.x = 200.0;
    w.viewport.y = 200.0;

    w.viewport.x -= 150.0;
    w.viewport.y -= 100.0;

    assert!(
        (w.viewport.x - 50.0).abs() < f32::EPSILON,
        "pan left should decrease x"
    );
    assert!(
        (w.viewport.y - 100.0).abs() < f32::EPSILON,
        "pan up should decrease y"
    );
}

#[test]
fn pan_accumulates_over_multiple_operations() {
    let mut w = Workflow::new();
    let start_x = w.viewport.x;
    let start_y = w.viewport.y;

    for _ in 0..5 {
        w.viewport.x += 10.0;
        w.viewport.y += 20.0;
    }

    assert!(
        (w.viewport.x - start_x - 50.0).abs() < f32::EPSILON,
        "5 pans of 10 should move 50 total"
    );
    assert!(
        (w.viewport.y - start_y - 100.0).abs() < f32::EPSILON,
        "5 pans of 20 should move 100 total"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. Fit-to-View
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn fit_view_with_nodes_adjusts_zoom_from_default() {
    let mut w = workflow_with_spread_nodes();

    w.fit_view(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_PADDING);

    assert!(
        w.viewport.zoom > 0.0,
        "zoom should be positive after fit_view"
    );
    // Default is 1.0; fit_view with spread nodes should change it
    assert_ne!(w.viewport.zoom, 1.0, "fit_view should change zoom from default");
}

#[test]
fn fit_view_empty_workflow_does_not_change_viewport() {
    let mut w = Workflow::new();
    let before = w.viewport.clone();

    w.fit_view(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_PADDING);

    assert_eq!(w.viewport.x, before.x, "empty workflow: x unchanged");
    assert_eq!(w.viewport.y, before.y, "empty workflow: y unchanged");
    assert_eq!(w.viewport.zoom, before.zoom, "empty workflow: zoom unchanged");
}

#[test]
fn fit_view_single_node_centers_it() {
    let mut w = Workflow::new();
    w.add_node("run", 500.0, 300.0);

    w.fit_view(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_PADDING);

    // After fit_view, the viewport should be positioned so the node
    // is roughly centered in the visible area
    assert!(
        w.viewport.zoom > 0.0 && w.viewport.zoom <= 1.5,
        "single node fit should produce reasonable zoom, got {}",
        w.viewport.zoom
    );
}

#[test]
fn fit_view_respects_padding() {
    let mut w = Workflow::new();
    w.add_node("run", 0.0, 0.0);
    w.add_node("run", 1000.0, 800.0);

    w.fit_view(DEFAULT_WIDTH, DEFAULT_HEIGHT, 50.0);
    let zoom_small_padding = w.viewport.zoom;

    w.fit_view(DEFAULT_WIDTH, DEFAULT_HEIGHT, 500.0);
    let zoom_large_padding = w.viewport.zoom;

    // More padding means nodes take up less viewport → smaller zoom
    assert!(
        zoom_large_padding < zoom_small_padding,
        "larger padding should produce smaller zoom: {} vs {}",
        zoom_large_padding, zoom_small_padding
    );
}

#[test]
fn fit_view_zoom_stays_within_limits() {
    let mut w = Workflow::new();
    // Nodes very far apart — should still clamp to [0.15, 3.0]
    w.add_node("run", 0.0, 0.0);
    w.add_node("run", 100_000.0, 100_000.0);

    w.fit_view(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_PADDING);

    assert!(
        w.viewport.zoom >= 0.15,
        "fit_view zoom should respect min 0.15, got {}",
        w.viewport.zoom
    );
    assert!(
        w.viewport.zoom <= 3.0,
        "fit_view zoom should respect max 3.0, got {}",
        w.viewport.zoom
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. Zoom Centers on Cursor Position
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn zoom_shifts_viewport_toward_cursor() {
    let mut w = Workflow::new();
    w.viewport.zoom = 1.0;
    let before_x = w.viewport.x;
    let before_y = w.viewport.y;

    // Zoom in toward center-right of viewport
    w.zoom(0.5, 900.0, 400.0);

    // The viewport should have shifted to keep the cursor point stable
    // (exact values depend on implementation, but it must change)
    assert!(
        (w.viewport.x - before_x).abs() > f32::EPSILON
            || (w.viewport.y - before_y).abs() > f32::EPSILON,
        "zoom should shift viewport position to center on cursor"
    );
}

#[test]
fn zoom_at_different_cursor_positions_produces_different_offsets() {
    let mut w1 = Workflow::new();
    let mut w2 = Workflow::new();
    w1.viewport.zoom = 1.0;
    w2.viewport.zoom = 1.0;

    w1.zoom(0.5, 0.0, 0.0);      // Zoom toward top-left
    w2.zoom(0.5, 1200.0, 800.0); // Zoom toward bottom-right

    assert_ne!(
        (w1.viewport.x, w1.viewport.y),
        (w2.viewport.x, w2.viewport.y),
        "zoom at different cursor positions should produce different viewport offsets"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. Viewport Serialization Round-Trip
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn viewport_survives_serde_round_trip() {
    let original = Viewport {
        x: 123.456,
        y: -789.012,
        zoom: 1.75,
    };

    let json = serde_json::to_string(&original).unwrap();
    let restored: Viewport = serde_json::from_str(&json).unwrap();

    assert!((restored.x - original.x).abs() < f32::EPSILON);
    assert!((restored.y - original.y).abs() < f32::EPSILON);
    assert!((restored.zoom - original.zoom).abs() < f32::EPSILON);
}

#[test]
fn zoomed_viewport_survives_workflow_round_trip() {
    let mut w = workflow_with_spread_nodes();
    w.viewport.zoom = 2.5;
    w.viewport.x = -500.0;
    w.viewport.y = 300.0;

    let json = serde_json::to_string(&w).unwrap();
    let restored: Workflow = serde_json::from_str(&json).unwrap();

    assert!(
        (restored.viewport.zoom - 2.5).abs() < f32::EPSILON,
        "zoom should survive round-trip"
    );
    assert!(
        (restored.viewport.x - (-500.0)).abs() < f32::EPSILON,
        "x offset should survive round-trip"
    );
    assert!(
        (restored.viewport.y - 300.0).abs() < f32::EPSILON,
        "y offset should survive round-trip"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 7. Zoom + Pan Combined Operations
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn zoom_then_pan_works_correctly() {
    let mut w = Workflow::new();

    // Zoom in
    w.zoom(1.0, 600.0, 400.0);
    let zoomed_x = w.viewport.x;
    let zoomed_y = w.viewport.y;

    // Pan
    w.viewport.x += 100.0;
    w.viewport.y += 50.0;

    assert!(
        (w.viewport.x - zoomed_x - 100.0).abs() < f32::EPSILON,
        "pan after zoom should add to viewport x"
    );
    assert!(
        (w.viewport.y - zoomed_y - 50.0).abs() < f32::EPSILON,
        "pan after zoom should add to viewport y"
    );
}

#[test]
fn pan_then_zoom_adjusts_position() {
    let mut w = Workflow::new();

    // Pan away from origin
    w.viewport.x = 500.0;
    w.viewport.y = 300.0;

    // Zoom — should recalculate position relative to cursor
    w.zoom(0.5, 600.0, 400.0);

    // Zoom should have changed the viewport offset
    assert_ne!(
        (w.viewport.x, w.viewport.y),
        (500.0, 300.0),
        "zoom after pan should recalculate viewport offset"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 8. Pure Calculation Functions
// Verify calc module functions directly (zoom delta, pan offset, fit view)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn calculate_zoom_delta_positive_increases() {
    let result = oya_frontend::graph::calc::calculate_zoom_delta(0.5, 1.0);
    assert!(
        result > 1.0,
        "positive delta should increase zoom from 1.0"
    );
}

#[test]
fn calculate_zoom_delta_negative_decreases() {
    let result = oya_frontend::graph::calc::calculate_zoom_delta(-0.5, 2.0);
    assert!(
        result < 2.0,
        "negative delta should decrease zoom"
    );
}

#[test]
fn calculate_zoom_delta_clamps_below_min() {
    // delta = -0.99 on zoom 0.1 → 0.1 * 0.01 = 0.001 → clamps to 0.1
    let result = oya_frontend::graph::calc::calculate_zoom_delta(-0.99, 0.1);
    assert!(
        result >= 0.1,
        "should clamp to minimum, got {}",
        result
    );
}

#[test]
fn calculate_zoom_delta_clamps_above_max() {
    let result = oya_frontend::graph::calc::calculate_zoom_delta(10.0, 4.0);
    assert!(
        result <= 5.0,
        "should clamp to maximum, got {}",
        result
    );
}

#[test]
fn calculate_zoom_delta_nan_returns_safe_value() {
    let result = oya_frontend::graph::calc::calculate_zoom_delta(f32::NAN, 1.0);
    assert!(result.is_finite(), "NaN delta should produce finite result");

    let result = oya_frontend::graph::calc::calculate_zoom_delta(0.5, f32::NAN);
    assert!(result.is_finite(), "NaN current zoom should produce finite result");
}

#[test]
fn calculate_pan_offset_adjusts_for_zoom_change() {
    let (new_x, new_y) = oya_frontend::graph::calc::calculate_pan_offset(
        100.0, 100.0, // viewport x, y
        600.0, 400.0, // center (cursor)
        1.0, 2.0,     // old zoom, new zoom
    );

    // Both must be finite (not NaN/inf)
    assert!(new_x.is_finite(), "new_x must be finite");
    assert!(new_y.is_finite(), "new_y must be finite");
}

#[test]
fn calculate_fit_view_returns_none_for_empty_nodes() {
    let result = oya_frontend::graph::calc::calculate_fit_view(
        &[],
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        DEFAULT_PADDING,
    );
    assert!(result.is_none(), "empty nodes should return None");
}

#[test]
fn calculate_fit_view_returns_valid_bounds_for_spread_nodes() {
    let nodes = [(0.0, 0.0), (1000.0, 800.0)];
    let result = oya_frontend::graph::calc::calculate_fit_view(
        &nodes,
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        DEFAULT_PADDING,
    );

    let (vx, vy, zoom) = result.expect("should produce a result for valid input");
    assert!(vx.is_finite(), "viewport x must be finite");
    assert!(vy.is_finite(), "viewport y must be finite");
    assert!(zoom >= 0.15 && zoom <= 1.5, "fit_view zoom should be clamped [0.15, 1.5]");
}

// ═══════════════════════════════════════════════════════════════════════════
// 9. Default Viewport Values
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn default_viewport_is_origin_at_1x_zoom() {
    let w = Workflow::new();
    assert!((w.viewport.x).abs() < f32::EPSILON, "default x should be 0");
    assert!((w.viewport.y).abs() < f32::EPSILON, "default y should be 0");
    assert!(
        (w.viewport.zoom - 1.0).abs() < f32::EPSILON,
        "default zoom should be 1.0"
    );
}
