//! D3 Stack Advanced module: stack offset strategies for stacked charts

/// No offset: baseline at zero (default D3 behavior)
pub fn stack_offset_none(_series: &mut [Vec<(f64, f64)>]) {
    // No-op: already baseline at zero
}

/// Expand offset: normalize values to sum to 1 at each x
pub fn stack_offset_expand(series: &mut [Vec<(f64, f64)>]) {
    if series.is_empty() { return; }
    let n = series[0].len();
    for i in 0..n {
        let sum: f64 = series.iter().map(|s| s[i].1 - s[i].0).sum();
        if sum != 0.0 {
            let mut acc = 0.0;
            for s in series.iter_mut() {
                let v = s[i].1 - s[i].0;
                s[i].0 = acc / sum;
                acc += v;
                s[i].1 = acc / sum;
            }
        } else {
            for s in series.iter_mut() {
                s[i].0 = 0.0;
                s[i].1 = 0.0;
            }
        }
    }
}

/// Silhouette offset: center the streamgraph vertically
pub fn stack_offset_silhouette(series: &mut [Vec<(f64, f64)>]) {
    if series.is_empty() { return; }
    let n = series[0].len();
    for i in 0..n {
        let sum: f64 = series.iter().map(|s| s[i].1 - s[i].0).sum();
        let offset = -sum / 2.0;
        for s in series.iter_mut() {
            s[i].0 += offset;
            s[i].1 += offset;
        }
    }
}

/// Wiggle offset: minimize weighted change in baseline (streamgraph)
pub fn stack_offset_wiggle(series: &mut [Vec<(f64, f64)>]) {
    if series.is_empty() { return; }
    let n = series[0].len();
    let m = series.len();
    let mut y0 = vec![0.0; n];
    for i in 1..n {
        let mut s1 = 0.0;
        let mut s2 = 0.0;
        for k in 0..m {
            let y = series[k][i].1 - series[k][i].0;
            s1 += y * (k as f64);
            s2 += y;
        }
        if s2 != 0.0 {
            y0[i] = y0[i-1] - s1 / s2;
        } else {
            y0[i] = y0[i-1];
        }
    }
    for i in 0..n {
        let offset = y0[i];
        for s in series.iter_mut() {
            s[i].0 += offset;
            s[i].1 += offset;
        }
    }
}

/// Appearance order: earliest series (by max value) at the bottom
pub fn stack_order_appearance(series: &[Vec<(f64, f64)>]) -> Vec<usize> {
    let mut indices: Vec<_> = (0..series.len()).collect();
    indices.sort_by(|&a, &b| {
        let max_a = series[a].iter().map(|&(a, b)| b - a).fold(f64::NEG_INFINITY, f64::max);
        let max_b = series[b].iter().map(|&(a, b)| b - a).fold(f64::NEG_INFINITY, f64::max);
        max_a.partial_cmp(&max_b).unwrap_or(std::cmp::Ordering::Equal)
    });
    indices
}

/// Ascending order: smallest sum at the bottom
pub fn stack_order_ascending(series: &[Vec<(f64, f64)>]) -> Vec<usize> {
    let mut indices: Vec<_> = (0..series.len()).collect();
    indices.sort_by(|&a, &b| {
        let sum_a: f64 = series[a].iter().map(|&(a, b)| b - a).sum();
        let sum_b: f64 = series[b].iter().map(|&(a, b)| b - a).sum();
        sum_a.partial_cmp(&sum_b).unwrap_or(std::cmp::Ordering::Equal)
    });
    indices
}

/// Descending order: largest sum at the bottom
pub fn stack_order_descending(series: &[Vec<(f64, f64)>]) -> Vec<usize> {
    let mut indices: Vec<_> = (0..series.len()).collect();
    indices.sort_by(|&a, &b| {
        let sum_a: f64 = series[a].iter().map(|&(a, b)| b - a).sum();
        let sum_b: f64 = series[b].iter().map(|&(a, b)| b - a).sum();
        sum_b.partial_cmp(&sum_a).unwrap_or(std::cmp::Ordering::Equal)
    });
    indices
}

/// Inside-out order: for streamgraphs (see D3)
pub fn stack_order_inside_out(series: &[Vec<(f64, f64)>]) -> Vec<usize> {
    let sums: Vec<_> = series.iter().map(|s| s.iter().map(|&(a, b)| b - a).sum::<f64>()).collect();
    let mut indices: Vec<_> = (0..series.len()).collect();
    indices.sort_by(|&a, &b| sums[a].partial_cmp(&sums[b]).unwrap_or(std::cmp::Ordering::Equal));
    let mut inside_out = Vec::with_capacity(indices.len());
    let mut left = 0;
    let mut right = indices.len() as isize - 1;
    for (i, _) in indices.iter().enumerate() {
        if i % 2 == 0 {
            inside_out.push(indices[left]);
            left += 1;
        } else {
            inside_out.push(indices[right as usize]);
            right -= 1;
        }
    }
    inside_out
}

/// None order: as given
pub fn stack_order_none(series: &[Vec<(f64, f64)>]) -> Vec<usize> {
    (0..series.len()).collect()
}

/// Reverse order: reverse of given
pub fn stack_order_reverse(series: &[Vec<(f64, f64)>]) -> Vec<usize> {
    (0..series.len()).rev().collect()
}

/// Diverging offset: positive above zero, negative below zero
pub fn stack_offset_diverging(series: &mut [Vec<(f64, f64)>]) {
    if series.is_empty() { return; }
    let n = series[0].len();
    let m = series.len();
    for i in 0..n {
        let mut pos = 0.0;
        let mut neg = 0.0;
        for k in 0..m {
            let v = series[k][i].1 - series[k][i].0;
            if v >= 0.0 {
                series[k][i].0 = pos;
                pos += v;
                series[k][i].1 = pos;
            } else {
                series[k][i].0 = neg;
                neg += v;
                series[k][i].1 = neg;
            }
        }
    }
}
