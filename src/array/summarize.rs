pub fn summarize(array: &[f64]) -> Option<(f64, f64, f64, f64, f64)> {
    if array.is_empty() {
        return None;
    }
    let min = array.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = array.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = array.iter().sum::<f64>() / array.len() as f64;
    let median = {
        let mut v = array.to_vec();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = v.len() / 2;
        if v.len() % 2 == 0 {
            (v[mid - 1] + v[mid]) / 2.0
        } else {
            v[mid]
        }
    };
    let variance = array.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (array.len() as f64);
    Some((min, max, mean, median, variance))
}
