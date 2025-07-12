//! d3-interpolate: Number interpolation

/// Linear interpolation between two numbers
pub fn interpolate_number(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
