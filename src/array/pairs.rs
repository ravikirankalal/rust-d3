pub fn pairs<T: Clone>(array: &[T]) -> Vec<(T, T)> {
    let mut result = Vec::new();
    if array.len() < 2 {
        return result;
    }
    for i in 0..array.len() - 1 {
        result.push((array[i].clone(), array[i + 1].clone()));
    }
    result
}
