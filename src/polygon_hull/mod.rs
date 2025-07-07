// D3 polygon hull utility for Rust D3
// Computes the convex hull of a set of 2D points using Graham scan.

pub fn convex_hull(mut points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    if points.len() < 3 {
        return points;
    }
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut lower = Vec::new();
    for p in &points {
        while lower.len() >= 2 && cross(lower[lower.len()-2], lower[lower.len()-1], *p) <= 0.0 {
            lower.pop();
        }
        lower.push(*p);
    }
    let mut upper = Vec::new();
    for p in points.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len()-2], upper[upper.len()-1], *p) <= 0.0 {
            upper.pop();
        }
        upper.push(*p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn cross(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}
