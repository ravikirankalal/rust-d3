// Constants for D3-compatible tick generation
const E10: f64 = 7.0710678118654752440084436210485; // Math.sqrt(50) in D3
const E5: f64 = 3.1622776601683793319988935444327; // Math.sqrt(10) in D3  
const E2: f64 = 1.4142135623730950488016887242097; // Math.sqrt(2) in D3

// D3-compatible tick specification function
// Calculates optimal ticks within a given start and stop range by determining factor and increment
// Used primarily to generate human-readable ticks based on D3-like specifications for better visualization
fn tick_spec(start: f64, stop: f64, count: usize) -> (i32, i32, f64) {
    let count = count.max(1) as f64;
    let step = (stop - start) / count;
    let power = step.log10().floor();
    let error = step / 10.0_f64.powf(power);
    
    let factor = if error >= E10 {
        10.0
    } else if error >= E5 {
        5.0
    } else if error >= E2 {
        2.0
    } else {
        1.0
    };
    
    let (i1, i2, inc) = if power < 0.0 {
        let inc = 10.0_f64.powf(-power) / factor;
        let i1 = (start * inc).round() as i32;
        let i2 = (stop * inc).round() as i32;
        let i1 = if (i1 as f64) / inc < start { i1 + 1 } else { i1 };
        let i2 = if (i2 as f64) / inc > stop { i2 - 1 } else { i2 };
        (i1, i2, -inc)
    } else {
        let inc = 10.0_f64.powf(power) * factor;
        let i1 = (start / inc).round() as i32;
        let i2 = (stop / inc).round() as i32;
        let i1 = if (i1 as f64) * inc < start { i1 + 1 } else { i1 };
        let i2 = if (i2 as f64) * inc > stop { i2 - 1 } else { i2 };
        (i1, i2, inc)
    };
    
    // Handle the case where we have too few ticks
    if i2 < i1 && count >= 0.5 && count < 2.0 {
        return tick_spec(start, stop, (count * 2.0) as usize);
    }
    
    (i1, i2, inc)
}

/// Generates evenly spaced ticks between start and stop values.
///
/// This function creates a vector of tick values that are nicely spaced for
/// visualization purposes. It uses D3-compatible logic with predefined constants
/// (E10, E5, E2) to determine appropriate spacing factors.
///
/// # Arguments
/// * `start` - The starting value of the range
/// * `stop` - The ending value of the range
/// * `count` - The desired number of ticks (actual count may vary)
///
/// # Returns
/// A vector of f64 values representing tick positions
///
/// # Examples
/// ```
/// let tick_values = ticks(0.0, 100.0, 5);
/// // Returns approximately [0, 20, 40, 60, 80, 100]
/// ```
///
/// # Edge Cases
/// - Returns empty vector if count is 0
/// - Returns single value if start == stop
/// - Returns empty vector for NaN inputs
/// - Handles reversed domains (stop < start)
pub fn ticks(start: f64, stop: f64, count: usize) -> Vec<f64> {
    // Handle edge cases
    if count == 0 {
        return vec![];
    }
    
    // If domain is zero-span, return single value
    if start == stop {
        return vec![start];
    }
    
    // Handle NaN inputs
    if start.is_nan() || stop.is_nan() {
        return vec![];
    }
    
    let reverse = stop < start;
    let (spec_start, spec_stop) = if reverse {
        (stop, start)
    } else {
        (start, stop)
    };
    
    let (i1, i2, inc) = tick_spec(spec_start, spec_stop, count);
    
    // If no valid ticks, return empty
    if i2 < i1 {
        return vec![];
    }
    
    let n = (i2 - i1 + 1) as usize;
    let mut ticks = Vec::with_capacity(n);
    
    if reverse {
        if inc < 0.0 {
            for i in 0..n {
                ticks.push((i2 - i as i32) as f64 / -inc);
            }
        } else {
            for i in 0..n {
                ticks.push((i2 - i as i32) as f64 * inc);
            }
        }
    } else {
        if inc < 0.0 {
            for i in 0..n {
                ticks.push((i1 + i as i32) as f64 / -inc);
            }
        } else {
            for i in 0..n {
                ticks.push((i1 + i as i32) as f64 * inc);
            }
        }
    }
    
    // Ensure we have at least one tick if we should
    if ticks.is_empty() && count > 0 {
        ticks.push(start);
    }
    
    ticks
}

pub fn tick_increment(start: f64, stop: f64, count: usize) -> f64 {
    let (_, _, inc) = tick_spec(start, stop, count);
    inc
}
