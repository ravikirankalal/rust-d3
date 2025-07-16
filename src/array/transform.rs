// Generic transformation function that applies a given function to each element
// in an array and collects the results.
pub fn transform<T, U, F>(array: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    array.iter().map(f).collect()
}
