use std::cmp::Ordering;

pub fn sort_by<T: Clone, F: Fn(&T, &T) -> Ordering>(array: &[T], cmp: F) -> Vec<T> {
    let mut v = array.to_vec();
    v.sort_by(cmp);
    v
}
