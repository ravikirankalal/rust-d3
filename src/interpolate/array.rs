//! d3-interpolate: Array interpolation

/// Linear interpolation between two arrays (elementwise)
pub fn interpolate_array(a: &[f64], b: &[f64], t: f64) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x + (y - x) * t).collect()
}

/// Piecewise interpolation for arrays of arrays
pub fn interpolate_piecewise(arrays: &[Vec<f64>], t: f64) -> Vec<f64> {
    let n = arrays.len();
    if n == 0 { return vec![]; }
    let idx = ((n - 1) as f64 * t).floor() as usize;
    let next_idx = ((n - 1) as f64 * t).ceil() as usize;
    let local_t = (t * (n - 1) as f64) - idx as f64;
    interpolate_array(&arrays[idx], &arrays[next_idx], local_t)
}
