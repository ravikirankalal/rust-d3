pub fn variance(array: &[f64]) -> Option<f64> {
    let n = array.len();
    if n < 2 {
        return None;
    }

    let mean_val: f64 = array.iter().sum::<f64>() / n as f64;

    let sum_squared_diff: f64 = array.iter().map(|&x| (x - mean_val).powi(2)).sum();

    Some(sum_squared_diff / (n - 1) as f64)
}
