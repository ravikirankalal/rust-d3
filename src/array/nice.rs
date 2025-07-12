use super::tick_step::tick_step;

pub fn nice(mut start: f64, mut stop: f64, count: usize) -> (f64, f64) {
    if start == stop {
        return (start, stop);
    }

    let reverse = start > stop;
    if reverse {
        std::mem::swap(&mut start, &mut stop);
    }

    let step = tick_step(start, stop, count);
    let nice_start = (start / step).floor() * step;
    let nice_stop = (stop / step).ceil() * step;

    if reverse {
        (nice_stop, nice_start)
    } else {
        (nice_start, nice_stop)
    }
}
