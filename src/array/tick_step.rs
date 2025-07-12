pub fn tick_step(start: f64, stop: f64, count: usize) -> f64 {
    let span = stop - start;
    let intervals = if count > 1 { count - 1 } else { 1 } as f64;
    let raw_step = (span / intervals).abs();
    let power = 10.0_f64.powf(raw_step.log10().floor());
    let error = raw_step / power;
    let nice_step = if error >= 10.0 {
        power * 10.0
    } else if error >= 5.0 {
        power * 5.0
    } else if error >= 2.0 {
        power * 2.0
    } else {
        power
    };
    if span < 0.0 { -nice_step } else { nice_step }
}
