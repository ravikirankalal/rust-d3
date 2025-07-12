use std::collections::HashSet;

pub fn union<T: Clone + Eq + std::hash::Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let mut set: HashSet<T> = a.iter().cloned().collect();
    set.extend(b.iter().cloned());
    set.into_iter().collect()
}
