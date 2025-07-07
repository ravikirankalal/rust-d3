use super::tick_step::tick_step;

pub fn ticks(start: f64, stop: f64, count: usize) -> Vec<f64> {
    let mut reverse = false;
    let mut s = start;
    let mut e = stop;
    if s > e {
        std::mem::swap(&mut s, &mut e);
        reverse = true;
    }

    let step = tick_step(s, e, count);
    let mut i = (s / step).ceil();
    let mut j = (e / step).floor();
    let mut result = Vec::new();

    if i > j {
        return result;
    }

    while i <= j {
        result.push(i * step);
        i += 1.0;
    }

    if reverse {
        result.reverse();
    }
    result
}
