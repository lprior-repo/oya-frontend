//! Unit tests for node drag behavior
//!
//! These tests verify the logic of node dragging without requiring a browser.
//! The formula is: new_pos = (current + delta/10).round() * 10

#[cfg(test)]
mod tests {
    use oya_frontend::graph::calc::update_node_position;

    #[test]
    fn test_update_node_position_normal() {
        // Normal drag: (100 + 10/10).round() * 10 = (101).round() * 10 = 1010
        // (200 + 20/10).round() * 10 = (202).round() * 10 = 2020
        let (new_x, new_y) = update_node_position(100.0, 200.0, 10.0, 20.0);
        assert_eq!(new_x, 1010.0);
        assert_eq!(new_y, 2020.0);
    }

    #[test]
    fn test_update_node_position_nan_delta() {
        // NaN delta should not update position
        let (new_x, new_y) = update_node_position(100.0, 200.0, f32::NAN, 20.0);
        assert_eq!(new_x, 100.0);
        assert_eq!(new_y, 200.0);
    }

    #[test]
    fn test_update_node_position_infinite_delta() {
        // Infinite delta should not update position
        let (new_x, new_y) = update_node_position(100.0, 200.0, 10.0, f32::INFINITY);
        assert_eq!(new_x, 100.0);
        assert_eq!(new_y, 200.0);
    }

    #[test]
    fn test_update_node_position_nan_current_x() {
        // NaN current x should not update
        let (new_x, new_y) = update_node_position(f32::NAN, 200.0, 10.0, 20.0);
        assert!(new_x.is_nan());
        assert_eq!(new_y, 200.0);
    }

    #[test]
    fn test_update_node_position_zero_delta() {
        // Zero delta: (100 + 0/10).round() * 10 = 1000
        let (new_x, new_y) = update_node_position(100.0, 200.0, 0.0, 0.0);
        assert_eq!(new_x, 1000.0);
        assert_eq!(new_y, 2000.0);
    }

    #[test]
    fn test_update_node_position_clamp_positive() {
        // Large positive position should be clamped to 100000
        let (new_x, _new_y) = update_node_position(500000.0, 500000.0, 0.0, 0.0);
        // (500000 + 0/10).round() * 10 = 5000000 but clamped to 100000
        assert_eq!(_new_y, 100000.0);
        assert_eq!(new_x, 100000.0);
    }

    #[test]
    fn test_update_node_position_negative_infinite_delta() {
        // Negative infinite delta should not update position
        let (new_x, new_y) = update_node_position(100.0, 200.0, f32::NEG_INFINITY, 0.0);
        assert_eq!(new_x, 100.0);
        assert_eq!(new_y, 200.0);
    }
}
