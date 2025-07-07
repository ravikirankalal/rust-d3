//! D3 Zoom Advanced module
//! Advanced zooming, e.g., zoom constraints, custom behaviors, etc.

/// Clamps a zoom scale within min/max bounds.
pub fn zoom_clamp(scale: f64, min: f64, max: f64) -> f64 {
    scale.max(min).min(max)
}
