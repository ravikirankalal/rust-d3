pub fn ticks(start: f64, stop: f64, count: usize) -> Vec<f64> {
    if count == 0 {
        return vec![];
    }
    if start == stop {
        return vec![start];
    }
    let reverse = start > stop;
    let (s, e) = if reverse { (stop, start) } else { (start, stop) };
    // let step = tick_step(s, e, count); // unused
    let mut ticks = Vec::with_capacity(count + 1);
    // let mut v = s; // unused
    let n = count;
    for i in 0..=n {
        ticks.push(s + (e - s) * (i as f64) / (n as f64));
    }
    if reverse {
        ticks.reverse();
    }
    ticks
}
