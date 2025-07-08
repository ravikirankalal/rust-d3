//! HSL color interpolation for d3-interpolate

use crate::interpolate::rgb::hex_to_hsl;
use crate::interpolate::rgb::hsl_to_hex;

/// Interpolates between two colors in HSL space.
pub fn interpolate_hsl(a: &str, b: &str, t: f64) -> String {
    let (h1, s1, l1) = hex_to_hsl(a);
    let (h2, s2, l2) = hex_to_hsl(b);
    // Interpolate hue correctly (circular)
    let mut dh = h2 - h1;
    if dh > 180.0 {
        dh -= 360.0;
    } else if dh < -180.0 {
        dh += 360.0;
    }
    let h = (h1 + t * dh + 360.0) % 360.0;
    let s = s1 + t * (s2 - s1);
    let l = l1 + t * (l2 - l1);
    hsl_to_hex(h, s, l)
}
