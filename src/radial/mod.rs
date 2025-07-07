//! D3 Radial module
//! Provides radial layouts and shapes (see d3-radial in JS).

/// Computes (x, y) coordinates for points evenly spaced around a circle.
pub fn radial_layout(n: usize, radius: f64) -> Vec<(f64, f64)> {
    (0..n)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            (radius * angle.cos(), radius * angle.sin())
        })
        .collect()
}
