//! D3 Selection Advanced module
//! More selection utilities, etc.

/// Selects the nth element from a slice, if it exists.
pub fn select_nth<T>(slice: &[T], n: usize) -> Option<&T> {
    slice.get(n)
}
