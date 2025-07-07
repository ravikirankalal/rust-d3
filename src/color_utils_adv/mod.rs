//! D3 Color Utils Advanced module
//! Advanced color utilities for D3.js API parity.

/// D3.js: d3.interpolateCubehelix (stub)
pub fn interpolate_cubehelix(_a: &str, _b: &str, _t: f64) -> String {
    // TODO: Implement Cubehelix interpolation
    String::new()
}

/// D3.js: d3.colorSpace (stub)
pub fn color_space_placeholder() -> &'static str {
    "colorSpace not implemented"
}

/// Checks if two RGB colors have sufficient contrast (WCAG AA).
pub fn is_contrasting(a: (u8, u8, u8), b: (u8, u8, u8)) -> bool {
    let luminance = |(r, g, b): (u8, u8, u8)| {
        let f = |c| {
            let c = c as f32 / 255.0;
            if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * f(r) + 0.7152 * f(g) + 0.0722 * f(b)
    };
    let l1 = luminance(a).max(luminance(b));
    let l2 = luminance(a).min(luminance(b));
    (l1 + 0.05) / (l2 + 0.05) > 4.5
}
