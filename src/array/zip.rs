pub fn zip<T: Clone>(arrays: &[Vec<T>]) -> Vec<Vec<T>> {
    if arrays.is_empty() {
        return Vec::new();
    }

    let min_len = arrays.iter().map(|arr| arr.len()).min().unwrap_or(0);
    if min_len == 0 {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(min_len);
    for i in 0..min_len {
        let mut row = Vec::with_capacity(arrays.len());
        for array in arrays {
            row.push(array[i].clone());
        }
        result.push(row);
    }
    result
}
