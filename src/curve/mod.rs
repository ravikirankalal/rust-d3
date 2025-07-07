//! D3 Curve module
//! Curve generators, e.g., curveBasis, curveCardinal, etc.

/// D3.js-like cubic basis spline generator (curveBasis).
pub struct CurveBasis;

impl CurveBasis {
    /// Generate a cubic basis spline for a sequence of points.
    /// Returns a Vec<(f64, f64)> of interpolated points.
    pub fn generate(points: &[(f64, f64)], resolution: usize) -> Vec<(f64, f64)> {
        if points.len() < 2 {
            return points.to_vec();
        }
        // D3.js basis: triplicate endpoints
        let n = points.len();
        let mut pts = Vec::with_capacity(n + 4);
        pts.push(points[0]);
        pts.push(points[0]);
        pts.extend_from_slice(points);
        pts.push(points[n - 1]);
        pts.push(points[n - 1]);
        let mut result = Vec::with_capacity(resolution + 1);
        for i in 0..=resolution {
            let t = i as f64 / resolution as f64;
            // Map t in [0,1] to segment
            let seg = (t * (n - 1) as f64).floor() as usize;
            let local_t = t * (n - 1) as f64 - seg as f64;
            let j = seg + 1;
            let (p0, p1, p2, p3) = (pts[j - 1], pts[j], pts[j + 1], pts[j + 2]);
            let x = basis_interp(p0.0, p1.0, p2.0, p3.0, local_t);
            let y = basis_interp(p0.1, p1.1, p2.1, p3.1, local_t);
            result.push((x, y));
        }
        result
    }
}

fn basis_interp(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    (
        (1.0 - 3.0 * t + 3.0 * t2 - t3) * p0
        + (4.0 - 6.0 * t2 + 3.0 * t3) * p1
        + (1.0 + 3.0 * t + 3.0 * t2 - 3.0 * t3) * p2
        + t3 * p3
    ) / 6.0
}
