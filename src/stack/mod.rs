// D3 stack module for Rust D3
// Provides a simple stack generator for stacked charts.

pub fn stack<T: Copy + Into<f64>>(series: &[Vec<T>]) -> Vec<Vec<(f64, f64)>> {
    if series.is_empty() {
        return vec![];
    }
    let n = series[0].len();
    let mut result = vec![vec![(0.0, 0.0); n]; series.len()];
    for i in 0..n {
        let mut acc = 0.0;
        for (j, s) in series.iter().enumerate() {
            let v = s[i].into();
            result[j][i] = (acc, acc + v);
            acc += v;
        }
    }
    result
}

/// Placeholder for d3-stack API parity.
/// See: https://github.com/d3/d3-shape#stacks
/// TODO: Implement full API parity with d3-stack (stackOrder, stackOffset, etc.)
pub fn stack_order_none<T>() {}

pub fn stack_offset_none<T>() {}

pub mod builder;
