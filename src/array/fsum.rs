pub fn fsum(array: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0; // A running compensation for lost low-order bits.

    for &x in array {
        let y = x - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}
