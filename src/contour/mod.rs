// src/contour/mod.rs

use crate::geojson::{GeoJsonFeature, GeoJsonGeometry};
use std::collections::HashMap;
use std::f64;

use crate::array::ascending::ascending;
use crate::array::extent::extent;
use crate::array::nice::nice;
use crate::array::ticks::ticks;
use serde_json;

#[derive(Debug, Clone, Copy)]
pub struct Point([f64; 2]);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0[0].to_bits() == other.0[0].to_bits() && self.0[1].to_bits() == other.0[1].to_bits()
    }
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0[0].to_bits().hash(state);
        self.0[1].to_bits().hash(state);
    }
}

impl From<[f64; 2]> for Point {
    fn from(arr: [f64; 2]) -> Self {
        Point(arr)
    }
}

impl From<Point> for [f64; 2] {
    fn from(point: Point) -> Self {
        point.0
    }
}

// Marching squares lookup table
// Each entry is a list of pairs of indices representing the edges to draw.
// The indices correspond to the midpoints of the edges:
// 0: top edge midpoint (0.5, 0)
// 1: right edge midpoint (1, 0.5)
// 2: bottom edge midpoint (0.5, 1)
// 3: left edge midpoint (0, 0.5)
const CASES: &[&[&[[f64; 2]; 2]]; 16] = &[
    &[],
    &[&[[1.0, 1.5], [0.5, 1.0]]],
    &[&[[1.5, 1.0], [1.0, 1.5]]],
    &[&[[1.5, 1.0], [0.5, 1.0]]],
    &[&[[1.0, 0.5], [1.5, 1.0]]],
    &[&[[1.0, 1.5], [0.5, 1.0]], &[[1.0, 0.5], [1.5, 1.0]]],
    &[&[[1.0, 0.5], [1.0, 1.5]]],
    &[&[[1.0, 0.5], [0.5, 1.0]]],
    &[&[[0.5, 1.0], [1.0, 0.5]]],
    &[&[[1.0, 1.5], [1.0, 0.5]]],
    &[&[[0.5, 1.0], [1.0, 0.5]], &[[1.5, 1.0], [1.0, 1.5]]],
    &[&[[1.5, 1.0], [1.0, 0.5]]],
    &[&[[0.5, 1.0], [1.5, 1.0]]],
    &[&[[1.0, 1.5], [1.5, 1.0]]],
    &[&[[0.5, 1.0], [1.0, 1.5]]],
    &[],
];

pub enum Thresholds {
    Count(usize),
    Values(Vec<f64>),
}

impl From<usize> for Thresholds {
    fn from(count: usize) -> Self {
        Thresholds::Count(count)
    }
}

impl From<Vec<f64>> for Thresholds {
    fn from(values: Vec<f64>) -> Self {
        Thresholds::Values(values)
    }
}

pub struct ContourGenerator {
    dx: usize,
    dy: usize,
    threshold: Box<dyn Fn(&[f64]) -> Vec<f64>>,
    smooth: Box<dyn Fn(Vec<[f64; 2]>, &[f64], f64, usize, usize) -> Vec<[f64; 2]>>,
}

impl ContourGenerator {
    pub fn new() -> Self {
        ContourGenerator {
            dx: 1,
            dy: 1,
            threshold: Box::new(threshold_sturges),
            smooth: Box::new(smooth_linear),
        }
    }

    pub fn size(mut self, s: [usize; 2]) -> Self {
        if s[0] == 0 || s[1] == 0 {
            panic!("invalid size");
        }
        self.dx = s[0];
        self.dy = s[1];
        self
    }

    pub fn thresholds(mut self, t: impl Into<Thresholds> + 'static) -> Self {
        match t.into() {
            Thresholds::Count(count) => {
                self.threshold = Box::new(move |values: &[f64]| {
                    let e = extent(values).expect("Extent could not be calculated");
                    let mut tz = ticks(nice(e.0, e.1, count).0, nice(e.0, e.1, count).1, count);
                    while let Some(&last) = tz.last() {
                        if last >= e.1 {
                            tz.pop();
                        } else {
                            break;
                        }
                    }
                    while let Some(&first) = tz.get(1) {
                        if first < e.0 {
                            tz.remove(0);
                        } else {
                            break;
                        }
                    }
                    tz
                });
            }
            Thresholds::Values(values) => {
                self.threshold = Box::new(move |_values: &[f64]| {
                    let mut v = values.clone();
                    v.sort_by(|a, b| ascending(a, b));
                    v
                });
            }
        }
        self
    }

    pub fn smooth(mut self, s: bool) -> Self {
        self.smooth = if s {
            Box::new(smooth_linear)
        } else {
            Box::new(|ring, _values, _value, _dx, _dy| ring) // noop
        };
        self
    }

    pub fn contours(&self, values: &[f64]) -> Vec<GeoJsonFeature> {
        let tz = (self.threshold)(values);
        tz.into_iter()
            .map(|value| self.contour(values, value))
            .collect()
    }

    fn contour(&self, values: &[f64], value: f64) -> GeoJsonFeature {
        let v = if value.is_nan() {
            panic!("invalid value: {}", value);
        } else {
            value
        };

        let mut polygons: Vec<Vec<Vec<[f64; 2]>>> = Vec::new();
        let mut holes: Vec<Vec<[f64; 2]>> = Vec::new();

        let dx = self.dx;
        let dy = self.dy;
        let smooth_fn = &self.smooth;

        isorings(values, v, dx, dy, |ring| {
            let smoothed_ring = (smooth_fn)(ring.clone(), values, v, dx, dy);
            if polygon_area(&smoothed_ring) > 0.0 {
                polygons.push(vec![smoothed_ring]);
            } else {
                holes.push(smoothed_ring);
            }
        });

        holes.into_iter().for_each(|hole| {
            for polygon in polygons.iter_mut() {
                if polygon_contains_polygon(&polygon[0], &hole) {
                    polygon.push(hole);
                    return;
                }
            }
        });

        GeoJsonFeature {
            _type: "MultiPolygon".to_string(),
            properties: None,
            geometry: GeoJsonGeometry {
                _type: "MultiPolygon".to_string(),
                coordinates: serde_json::to_value(polygons).unwrap(),
            },
        }
    }
}

// Helper functions

// Calculates the signed area of a polygon using the shoelace formula.
// A positive area typically indicates a counter-clockwise winding order (exterior ring).
// A negative area typically indicates a clockwise winding order (interior ring/hole).
fn polygon_area(ring: &Vec<[f64; 2]>) -> f64 {
    let n = ring.len();
    if n < 3 {
        return 0.0;
    } // Not a polygon

    let mut area = 0.0;
    for i in 0..n {
        let p1 = ring[i];
        let p2 = ring[(i + 1) % n]; // Wrap around to the first point
        area += (p1[0] * p2[1]) - (p1[1] * p2[0]);
    }
    area / 2.0
}

// Determines if a point is inside a polygon using the winding number algorithm.
// The polygon is represented by a list of its vertices (ring).
fn contains(polygon_ring: &Vec<[f64; 2]>, point: &[f64; 2]) -> bool {
    let x = point[0];
    let y = point[1];
    let n = polygon_ring.len();
    let mut winding_number = 0;

    for i in 0..n {
        let p1 = polygon_ring[i];
        let p2 = polygon_ring[(i + 1) % n];

        if p1[1] <= y {
            if p2[1] > y && (p2[0] - p1[0]) * (y - p1[1]) - (x - p1[0]) * (p2[1] - p1[1]) > 0.0 {
                winding_number += 1;
            }
        } else if p2[1] <= y && (p2[0] - p1[0]) * (y - p1[1]) - (x - p1[0]) * (p2[1] - p1[1]) < 0.0
        {
            winding_number -= 1;
        }
    }
    winding_number != 0
}

fn polygon_contains_polygon(outer: &Vec<[f64; 2]>, inner: &Vec<[f64; 2]>) -> bool {
    // A simplified check: if the first point of the inner polygon is within the outer polygon,
    // and the outer polygon has a positive area (exterior ring) and inner has negative (hole).
    // This is not a robust general solution for polygon-in-polygon, but a common heuristic.
    if polygon_area(outer) > 0.0 && polygon_area(inner) < 0.0 {
        contains(outer, &inner[0])
    } else {
        false
    }
}

fn above(x: f64, value: f64) -> bool {
    !x.is_nan() && x >= value
}

fn valid(v: f64) -> f64 {
    if v.is_nan() { f64::NEG_INFINITY } else { v }
}

fn smooth1(x: f64, v0: f64, v1: f64, value: f64) -> f64 {
    let a = value - v0;
    let b = v1 - v0;
    let d = if a.is_finite() || b.is_finite() {
        a / b
    } else {
        a.signum() / b.signum()
    };
    if d.is_nan() { x } else { x + d - 0.5 }
}

fn index(point: &[f64; 2], dx: usize) -> usize {
    (point[0] * 2.0 + point[1] * (dx + 1) as f64 * 4.0) as usize
}

fn threshold_sturges(values: &[f64]) -> Vec<f64> {
    let n = values.len();
    if n == 0 {
        return Vec::new();
    }
    let k = (f64::log2(n as f64) + 1.0).ceil() as usize;
    let (e0, e1) = extent(values).expect("Extent could not be calculated");
    ticks(nice(e0, e1, k).0, nice(e0, e1, k).1, k)
}

fn smooth_linear(
    mut ring: Vec<[f64; 2]>,
    values: &[f64],
    value: f64,
    dx: usize,
    dy: usize,
) -> Vec<[f64; 2]> {
    ring.iter_mut().for_each(|point| {
        let x = point[0];
        let y = point[1];
        let xt = x as usize;
        let yt = y as usize;
        let v1 = valid(values[yt * dx + xt]);
        if x > 0.0 && x < dx as f64 && xt as f64 == x {
            point[0] = smooth1(x, valid(values[yt * dx + xt - 1]), v1, value);
        }
        if y > 0.0 && y < dy as f64 && yt as f64 == y {
            point[1] = smooth1(y, valid(values[(yt - 1) * dx + xt]), v1, value);
        }
    });
    ring
}

fn isorings<F>(values: &[f64], value: f64, dx: usize, dy: usize, mut callback: F)
where
    F: FnMut(Vec<[f64; 2]>),
{
    let mut fragment_by_start: HashMap<usize, Fragment> = HashMap::new();
    let mut fragment_by_end: HashMap<usize, Fragment> = HashMap::new();

    let above_fn = |v: f64| above(v, value);

    // Special case for the first row (y = -1, t2 = t3 = 0).
    let mut x = -1;
    let mut y = -1;
    let mut t1 = above_fn(values[0]);
    for line in CASES[(t1 as usize) << 1].iter() {
        stitch(
            *line,
            &mut fragment_by_start,
            &mut fragment_by_end,
            x,
            y,
            dx,
            &mut callback,
        );
    }

    while x < (dx as isize) - 1 {
        x += 1;
        let t0 = t1;
        t1 = above_fn(values[x as usize + 1]);
        for line in CASES[(t0 as usize) | ((t1 as usize) << 1)].iter() {
            stitch(
                *line,
                &mut fragment_by_start,
                &mut fragment_by_end,
                x,
                y,
                dx,
                &mut callback,
            );
        }
    }
    for line in CASES[t1 as usize].iter() {
        stitch(
            *line,
            &mut fragment_by_start,
            &mut fragment_by_end,
            x,
            y,
            dx,
            &mut callback,
        );
    }

    // General case for the intermediate rows.
    while y < (dy as isize) - 1 {
        y += 1;
        x = -1;
        t1 = above_fn(values[(y * dx as isize + dx as isize) as usize]);
        let mut t2 = above_fn(values[(y * dx as isize) as usize]);
        for line in CASES[((t1 as usize) << 1) | ((t2 as usize) << 2)].iter() {
            stitch(
                *line,
                &mut fragment_by_start,
                &mut fragment_by_end,
                x,
                y,
                dx,
                &mut callback,
            );
        }

        while x < (dx as isize) - 1 {
            x += 1;
            let t0 = t1;
            t1 = above_fn(values[(y * dx as isize + dx as isize + x as isize + 1) as usize]);
            let t3 = t2;
            t2 = above_fn(values[(y * dx as isize + x as isize + 1) as usize]);
            for line in CASES
                [(t0 as usize) | ((t1 as usize) << 1) | ((t2 as usize) << 2) | ((t3 as usize) << 3)]
                .iter()
            {
                stitch(
                    *line,
                    &mut fragment_by_start,
                    &mut fragment_by_end,
                    x,
                    y,
                    dx,
                    &mut callback,
                );
            }
        }
        for line in CASES[(t1 as usize) | ((t2 as usize) << 3)].iter() {
            stitch(
                *line,
                &mut fragment_by_start,
                &mut fragment_by_end,
                x,
                y,
                dx,
                &mut callback,
            );
        }
    }

    // Special case for the last row (y = dy - 1, t0 = t1 = 0).
    x = -1;
    let mut t2 = above_fn(values[(y * dx as isize) as usize]);
    for line in CASES[(t2 as usize) << 2].iter() {
        stitch(
            *line,
            &mut fragment_by_start,
            &mut fragment_by_end,
            x,
            y,
            dx,
            &mut callback,
        );
    }

    while x < (dx as isize) - 1 {
        x += 1;
        let t3 = t2;
        t2 = above_fn(values[(y * dx as isize + x as isize + 1) as usize]);
        for line in CASES[((t2 as usize) << 2) | ((t3 as usize) << 3)].iter() {
            stitch(
                *line,
                &mut fragment_by_start,
                &mut fragment_by_end,
                x,
                y,
                dx,
                &mut callback,
            );
        }
    }
    for line in CASES[(t2 as usize) << 3].iter() {
        stitch(
            *line,
            &mut fragment_by_start,
            &mut fragment_by_end,
            x,
            y,
            dx,
            &mut callback,
        );
    }
}

#[derive(Debug, Clone)]
struct Fragment {
    start: usize,
    end: usize,
    ring: Vec<[f64; 2]>,
}

fn stitch<F>(
    line: &[[f64; 2]; 2],
    fragment_by_start: &mut HashMap<usize, Fragment>,
    fragment_by_end: &mut HashMap<usize, Fragment>,
    x: isize,
    y: isize,
    dx: usize,
    callback: &mut F,
) where
    F: FnMut(Vec<[f64; 2]>),
{
    let start_point = [line[0][0] + x as f64, line[0][1] + y as f64];
    let end_point = [line[1][0] + x as f64, line[1][1] + y as f64];
    let start_index = index(&start_point, dx);
    let end_index = index(&end_point, dx);

    let mut f_opt = fragment_by_end.remove(&start_index);
    let mut g_opt = fragment_by_start.remove(&end_index);

    if let Some(mut f) = f_opt {
        if let Some(mut g) = g_opt {
            fragment_by_start.remove(&f.start);
            fragment_by_end.remove(&g.end);
            if f.start == g.start && f.end == g.end {
                // This is a simplified check for f === g
                f.ring.push(end_point);
                callback(f.ring);
            } else {
                let new_ring = f.ring.into_iter().chain(g.ring.into_iter()).collect();
                let new_fragment = Fragment {
                    start: f.start,
                    end: g.end,
                    ring: new_ring,
                };
                fragment_by_start.insert(new_fragment.start, new_fragment.clone());
                fragment_by_end.insert(new_fragment.end, new_fragment);
            }
        } else {
            fragment_by_end.remove(&f.end);
            f.ring.push(end_point);
            f.end = end_index;
            fragment_by_end.insert(f.end, f);
        }
    } else if let Some(mut f) = g_opt {
        if let Some(mut g) = fragment_by_end.remove(&start_index) {
            fragment_by_start.remove(&f.start);
            fragment_by_end.remove(&g.end);
            if f.start == g.start && f.end == g.end {
                // This is a simplified check for f === g
                f.ring.push(end_point);
                callback(f.ring);
            } else {
                let new_ring = g.ring.into_iter().chain(f.ring.into_iter()).collect();
                let new_fragment = Fragment {
                    start: g.start,
                    end: f.end,
                    ring: new_ring,
                };
                fragment_by_start.insert(new_fragment.start, new_fragment.clone());
                fragment_by_end.insert(new_fragment.end, new_fragment);
            }
        } else {
            fragment_by_start.remove(&f.start);
            f.ring.insert(0, start_point);
            f.start = start_index;
            fragment_by_start.insert(f.start, f);
        }
    } else {
        let new_fragment = Fragment {
            start: start_index,
            end: end_index,
            ring: vec![start_point, end_point],
        };
        fragment_by_start.insert(start_index, new_fragment.clone());
        fragment_by_end.insert(end_index, new_fragment);
    }
}

pub fn contour() -> ContourGenerator {
    ContourGenerator::new()
}
