// D3 polygon module for Rust D3
// Provides basic polygon utilities: area, centroid, contains.

pub fn area(polygon: &[(f64, f64)]) -> f64 {
    let n = polygon.len();
    if n < 3 {
        return 0.0;
    }
    let mut a = 0.0;
    for i in 0..n {
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[(i + 1) % n];
        a += x0 * y1 - x1 * y0;
    }
    a.abs() / 2.0
}

pub fn centroid(polygon: &[(f64, f64)]) -> (f64, f64) {
    let n = polygon.len();
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut a = 0.0;
    for i in 0..n {
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[(i + 1) % n];
        let cross = x0 * y1 - x1 * y0;
        cx += (x0 + x1) * cross;
        cy += (y0 + y1) * cross;
        a += cross;
    }
    let a = a / 2.0;
    if a == 0.0 {
        return (0.0, 0.0);
    }
    (cx / (6.0 * a), cy / (6.0 * a))
}

pub fn contains(polygon: &[(f64, f64)], point: (f64, f64)) -> bool {
    let (px, py) = point;
    let mut inside = false;
    let n = polygon.len();
    for i in 0..n {
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[(i + 1) % n];
        if ((y0 > py) != (y1 > py)) && (px < (x1 - x0) * (py - y0) / (y1 - y0 + 1e-12) + x0) {
            inside = !inside;
        }
    }
    inside
}

/// Placeholder for d3-polygon API parity.
/// See: https://github.com/d3/d3-polygon#api-reference
/// TODO: Implement full API parity with d3-polygon (polygonLength, polygonHull, polygonContains, etc.)

pub fn polygon_length(polygon: &[(f64, f64)]) -> f64 {
    if polygon.len() < 2 {
        return 0.0;
    }
    let mut len = 0.0;
    for i in 0..polygon.len() {
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[(i + 1) % polygon.len()];
        len += ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
    }
    len
}

pub fn polygon_hull(_points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    // polygon_hull module removed; implement or stub as needed
    // TODO: Implement convex hull algorithm here
    todo!("convex_hull not implemented: polygon_hull module removed during unification cleanup")
}
