//! d3-polygon: Polygon geometry utilities (Rust port)

/// Returns the signed area of the polygon.
pub fn polygon_area(points: &[(f64, f64)]) -> f64 {
    let n = points.len();
    if n < 3 {
        return 0.0;
    }
    let mut area = 0.0;
    for i in 0..n {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % n];
        area += x0 * y1 - x1 * y0;
    }
    area * 0.5
}

/// Returns the centroid of the polygon.
pub fn polygon_centroid(points: &[(f64, f64)]) -> (f64, f64) {
    let n = points.len();
    if n < 3 {
        return (0.0, 0.0);
    }
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut area = 0.0;
    for i in 0..n {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % n];
        let a = x0 * y1 - x1 * y0;
        cx += (x0 + x1) * a;
        cy += (y0 + y1) * a;
        area += a;
    }
    if area == 0.0 {
        return (0.0, 0.0);
    }
    (cx / (3.0 * area), cy / (3.0 * area))
}

/// Returns the length (perimeter) of the polygon.
pub fn polygon_length(points: &[(f64, f64)]) -> f64 {
    let n = points.len();
    if n < 2 {
        return 0.0;
    }
    let mut len = 0.0;
    for i in 0..n {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % n];
        len += ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
    }
    len
}

/// Returns true if the point (x, y) is inside the polygon.
pub fn polygon_contains(points: &[(f64, f64)], x: f64, y: f64) -> bool {
    let n = points.len();
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = points[i];
        let (xj, yj) = points[j];
        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi + 1e-12) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Returns the convex hull of the points using Graham scan.
pub fn polygon_hull(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut pts = points.to_vec();
    if pts.len() < 3 {
        return pts;
    }
    pts.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap()
            .then(a.1.partial_cmp(&b.1).unwrap())
    });
    let mut lower = Vec::new();
    for &p in &pts {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}
fn cross(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

#[cfg(test)]
mod tests;
