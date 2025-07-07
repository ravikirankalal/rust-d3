pub fn quantile(arr: &[f64], p: f64) -> Option<f64> {
    if arr.is_empty() {
        return None;
    }
    if !(0.0..=1.0).contains(&p) {
        return None; // Or panic, depending on desired error handling for invalid p
    }

    let n = arr.len();
    let index = (p * (n as f64 - 1.0));
    let index_floor = index.floor() as usize;
    let index_ceil = index.ceil() as usize;

    if index_floor == index_ceil {
        Some(arr[index_floor])
    } else {
        let v0 = arr[index_floor];
        let v1 = arr[index_ceil];
        Some(v0 + (v1 - v0) * (index - index_floor as f64))
    }
}
