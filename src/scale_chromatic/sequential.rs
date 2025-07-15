//! d3-scale-chromatic: Sequential color schemes and interpolators

/// Interpolate Viridis (0.0-1.0)
pub fn interpolate_viridis(t: f64) -> &'static str {
    // This is a stub. For real use, use a lookup table or color math.
    match t {
        t if t <= 0.0 => "#440154",
        t if t >= 1.0 => "#fde725",
        t if t < 0.5 => "#31688e",
        _ => "#b5de2b",
    }
}

/// Viridis color palette (7 steps)
pub const VIRIDIS: [&str; 7] = [
    "#440154", "#31688e", "#35b779", "#fde725", "#b5de2b", "#21918c", "#3f4788",
];

/// Plasma color palette (7 steps)
pub const PLASMA: [&str; 7] = [
    "#0d0887", "#6a00a8", "#cb4679", "#f89441", "#f0f921", "#b12a90", "#ed7953",
];

/// Interpolate Plasma (0.0-1.0)
pub fn interpolate_plasma(t: f64) -> &'static str {
    match t {
        t if t <= 0.0 => "#0d0887",
        t if t >= 1.0 => "#f0f921",
        t if t < 0.5 => "#cb4679",
        _ => "#f89441",
    }
}
