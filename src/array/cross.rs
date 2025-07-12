pub fn cross<T: Clone, U: Clone>(a: &[T], b: &[U]) -> Vec<(T, U)> {
    let mut result = Vec::new();
    for item_a in a {
        for item_b in b {
            result.push((item_a.clone(), item_b.clone()));
        }
    }
    result
}
