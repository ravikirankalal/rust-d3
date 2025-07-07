pub fn deviation(array: &[f64]) -> Option<f64> {
    let n = array.len();
    if n < 2 {
        return None;
    }

    let mean_val: f64 = array.iter().sum::<f64>() / n as f64;

    let variance = array.iter()
        .map(|&x| (x - mean_val).powi(2))
        .sum::<f64>() / (n - 1) as f64;

    Some(variance.sqrt())
}
