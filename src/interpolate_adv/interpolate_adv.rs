pub fn interpolate_array(a: &[f64], b: &[f64], t: f64) -> Vec<f64> {
    a.iter().zip(b.iter())
        .map(|(a, b)| a + (b - a) * t)
        .collect()
}

pub fn interpolate_color_rgb(a: (u8, u8, u8), b: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    let lerp = |a, b| (a as f64 + (b as f64 - a as f64) * t).round() as u8;
    (lerp(a.0, b.0), lerp(a.1, b.1), lerp(a.2, b.2))
}
