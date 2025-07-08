//! Perceptual and cubehelix palettes for d3-scale-chromatic
// This module provides perceptual palettes: inferno, magma, plasma, cividis, turbo, cubehelix
// These are ported from d3-scale-chromatic and use the palette crate for interpolation.

use palette::LinSrgb;

fn hex_color(rgb: LinSrgb) -> String {
    let (r, g, b) = (rgb.red, rgb.green, rgb.blue);
    format!("#{:02x}{:02x}{:02x}",
        (r.clamp(0.0, 1.0) * 255.0).round() as u8,
        (g.clamp(0.0, 1.0) * 255.0).round() as u8,
        (b.clamp(0.0, 1.0) * 255.0).round() as u8)
}

fn lerp(a: LinSrgb, b: LinSrgb, t: f64) -> LinSrgb {
    let t = t as f32;
    LinSrgb::new(
        a.red + (b.red - a.red) * t,
        a.green + (b.green - a.green) * t,
        a.blue + (b.blue - a.blue) * t,
    )
}

fn interpolate_palette(stops: &[LinSrgb], t: f64) -> String {
    let n = stops.len();
    if n == 0 {
        return "#000000".to_string();
    }
    if n == 1 || t <= 0.0 {
        return hex_color(stops[0]);
    }
    if t >= 1.0 {
        return hex_color(stops[n - 1]);
    }
    let scaled = t * (n as f64 - 1.0);
    let i = scaled.floor() as usize;
    let frac = scaled - i as f64;
    hex_color(lerp(stops[i], stops[i + 1], frac))
}

macro_rules! perceptual_palette {
    ($name:ident, $stops:expr) => {
        pub fn $name(t: f64) -> String {
            interpolate_palette(&$stops, t.clamp(0.0, 1.0))
        }
    };
}

perceptual_palette!(inferno, [
    LinSrgb::new(0.0015, 0.0005, 0.0139),
    LinSrgb::new(0.220, 0.029, 0.206),
    LinSrgb::new(0.572, 0.384, 0.008),
    LinSrgb::new(0.988, 0.998, 0.644)
]);
perceptual_palette!(magma, [
    LinSrgb::new(0.0015, 0.0005, 0.0139),
    LinSrgb::new(0.251, 0.063, 0.355),
    LinSrgb::new(0.788, 0.294, 0.145),
    LinSrgb::new(0.987, 0.991, 0.749)
]);
perceptual_palette!(plasma, [
    LinSrgb::new(0.050, 0.030, 0.527),
    LinSrgb::new(0.518, 0.003, 0.753),
    LinSrgb::new(0.998, 0.972, 0.643)
]);
perceptual_palette!(cividis, [
    LinSrgb::new(0.000, 0.135, 0.304),
    LinSrgb::new(0.327, 0.475, 0.537),
    LinSrgb::new(0.996, 0.984, 0.643)
]);
perceptual_palette!(turbo, [
    LinSrgb::new(0.18995, 0.07176, 0.23217),
    LinSrgb::new(0.25107, 0.25237, 0.63374),
    LinSrgb::new(0.27628, 0.69115, 0.48336),
    LinSrgb::new(0.98803, 0.99883, 0.64492)
]);
// Cubehelix example (simplified)
perceptual_palette!(cubehelix, [
    LinSrgb::new(0.0, 0.0, 0.0),
    LinSrgb::new(0.5, 0.5, 0.0),
    LinSrgb::new(1.0, 1.0, 1.0)
]);
