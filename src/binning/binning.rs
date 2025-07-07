pub fn histogram<T: Into<f64> + Copy>(data: &[T], bins: usize) -> Vec<Vec<T>> {
    if data.is_empty() || bins == 0 {
        return vec![];
    }
    let min = data.iter().map(|&x| x.into()).fold(f64::INFINITY, f64::min);
    let max = data.iter().map(|&x| x.into()).fold(f64::NEG_INFINITY, f64::max);
    let bin_width = (max - min) / bins as f64;
    let mut result = vec![vec![]; bins];
    for &value in data {
        let idx = if bin_width == 0.0 {
            0
        } else {
            ((value.into() - min) / bin_width).floor() as usize
        };
        let idx = idx.min(bins - 1);
        result[idx].push(value);
    }
    result
}

/// Placeholder for d3.bin, d3.histogram, and related binning functions.
/// See: https://github.com/d3/d3-array#binning
/// TODO: Implement full API parity with d3-array binning functions.
pub fn bin<T: Into<f64> + PartialOrd + Copy>(values: &[T]) -> Vec<Vec<&T>> {
    if values.is_empty() {
        return vec![];
    }
    let mut refs: Vec<&T> = values.iter().collect();
    refs.sort_by(|a, b| {
        let a_f: f64 = (**a).into();
        let b_f: f64 = (**b).into();
        a_f.partial_cmp(&b_f).unwrap()
    });
    let min = refs.first().map(|x| (**x).into()).unwrap();
    let max = refs.last().map(|x| (**x).into()).unwrap();
    let bin_count = 10; // D3 default
    let bin_width = (max - min) / bin_count as f64;
    let mut bins: Vec<Vec<&T>> = vec![vec![]; bin_count];
    for &v in &refs {
        let v_f: f64 = (*v).into();
        let idx = if bin_width == 0.0 {
            0
        } else {
            ((v_f - min) / bin_width).floor() as usize
        };
        let idx = idx.min(bin_count - 1);
        bins[idx].push(v);
    }
    bins
}

/// Flexible histogram with custom bin edges and accessor.
pub fn histogram_with<'a, T, F>(data: &'a [T], bin_edges: &[f64], accessor: F) -> Vec<Vec<&'a T>>
where
    F: Fn(&T) -> f64,
{
    if data.is_empty() || bin_edges.len() < 2 {
        return vec![];
    }
    let mut bins = vec![vec![]; bin_edges.len() - 1];
    for item in data {
        let value = accessor(item);
        for i in 0..bin_edges.len() - 1 {
            let is_last = i == bin_edges.len() - 2;
            if (value >= bin_edges[i] && value < bin_edges[i + 1]) || (is_last && value == bin_edges[i + 1]) {
                bins[i].push(item);
                break;
            }
        }
    }
    bins
}

/// Flexible bin function: custom bin count, thresholds, and accessor.
pub fn bin_with<'a, T, F>(values: &'a [T], bin_count: usize, accessor: F) -> Vec<Vec<&'a T>>
where
    F: Fn(&T) -> f64,
{
    if values.is_empty() || bin_count == 0 {
        return vec![];
    }
    let min = values.iter().map(|x| accessor(x)).fold(f64::INFINITY, f64::min);
    let max = values.iter().map(|x| accessor(x)).fold(f64::NEG_INFINITY, f64::max);
    let bin_width = (max - min) / bin_count as f64;
    let mut bins = vec![vec![]; bin_count];
    for item in values {
        let value = accessor(item);
        let idx = if bin_width == 0.0 {
            0
        } else {
            ((value - min) / bin_width).floor() as usize
        };
        let idx = idx.min(bin_count - 1);
        bins[idx].push(item);
    }
    bins
}
