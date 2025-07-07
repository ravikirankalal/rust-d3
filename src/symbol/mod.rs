//! D3 Symbol module
//! Symbol generators, e.g., symbolCircle, symbolSquare, etc.

/// Returns SVG path for a D3 symbol type and size
pub fn symbol(symbol_type: &str, size: f64) -> String {
    match symbol_type {
        "circle" => symbol_circle(size),
        "cross" => symbol_cross(size),
        "diamond" => symbol_diamond(size),
        "square" => symbol_square(size),
        "star" => symbol_star(size),
        "triangle" => symbol_triangle(size),
        "wye" => symbol_wye(size),
        _ => String::new(),
    }
}

pub fn symbol_circle(size: f64) -> String {
    let r = (size / std::f64::consts::PI).sqrt();
    format!(
        "M{:.6} 0A{:.6} {:.6} 0 1 1 {:.6} 0A{:.6} {:.6} 0 1 1 {:.6} 0Z",
        r, r, r, -r, r, r, r
    )
}

pub fn symbol_cross(size: f64) -> String {
    let s = (size / 5.0).sqrt();
    // Simple cross: vertical and horizontal lines
    format!("M{:.3},0L{:.3},0M0,{:.3}L0,{:.3}", -s, s, -s, s)
}

pub fn symbol_diamond(size: f64) -> String {
    let y = (size / (2.0 * (3.0f64).sqrt())).sqrt();
    let x = y * (3.0f64).sqrt();
    format!("M0,{:.3}L{:.3},0L0,{:.3}L{:.3},0Z", -y, x, y, -x)
}

pub fn symbol_square(size: f64) -> String {
    let w = size.sqrt() / 2.0;
    format!("M{:.3},{:.3}L{:.3},{:.3}L{:.3},{:.3}L{:.3},{:.3}Z", -w, -w, w, -w, w, w, -w, w)
}

pub fn symbol_star(size: f64) -> String {
    let r = (size * 1.25 / std::f64::consts::PI).sqrt();
    let mut path = String::new();
    for i in 0..10 {
        let a = i as f64 * std::f64::consts::PI / 5.0;
        let r_i = if i % 2 == 0 { r } else { r / 2.5 };
        let x = r_i * a.sin();
        let y = -r_i * a.cos();
        if i == 0 {
            path += &format!("M{},{}", x, y);
        } else {
            path += &format!("L{},{}", x, y);
        }
    }
    path + "Z"
}

pub fn symbol_triangle(size: f64) -> String {
    let h = (size * (4.0 / (3.0f64).sqrt())).sqrt();
    let y = -h / 2.0;
    let x = h / (2.0 * (3.0f64).sqrt());
    format!("M0,{}L{},{}L{},{}Z", y, x, -y, -x, -y)
}

pub fn symbol_wye(size: f64) -> String {
    let r = (size / ((3.0f64).sqrt() * 2.0)).sqrt();
    // Simple placeholder for wye
    format!("M0,0L{:.3},{:.3}L{:.3},{:.3}Z", r, r, -r, r)
}
