
// src/color/convert.rs

pub fn rgb_to_xyz(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = r / 255.0;
    let g = g / 255.0;
    let b = b / 255.0;

    let r = if r > 0.04045 { ((r + 0.055) / 1.055).powf(2.4) } else { r / 12.92 };
    let g = if g > 0.04045 { ((g + 0.055) / 1.055).powf(2.4) } else { g / 12.92 };
    let b = if b > 0.04045 { ((b + 0.055) / 1.055).powf(2.4) } else { b / 12.92 };

    let x = (r * 0.4124564 + g * 0.3575761 + b * 0.1804375) * 100.0;
    let y = (r * 0.2126729 + g * 0.7151522 + b * 0.0721750) * 100.0;
    let z = (r * 0.0193339 + g * 0.1191920 + b * 0.9503041) * 100.0;

    (x, y, z)
}

pub fn xyz_to_lab(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let xr = 95.047; // D65 white point
    let yr = 100.0;
    let zr = 108.883;

    let fx = f(x / xr);
    let fy = f(y / yr);
    let fz = f(z / zr);

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);

    (l, a, b)
}

pub fn f(t: f32) -> f32 {
    const EPSILON: f32 = 0.008856;
    const KAPPA: f32 = 903.3;
    if t > EPSILON.powf(3.0) { t.powf(1.0 / 3.0) } else { (KAPPA * t + 16.0) / 116.0 }
}

pub fn lab_to_xyz(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let yr = 100.0;
    let xr = 95.047;
    let zr = 108.883;

    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;

    let x = if fx.powf(3.0) > 0.008856 { fx.powf(3.0) } else { (116.0 * fx - 16.0) / 903.3 };
    let y = if fy.powf(3.0) > 0.008856 { fy.powf(3.0) } else { (116.0 * fy - 16.0) / 903.3 };
    let z = if fz.powf(3.0) > 0.008856 { fz.powf(3.0) } else { (116.0 * fz - 16.0) / 903.3 };

    (x * xr, y * yr, z * zr)
}

pub fn xyz_to_rgb(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let x = x / 100.0;
    let y = y / 100.0;
    let z = z / 100.0;

    let r = x * 3.2404542 + y * -1.5371385 + z * -0.4985314;
    let g = x * -0.9692660 + y * 1.8760108 + z * 0.0415560;
    let b = x * 0.0556434 + y * -0.2040259 + z * 1.0572252;

    let r = if r > 0.0031308 { 1.055 * r.powf(1.0 / 2.4) - 0.055 } else { 12.92 * r };
    let g = if g > 0.0031308 { 1.055 * g.powf(1.0 / 2.4) - 0.055 } else { 12.92 * g };
    let b = if b > 0.0031308 { 1.055 * b.powf(1.0 / 2.4) - 0.055 } else { 12.92 * b };

    (r.max(0.0).min(1.0), g.max(0.0).min(1.0), b.max(0.0).min(1.0))
}
