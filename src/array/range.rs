pub fn range(start: f64, stop: f64, step: f64) -> Vec<f64> {
    let mut result = Vec::new();
    let mut current = start;

    if step == 0.0 {
        if start == stop {
            return vec![start];
        } else {
            return Vec::new();
        }
    }

    if step > 0.0 {
        while current < stop {
            result.push(current);
            current += step;
        }
    } else {
        // step < 0.0
        while current > stop {
            result.push(current);
            current += step;
        }
    }
    result
}
