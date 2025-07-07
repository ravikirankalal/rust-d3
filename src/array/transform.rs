pub fn transform<T, U, F>(array: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    array.iter().map(f).collect()
}
