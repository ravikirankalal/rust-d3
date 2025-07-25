use std::collections::HashSet;

pub fn symmetric_difference<T: Clone + Eq + std::hash::Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let set_a: HashSet<_> = a.iter().cloned().collect();
    let set_b: HashSet<_> = b.iter().cloned().collect();
    set_a.symmetric_difference(&set_b).cloned().collect()
}
