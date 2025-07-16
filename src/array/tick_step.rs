use crate::array::ticks::tick_increment;

/// Calculates the step size for ticks between start and stop values.
/// 
/// This function determines the appropriate step size for generating ticks
/// based on the range and desired count. It handles domain reversal (when stop < start)
/// and converts negative increments to positive step values.
///
/// # Arguments
/// * `start` - The starting value of the range
/// * `stop` - The ending value of the range  
/// * `count` - The desired number of ticks
///
/// # Returns
/// The step size for generating evenly spaced ticks
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
