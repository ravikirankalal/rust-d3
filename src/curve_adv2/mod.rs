//! D3 Curve Advanced 2 module
//! Additional curve generators: Catmull-Rom, Cardinal, etc.

/// Generate a Catmull-Rom spline through the given points.
pub fn curve_catmull_rom(points: &[(f64, f64)], alpha: f64, samples: usize) -> Vec<(f64, f64)> {
    if points.len() < 2 || samples < 2 {
        return points.to_vec();
    }
    let mut out = Vec::with_capacity(samples * (points.len() - 1) + 1);
    out.push(points[0]); // D3: first output point is first input point
    let n = points.len();
    for i in 0..n - 1 {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < n { points[i + 2] } else { points[n - 1] };
        for j in 1..samples { // start at 1 to avoid duplicating p1
            let t = j as f64 / (samples as f64);
            let t0 = 0.0;
            let t1 = t0 + ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powf(2.0)).powf(alpha * 0.5);
            let t2 = t1 + ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powf(2.0)).powf(alpha * 0.5);
            let t3 = t2 + ((p3.0 - p2.0).powi(2) + (p3.1 - p2.1).powf(2.0)).powf(alpha * 0.5);
            let tt = t1 + t * (t2 - t1);
            let a1 = (
                (t1 - tt) / (t1 - t0) * p0.0 + (tt - t0) / (t1 - t0) * p1.0,
                (t1 - tt) / (t1 - t0) * p0.1 + (tt - t0) / (t1 - t0) * p1.1,
            );
            let a2 = (
                (t2 - tt) / (t2 - t1) * p1.0 + (tt - t1) / (t2 - t1) * p2.0,
                (t2 - tt) / (t2 - t1) * p1.1 + (tt - t1) / (t2 - t1) * p2.1,
            );
            let a3 = (
                (t3 - tt) / (t3 - t2) * p2.0 + (tt - t2) / (t3 - t2) * p3.0,
                (t3 - tt) / (t3 - t2) * p2.1 + (tt - t2) / (t3 - t2) * p3.1,
            );
            let b1 = (
                (t2 - tt) / (t2 - t0) * a1.0 + (tt - t0) / (t2 - t0) * a2.0,
                (t2 - tt) / (t2 - t0) * a1.1 + (tt - t0) / (t2 - t0) * a2.1,
            );
            let b2 = (
                (t3 - tt) / (t3 - t1) * a2.0 + (tt - t1) / (t3 - t1) * a3.0,
                (t3 - tt) / (t3 - t1) * a2.1 + (tt - t1) / (t3 - t1) * a3.1,
            );
            let c = (
                (t2 - tt) / (t2 - t1) * b1.0 + (tt - t1) / (t2 - t1) * b2.0,
                (t2 - tt) / (t2 - t1) * b1.1 + (tt - t1) / (t2 - t1) * b2.1,
            );
            out.push(c);
        }
    }
    if *out.last().unwrap() != points[n - 1] {
        out.push(points[n - 1]); // D3: last output point is last input point
    }
    out
}

/// Generate a Cardinal spline through the given points (D3.js: d3.curveCardinal)
pub fn curve_cardinal(points: &[(f64, f64)], tension: f64, samples: usize) -> Vec<(f64, f64)> {
    if points.len() < 2 || samples < 2 {
        return points.to_vec();
    }
    let mut out = Vec::with_capacity(samples * (points.len() - 1));
    let n = points.len();
    let t = (1.0 - tension) / 2.0;
    for i in 0..n - 1 {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < n { points[i + 2] } else { points[n - 1] };
        for j in 0..samples {
            let s = j as f64 / (samples as f64);
            let s2 = s * s;
            let s3 = s2 * s;
            let a = -t * s3 + 2.0 * t * s2 - t * s;
            let b = (2.0 - t) * s3 + (t - 3.0) * s2 + 1.0;
            let c = (t - 2.0) * s3 + (3.0 - 2.0 * t) * s2 + t * s;
            let d = t * s3 - t * s2;
            let x = a * p0.0 + b * p1.0 + c * p2.0 + d * p3.0;
            let y = a * p0.1 + b * p1.1 + c * p2.1 + d * p3.1;
            out.push((x, y));
        }
    }
    out
}
