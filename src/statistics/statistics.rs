pub fn mean<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    if data.is_empty() {
        None
    } else {
        Some(sum(data)? / data.len() as f64)
    }
}

pub fn sum<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    if data.is_empty() {
        None
    } else {
        Some(data.iter().map(|&x| x.into()).sum())
    }
}

pub fn median<T: Into<f64> + Copy + Ord>(data: &[T]) -> Option<f64> {
    let mut v: Vec<f64> = data.iter().map(|&x| x.into()).collect();
    let n = v.len();
    if n == 0 {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if n % 2 == 0 {
        Some((v[n / 2 - 1] + v[n / 2]) / 2.0)
    } else {
        Some(v[n / 2])
    }
}

pub fn variance<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    let n = data.len();
    if n < 2 {
        return None;
    }
    let mean = data.iter().map(|&x| x.into()).sum::<f64>() / n as f64;
    let mut sum_sq = 0.0;
    let mut sum = 0.0;
    for &x in data {
        let d = x.into() - mean;
        sum_sq += d * d;
        sum += d;
    }
    // Compensated two-pass: see Bessel's correction
    let variance = (sum_sq - (sum * sum) / n as f64) / (n as f64 - 1.0);
    Some(variance)
}

pub fn deviation<T: Into<f64> + Copy>(values: &[T]) -> Option<f64> {
    variance(values).map(|v| v.sqrt())
}

pub fn quantile<T: Into<f64> + Copy>(values: &[T], p: f64) -> Option<f64> {
    if values.is_empty() || !(0.0..=1.0).contains(&p) {
        return None;
    }
    let mut v: Vec<f64> = values.iter().map(|&x| x.into()).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n == 1 {
        return Some(v[0]);
    }
    let idx = p * (n as f64 - 1.0);
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    if lo == hi {
        Some(v[lo])
    } else {
        let h = idx - lo as f64;
        Some(v[lo] * (1.0 - h) + v[hi] * h)
    }
}

pub fn cumsum<T: Into<f64> + Copy>(values: &[T]) -> Vec<f64> {
    let mut result = Vec::with_capacity(values.len());
    let mut sum = 0.0;
    for &x in values {
        sum += x.into();
        result.push(sum);
    }
    result
}

pub fn scan<T: PartialOrd + Copy>(values: &[T]) -> Option<usize> {
    if values.is_empty() {
        return None;
    }
    let mut min_idx = 0;
    for (i, &v) in values.iter().enumerate().skip(1) {
        if v < values[min_idx] {
            min_idx = i;
        }
    }
    Some(min_idx)
}
