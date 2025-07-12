pub fn median(array: &[f64]) -> Option<f64> {
    if array.is_empty() {
        return None;
    }

    let mut sorted_array = array.to_vec();
    sorted_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = sorted_array.len() / 2;
    if sorted_array.len() % 2 == 0 {
        // Even number of elements
        Some((sorted_array[mid - 1] + sorted_array[mid]) / 2.0)
    } else {
        // Odd number of elements
        Some(sorted_array[mid])
    }
}
