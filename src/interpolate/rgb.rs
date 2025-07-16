//! d3-interpolate: RGB color interpolation (simple, not gamma-corrected)

/// Interpolate between two RGB colors (hex strings, e.g. "#ff0000")
pub fn interpolate_rgb(a: &str, b: &str, t: f64) -> String {
    fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        (r, g, b)
    }
    let (ar, ag, ab) = hex_to_rgb(a);
    let (br, bg, bb) = hex_to_rgb(b);
    let r = (ar as f64 + (br as f64 - ar as f64) * t).round() as u8;
    let g = (ag as f64 + (bg as f64 - ag as f64) * t).round() as u8;
    let b = (ab as f64 + (bb as f64 - ab as f64) * t).round() as u8;
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Convert hex color to HSL (simple, not gamma-corrected)
pub fn hex_to_hsl(hex: &str) -> (f64, f64, f64) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let (h, s);
    if max == min {
        h = 0.0;
        s = 0.0;
    } else {
        let d = max - min;
        s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };
        h = if max == r {
            (g - b) / d + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        } / 6.0;
    }
    (h, s, l)
}

/// Convert HSL to RGB hex string
pub fn hsl_to_hex(h: f64, s: f64, l: f64) -> String {
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
        let mut t = t;
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        p
    }
    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    format!(
        "#{:02x}{:02x}{:02x}",
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    )
}

/// Interpolate between two HSL colors (hex strings, e.g. "#ff0000")
pub fn interpolate_hsl(a: &str, b: &str, t: f64) -> String {
    let (h1, s1, l1) = hex_to_hsl(a);
    let (h2, s2, l2) = hex_to_hsl(b);
    let h = h1 + (h2 - h1) * t;
    let s = s1 + (s2 - s1) * t;
    let l = l1 + (l2 - l1) * t;
    hsl_to_hex(h, s, l)
}
