pub fn permute<T: Clone>(array: &[T], indexes: &[usize]) -> Vec<T> {
    indexes.iter().map(|&i| array[i].clone()).collect()
}
