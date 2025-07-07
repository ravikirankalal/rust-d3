//! D3 Bundle Advanced module
//! Implements curveBundle with configurable beta (bundle strength).

/// D3.js-like curveBundle generator.
/// For beta=0, returns straight line; for beta=1, returns basis spline; for 0<beta<1, interpolates.
pub struct CurveBundle {
    pub beta: f64, // bundle strength in [0, 1]
}

impl CurveBundle {
    pub fn new(beta: f64) -> Self {
        Self { beta: beta.clamp(0.0, 1.0) }
    }

    /// Generate a bundle curve for a sequence of points.
    /// Returns a Vec<(f64, f64)> of interpolated points.
    pub fn generate(&self, points: &[(f64, f64)], resolution: usize) -> Vec<(f64, f64)> {
        if points.len() < 2 {
            return points.to_vec();
        }
        if self.beta <= 0.0 {
            // Straight line between endpoints
            let (x0, y0) = points.first().unwrap();
            let (x1, y1) = points.last().unwrap();
            return (0..=resolution)
                .map(|i| {
                    let t = i as f64 / resolution as f64;
                    (x0 + (x1 - x0) * t, y0 + (y1 - y0) * t)
                })
                .collect();
        } else if self.beta >= 1.0 {
            // Standard basis spline
            return basis_spline(points, resolution);
        } else {
            // Interpolate between straight line and basis spline
            let line: Vec<(f64, f64)> = {
                let (x0, y0) = points.first().unwrap();
                let (x1, y1) = points.last().unwrap();
                (0..=resolution)
                    .map(|i| {
                        let t = i as f64 / resolution as f64;
                        (x0 + (x1 - x0) * t, y0 + (y1 - y0) * t)
                    })
                    .collect()
            };
            let basis = basis_spline(points, resolution);
            line.iter()
                .zip(basis.iter())
                .map(|(&(lx, ly), &(bx, by))| {
                    let b = self.beta;
                    (lx * (1.0 - b) + bx * b, ly * (1.0 - b) + by * b)
                })
                .collect()
        }
    }
}

/// Cubic basis spline interpolation for a sequence of points.
fn basis_spline(points: &[(f64, f64)], resolution: usize) -> Vec<(f64, f64)> {
    // D3.js basis: triplicate endpoints
    let n = points.len();
    if n < 2 {
        return points.to_vec();
    }
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
