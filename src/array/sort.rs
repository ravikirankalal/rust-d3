pub fn sort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let mut v = array.to_vec();
    v.sort();
    v
}
