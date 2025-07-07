// Area and arc generators implementation

pub fn area<T, F>(data: &[T], mut accessor: F) -> Vec<(f64, f64)>
where
    F: FnMut(&T) -> (f64, f64),
{
    data.iter().map(|d| accessor(d)).collect()
}

pub fn arc(_inner_radius: f64, outer_radius: f64, start_angle: f64, end_angle: f64) -> Vec<(f64, f64)> {
    // Placeholder: returns points on the arc
    let n = 10;
    (0..=n)
        .map(|i| {
            let t = start_angle + (end_angle - start_angle) * i as f64 / n as f64;
            (
                outer_radius * t.cos(),
                outer_radius * t.sin(),
            )
        })
        .collect()
}
