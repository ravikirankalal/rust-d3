use crate::array::ticks::tick_increment;

pub fn tick_step(start: f64, stop: f64, count: usize) -> f64 {
    let reverse = stop < start;
    let inc = if reverse {
        tick_increment(stop, start, count)
    } else {
        tick_increment(start, stop, count)
    };
    
    let step = if reverse { -1.0 } else { 1.0 } * if inc < 0.0 { 1.0 / -inc } else { inc };
    step
}
