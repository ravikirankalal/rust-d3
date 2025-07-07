//! D3 Transition Advanced module
//! Advanced transition utilities for D3.js API parity.

/// D3.js: d3.transition().tween
pub fn transition_tween<F>(from: f64, to: f64, duration: u64, mut f: F)
where
    F: FnMut(f64),
{
    let steps = duration as usize / 16;
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let value = from + (to - from) * t;
        f(value);
    }
}

/// D3.js: d3.transition().filter
/// Filters a vector of items using a predicate, returning a new Vec.
pub fn transition_filter<T, F>(items: &[T], mut pred: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    items.iter().cloned().filter(|x| pred(x)).collect()
}

/// D3.js: Advanced transition utilities (composable trait)
pub trait TransitionAdvanced<T> {
    fn chain(self, next: impl FnOnce(Vec<T>) -> Vec<T>) -> Vec<T>;
}

impl<T> TransitionAdvanced<T> for Vec<T> {
    fn chain(self, next: impl FnOnce(Vec<T>) -> Vec<T>) -> Vec<T> {
        next(self)
    }
}
