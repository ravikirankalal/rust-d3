pub fn merge<T: Clone>(arrays: &[Vec<T>]) -> Vec<T> {
    arrays.iter().flatten().cloned().collect()
}
