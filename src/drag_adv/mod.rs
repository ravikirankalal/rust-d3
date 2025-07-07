//! D3 Drag Advanced module
//! Advanced drag behaviors, e.g., drag constraints, custom handles, etc.

/// Clamps a value within a min/max range during drag.
pub fn clamp_drag(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}
