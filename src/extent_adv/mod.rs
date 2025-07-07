//! D3 Extent Advanced module
//! Advanced extent utilities, e.g., multi-dimensional extent, etc.

/// Computes the min/max for each dimension in a 2D array.
pub fn extent_multi(data: &[Vec<f64>]) -> Option<Vec<(f64, f64)>> {
    if data.is_empty() { return None; }
    let dim = data[0].len();
    let mut min = vec![f64::INFINITY; dim];
    let mut max = vec![f64::NEG_INFINITY; dim];
    for row in data {
        for (i, &val) in row.iter().enumerate() {
            if val < min[i] { min[i] = val; }
            if val > max[i] { max[i] = val; }
        }
    }
    Some(min.into_iter().zip(max.into_iter()).collect())
}
