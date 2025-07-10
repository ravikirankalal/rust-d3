// src/contour/marching_squares.rs

use std::collections::HashMap;

pub const CASES: &[&[&[[f64; 2]; 2]]; 16] = &[
    &[],
    &[&[[0.5, 1.0], [0.0, 0.5]]],
    &[&[[1.0, 0.5], [0.5, 1.0]]],
    &[&[[1.0, 0.5], [0.0, 0.5]]],
    &[&[[0.0, 0.5], [0.5, 0.0]]],
    &[&[[0.5, 1.0], [0.5, 0.0]], &[[1.0, 0.5], [0.0, 0.5]]],
    &[&[[1.0, 0.5], [0.5, 0.0]]],
    &[&[[1.0, 0.5], [0.0, 0.5]]],
    &[&[[0.0, 0.5], [0.5, 1.0]]],
    &[&[[0.5, 0.0], [0.5, 1.0]]],
    &[&[[0.0, 0.5], [1.0, 0.5]], &[[0.5, 0.0], [0.5, 1.0]]],
    &[&[[0.5, 0.0], [1.0, 0.5]]],
    &[&[[0.0, 0.5], [0.5, 0.0]]],
    &[&[[0.0, 0.5], [0.5, 1.0]]],
    &[&[[0.5, 1.0], [0.5, 0.0]]],
    &[],
];

#[allow(dead_code)]
fn index(point: &[f64; 2], dx: usize) -> usize {
    (point[0] * 2.0 + point[1] * (dx as f64 + 1.0) * 4.0) as usize
}

#[allow(dead_code)]
fn quant_key(point: &[f64; 2]) -> (i32, i32) {
    ((point[0] * 1_000_000.0).round() as i32, (point[1] * 1_000_000.0).round() as i32)
}

#[allow(dead_code)]
fn stitch<F>(
    fragment_by_start: &mut HashMap<(i32, i32), Vec<[f64; 2]>>,
    fragment_by_end: &mut HashMap<(i32, i32), Vec<[f64; 2]>>,
    start_point: [f64; 2],
    end_point: [f64; 2],
    _dx: usize,
    callback: &mut F,
) where
    F: FnMut(Vec<[f64; 2]>),
{
    let start_index = quant_key(&start_point);
    let end_index = quant_key(&end_point);

    let prepend = fragment_by_end.remove(&start_index);
    let append = fragment_by_start.remove(&end_index);

    match (prepend, append) {
        (Some(pre), Some(app)) => {
            let mut pre = pre;
            let app = app;
            // Join two fragments: pre + [end_point] + app (skip duplicate point)
            pre.push(end_point);
            pre.extend(app.into_iter().skip(1));
            if quant_key(&pre[0]) == quant_key(&pre[pre.len() - 1]) {
                callback(pre);
            } else {
                let new_start_index = quant_key(&pre[0]);
                let new_end_index = quant_key(&pre[pre.len() - 1]);
                fragment_by_start.insert(new_start_index, pre.clone());
                fragment_by_end.insert(new_end_index, pre);
            }
        }
        (Some(pre), None) => {
            let mut pre = pre;
            pre.push(end_point);
            let new_end_index = quant_key(&pre[pre.len() - 1]);
            if quant_key(&pre[0]) == quant_key(&pre[pre.len() - 1]) {
                callback(pre);
            } else {
                let new_start_index = quant_key(&pre[0]);
                fragment_by_start.insert(new_start_index, pre.clone());
                fragment_by_end.insert(new_end_index, pre);
            }
        }
        (None, Some(app)) => {
            let mut app = app;
            app.insert(0, start_point);
            let new_start_index = quant_key(&app[0]);
            let new_end_index = quant_key(&app[app.len() - 1]);
            if quant_key(&app[0]) == quant_key(&app[app.len() - 1]) {
                callback(app);
            } else {
                fragment_by_start.insert(new_start_index, app.clone());
                fragment_by_end.insert(new_end_index, app);
            }
        }
        (None, None) => {
            let frag = vec![start_point, end_point];
            if quant_key(&start_point) == quant_key(&end_point) {
                callback(frag);
            } else {
                fragment_by_start.insert(start_index, frag.clone());
                fragment_by_end.insert(end_index, frag);
            }
        }
    }
}

pub fn isorings<F>(values: &[f64], value: f64, dx: usize, dy: usize, mut callback: F)
where
    F: FnMut(Vec<[f64; 2]>),
{
    use std::collections::HashMap;
    #[derive(Debug)]
    struct Fragment {
        start: (i32, i32),
        end: (i32, i32),
        ring: Vec<[f64; 2]>,
    }
    let mut fragment_by_start: HashMap<(i32, i32), Fragment> = HashMap::new();
    let mut fragment_by_end: HashMap<(i32, i32), Fragment> = HashMap::new();
    if dx < 2 || dy < 2 {
        return;
    }
    for y in 0..(dy - 1) {
        for x in 0..(dx - 1) {
            let i = y * dx + x;
            let (t0, t1, t2, t3) = cell_mask(values, i, dx, value);
            let case = marching_case(t0, t1, t2, t3);
            for (start_point, end_point) in marching_segments(case, x, y) {
                let start_index = quant_key(&start_point);
                let end_index = quant_key(&end_point);
                let f = fragment_by_end.remove(&start_index);
                let g = fragment_by_start.remove(&end_index);
                if let Some(frag_f) = f {
                    let mut frag_f = frag_f;
                    if let Some(frag_g) = g {
                        let frag_g = frag_g;
                        // Join two fragments: f.ring + g.ring (skip duplicate point)
                        frag_f.ring.push(end_point);
                        frag_f.ring.extend(frag_g.ring.into_iter().skip(1));
                        // Remove old entries
                        fragment_by_end.remove(&frag_f.end);
                        fragment_by_start.remove(&frag_g.start);
                        let frag_f_last = quant_key(&frag_f.ring[frag_f.ring.len() - 1]);
                        if frag_f.start == frag_f_last {
                            callback(frag_f.ring);
                        } else {
                            let new_start = frag_f.start;
                            let new_end = frag_f_last;
                            let ring_clone = frag_f.ring.clone();
                            fragment_by_start.insert(new_start, Fragment { start: new_start, end: new_end, ring: frag_f.ring });
                            fragment_by_end.insert(new_end, Fragment { start: new_start, end: new_end, ring: ring_clone });
                        }
                    } else {
                        // Extend fragment at end
                        frag_f.ring.push(end_point);
                        fragment_by_end.remove(&frag_f.end);
                        let new_end = quant_key(&frag_f.ring[frag_f.ring.len() - 1]);
                        if frag_f.start == new_end {
                            callback(frag_f.ring);
                        } else {
                            fragment_by_end.insert(new_end, Fragment { start: frag_f.start, end: new_end, ring: frag_f.ring.clone() });
                            fragment_by_start.insert(frag_f.start, Fragment { start: frag_f.start, end: new_end, ring: frag_f.ring });
                        }
                    }
                } else if let Some(frag_g) = g {
                    let mut frag_g = frag_g;
                    // Extend fragment at start
                    frag_g.ring.insert(0, start_point);
                    fragment_by_start.remove(&frag_g.start);
                    let new_start = quant_key(&frag_g.ring[0]);
                    if new_start == frag_g.end {
                        callback(frag_g.ring);
                    } else {
                        fragment_by_start.insert(new_start, Fragment { start: new_start, end: frag_g.end, ring: frag_g.ring.clone() });
                        fragment_by_end.insert(frag_g.end, Fragment { start: new_start, end: frag_g.end, ring: frag_g.ring });
                    }
                } else {
                    // New fragment
                    let ring = vec![start_point, end_point];
                    let start = quant_key(&start_point);
                    let end = quant_key(&end_point);
                    if start == end {
                        callback(ring);
                    } else {
                        fragment_by_start.insert(start, Fragment { start, end, ring: ring.clone() });
                        fragment_by_end.insert(end, Fragment { start, end, ring });
                    }
                }
            }
        }
    }
    // Emit all closed rings (fragments where start == end)
    for (_k, frag) in fragment_by_start.drain() {
        if frag.ring.len() > 1 {
            callback(frag.ring);
        }
    }
}

/// Returns the marching squares case for a cell given the four corner values and threshold.
pub fn marching_case(t0: bool, t1: bool, t2: bool, t3: bool) -> usize {
    (t0 as usize) | ((t1 as usize) << 1) | ((t2 as usize) << 2) | ((t3 as usize) << 3)
}

/// Returns the interpolated segment endpoints for a given case and cell position.
pub fn marching_segments(case: usize, x: usize, y: usize) -> Vec<([f64; 2], [f64; 2])> {
    let mut segments = Vec::new();
    for line in CASES[case].iter() {
        let start_point = [line[0][0] + x as f64, line[0][1] + y as f64];
        let end_point = [line[1][0] + x as f64, line[1][1] + y as f64];
        segments.push((start_point, end_point));
    }
    segments
}

/// Returns the boolean mask for a cell's corners given the values and threshold.
pub fn cell_mask(values: &[f64], i: usize, dx: usize, value: f64) -> (bool, bool, bool, bool) {
    let t0 = values[i] >= value;
    let t1 = values[i + 1] >= value;
    let t2 = values[i + dx] >= value;
    let t3 = values[i + dx + 1] >= value;
    (t0, t1, t2, t3)
}
