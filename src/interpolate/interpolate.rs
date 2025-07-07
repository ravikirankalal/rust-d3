use palette::{FromColor, Hsl, Srgb, RgbHue};
use serde_json::{self, Value};

// Linear interpolation implementation

pub fn interpolate(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}


pub fn interpolate_array<T>(a: &[T], b: &[T], t: f64) -> Vec<T>
where
    T: Copy + Into<f64> + From<f64>,
{
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let x_f: f64 = x.into();
            let y_f: f64 = y.into();
            T::from(x_f + (y_f - x_f) * t)
        })
        .collect()
}

pub fn interpolate_round(a: f64, b: f64, t: f64) -> f64 {
    (a + (b - a) * t).round()
}

fn parse_hex_color(s: &str) -> Srgb<f32> {
    let s = s.trim_start_matches('#');
    if s.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&s[0..2], 16),
            u8::from_str_radix(&s[2..4], 16),
            u8::from_str_radix(&s[4..6], 16),
        ) {
            return Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        }
    }
    Srgb::new(0.0, 0.0, 0.0)
}

pub fn interpolate_rgb(a: &str, b: &str, t: f64) -> String {
    let parse = |s: &str| {
        if s.starts_with('#') {
            parse_hex_color(s)
        } else {
            let vals: Vec<f32> = s.split(',').filter_map(|v| v.trim().parse().ok()).collect();
            if vals.len() == 3 {
                Srgb::new(vals[0] / 255.0, vals[1] / 255.0, vals[2] / 255.0)
            } else {
                Srgb::new(0.0, 0.0, 0.0)
            }
        }
    };
    let c1 = parse(a);
    let c2 = parse(b);
    let r = c1.red + (c2.red - c1.red) * t as f32;
    let g = c1.green + (c2.green - c1.green) * t as f32;
    let b = c1.blue + (c2.blue - c1.blue) * t as f32;
    let rgb = Srgb::new(r, g, b).into_format::<u8>();
    format!("#{:02x}{:02x}{:02x}", rgb.red, rgb.green, rgb.blue)
}

pub fn interpolate_hsl(a: &str, b: &str, t: f64) -> String {
    let parse = |s: &str| {
        if s.starts_with('#') {
            let rgb = parse_hex_color(s);
            Hsl::from_color(rgb)
        } else {
            let vals: Vec<f32> = s.split(',').filter_map(|v| v.trim().parse().ok()).collect();
            if vals.len() == 3 {
                Hsl::from_color(Srgb::new(vals[0] / 255.0, vals[1] / 255.0, vals[2] / 255.0))
            } else {
                Hsl::new(0.0, 0.0, 0.0)
            }
        }
    };
    let h1 = parse(a);
    let h2 = parse(b);
    let h = f32::from(h1.hue) + (f32::from(h2.hue) - f32::from(h1.hue)) * t as f32;
    let s = h1.saturation + (h2.saturation - h1.saturation) * t as f32;
    let l = h1.lightness + (h2.lightness - h1.lightness) * t as f32;
    let hsl = Hsl::new(RgbHue::from_degrees(h), s, l);
    let rgb = Srgb::from_color(hsl).into_format::<u8>();
    format!("#{:02x}{:02x}{:02x}", rgb.red, rgb.green, rgb.blue)
}

pub fn interpolate_object<T>(a: &T, b: &T, t: f64) -> T
where
    T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    // Interpolate all numeric fields, copy others from a
    let a_json = serde_json::to_value(a).unwrap();
    let b_json = serde_json::to_value(b).unwrap();
    let mut result = a_json.clone();
    if let (Value::Object(a_map), Value::Object(b_map)) = (&a_json, &b_json) {
        for (k, v_a) in a_map {
            if let Some(v_b) = b_map.get(k) {
                let v = match (v_a, v_b) {
                    (Value::Number(na), Value::Number(nb)) => {
                        let fa = na.as_f64().unwrap_or(0.0);
                        let fb = nb.as_f64().unwrap_or(0.0);
                        Value::Number(serde_json::Number::from_f64(fa + (fb - fa) * t).unwrap())
                    }
                    _ => v_a.clone(),
                };
                result[k] = v;
            }
        }
    }
    serde_json::from_value(result).unwrap_or_else(|_| a.clone())
}

/// Interpolates between two numbers (alias for interpolate)
pub fn interpolate_number(a: f64, b: f64, t: f64) -> f64 {
    interpolate(a, b, t)
}

/// Interpolates between two strings (simple character-wise interpolation)
pub fn interpolate_string(a: &str, b: &str, t: f64) -> String {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let len = a_bytes.len().max(b_bytes.len());
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        let ac = *a_bytes.get(i).unwrap_or(&b' ');
        let bc = *b_bytes.get(i).unwrap_or(&b' ');
        let c = ac as f64 + (bc as f64 - ac as f64) * t;
        result.push(c.round() as u8);
    }
    String::from_utf8_lossy(&result).to_string()
}
