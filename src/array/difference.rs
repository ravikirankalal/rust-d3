use std::collections::HashSet;

pub fn difference<T: Clone + Eq + std::hash::Hash + std::fmt::Debug>(a: &[T], b: &[T]) -> Vec<T> {
    println!("Input a: {:?}", a);
    println!("Input b: {:?}", b);
    let set_a: HashSet<_> = a.iter().cloned().collect();
    let set_b: HashSet<_> = b.iter().cloned().collect();
    println!("Set a: {:?}", set_a);
    println!("Set b: {:?}", set_b);
    let diff: Vec<T> = set_a.difference(&set_b).cloned().collect();
    println!("Difference (a - b): {:?}", diff);
    diff
}
