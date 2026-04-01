//! ZoomFactor newtype enforcing the viewport zoom contract.
//!
//! Valid zoom range: `0.15 <= zoom <= 3.0`.

use serde::{Deserialize, Serialize};

/// Minimum allowed zoom level.
pub const MIN_ZOOM: f32 = 0.15;
/// Maximum allowed zoom level.
pub const MAX_ZOOM: f32 = 3.0;

/// A validated zoom factor in the range `[MIN_ZOOM, MAX_ZOOM]`.
///
/// Use [`ZoomFactor::new`] for fallible construction or
/// [`ZoomFactor::new_clamped`] for auto-clamping.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ZoomFactor(f32);

impl ZoomFactor {
    /// Create a new `ZoomFactor` returning `None` if the value falls
    /// outside `[MIN_ZOOM, MAX_ZOOM]`.
    #[must_use]
    pub fn new(value: f32) -> Option<Self> {
        if value >= MIN_ZOOM && value <= MAX_ZOOM {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Create a new `ZoomFactor`, clamping the value to `[MIN_ZOOM, MAX_ZOOM]`.
    #[must_use]
    pub fn new_clamped(value: f32) -> Self {
        Self(value.clamp(MIN_ZOOM, MAX_ZOOM))
    }

    /// Return the raw `f32` value.
    #[must_use]
    pub const fn value(&self) -> f32 {
        self.0
    }
}

impl Default for ZoomFactor {
    fn default() -> Self {
        Self(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{ZoomFactor, MAX_ZOOM, MIN_ZOOM};

    #[test]
    fn value_below_min_returns_none() {
        assert_eq!(ZoomFactor::new(0.14), None);
    }

    #[test]
    fn value_above_max_returns_none() {
        assert_eq!(ZoomFactor::new(3.1), None);
    }

    #[test]
    fn min_boundary_returns_some() {
        let zf = ZoomFactor::new(MIN_ZOOM);
        assert!(zf.is_some());
        assert_eq!(zf.map(|z| z.value()), Some(MIN_ZOOM));
    }

    #[test]
    fn max_boundary_returns_some() {
        let zf = ZoomFactor::new(MAX_ZOOM);
        assert!(zf.is_some());
        assert_eq!(zf.map(|z| z.value()), Some(MAX_ZOOM));
    }

    #[test]
    fn clamped_below_min_returns_min() {
        let zf = ZoomFactor::new_clamped(0.01);
        assert_eq!(zf.value(), MIN_ZOOM);
    }

    #[test]
    fn clamped_above_max_returns_max() {
        let zf = ZoomFactor::new_clamped(10.0);
        assert_eq!(zf.value(), MAX_ZOOM);
    }

    #[test]
    fn default_is_one() {
        assert_eq!(ZoomFactor::default().value(), 1.0);
    }
}
