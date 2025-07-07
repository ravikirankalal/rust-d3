pub fn tick_step(start: f64, stop: f64, count: usize) -> f64 {
    let span = stop - start;
    let step = (span / count as f64).abs();
    let power = 10.0_f64.powf(step.log10().floor());
    let error = step / power;

    if error >= 0.92 {
        power * 10.0
    } else if error >= 0.42 {
        power * 5.0
    } else if error >= 0.12 {
        power * 2.0
    } else {
        power
    }
}
