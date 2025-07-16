pub fn sum(array: &[f64]) -> Option<f64> {
    if array.is_empty() {
        return None;
    }
    Some(array.iter().sum())
}
