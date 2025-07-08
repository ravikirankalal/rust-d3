//! d3-scale-chromatic: Diverging color schemes and interpolators

/// Interpolate RdYlBu (0.0-1.0)
pub fn interpolate_rdyblu(t: f64) -> &'static str {
    // This is a stub. For real use, use a lookup table or color math.
    match t {
        t if t <= 0.0 => "#d73027",
        t if t >= 1.0 => "#313695",
        t if t < 0.5 => "#fee090",
        _ => "#4575b4",
    }
}

/// RdYlBu color palette (5 steps)
pub const RDYLBU: [&str; 5] = [
    "#d73027", "#fee090", "#ffffbf", "#4575b4", "#313695"
];
