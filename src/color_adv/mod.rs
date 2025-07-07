//! D3 Color Advanced module
//! Advanced color spaces, blending, etc.

/// Blends two RGB colors by a given ratio (0.0-1.0).
pub fn blend_colors(a: (u8, u8, u8), b: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let blend = |x, y| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    (blend(a.0, b.0), blend(a.1, b.1), blend(a.2, b.2))
}
