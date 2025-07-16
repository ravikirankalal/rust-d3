pub fn mean(array: &[f64]) -> Option<f64> {
    if array.is_empty() {
        return None;
    }

    let sum: f64 = array.iter().sum();
    Some(sum / array.len() as f64)
}
