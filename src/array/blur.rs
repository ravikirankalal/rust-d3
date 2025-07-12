// Port of d3-array's blur function to Rust
// https://github.com/d3/d3-array/blob/main/src/blur.js

pub fn blur1d(input: &[f64], radius: usize) -> Vec<f64> {
    if radius == 0 || input.is_empty() {
        return input.to_vec();
    }
    let n = input.len();
    let mut output = vec![0.0; n];
    for i in 0..n {
        let start = i.saturating_sub(radius);
        let end = (i + radius + 1).min(n);
        let window = &input[start..end];
        let sum: f64 = window.iter().sum();
        output[i] = sum / window.len() as f64;
    }
    output
}

// 2D blur (optional, for parity)
pub fn blur2d(input: &[Vec<f64>], radius: usize) -> Vec<Vec<f64>> {
    input.iter().map(|row| blur1d(row, radius)).collect()
}
