// src/contour/mod.rs

use crate::array::extent::extent;
use crate::array::nice::nice;
use crate::array::ticks::ticks;

pub mod marching_squares;

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

// --- d3-contour density estimation API ---

pub struct ContourDensity {
    x: Box<dyn Fn(&[f64]) -> f64>,
    y: Box<dyn Fn(&[f64]) -> f64>,
    value: Box<dyn Fn(&[f64]) -> f64>,
    weight: Box<dyn Fn(&[f64]) -> f64>,
    bandwidth: f64,
    thresholds: Thresholds,
    size: [usize; 2],
    smooth: Box<dyn Fn(Vec<[f64; 2]>, &[f64], f64, usize, usize) -> Vec<[f64; 2]>>,
}

impl ContourDensity {
    pub fn new() -> Self {
        Self {
            x: Box::new(|d| d[0]),
            y: Box::new(|d| d[1]),
            value: Box::new(|_d| 1.0),
            weight: Box::new(|_d| 1.0),
            bandwidth: 20.0,
            thresholds: Thresholds::Count(20),
            size: [960, 500],
            smooth: Box::new(smooth_linear),
        }
    }
    pub fn x<F: 'static + Fn(&[f64]) -> f64>(mut self, f: F) -> Self { self.x = Box::new(f); self }
    pub fn y<F: 'static + Fn(&[f64]) -> f64>(mut self, f: F) -> Self { self.y = Box::new(f); self }
    pub fn value<F: 'static + Fn(&[f64]) -> f64>(mut self, f: F) -> Self { self.value = Box::new(f); self }
    pub fn weight<F: 'static + Fn(&[f64]) -> f64>(mut self, f: F) -> Self { self.weight = Box::new(f); self }
    pub fn bandwidth(mut self, b: f64) -> Self { self.bandwidth = b; self }
    pub fn thresholds(mut self, t: impl Into<Thresholds> + 'static) -> Self { self.thresholds = t.into(); self }
    pub fn size(mut self, s: [usize; 2]) -> Self { self.size = s; self }
    pub fn smooth(mut self, s: bool) -> Self {
        self.smooth = if s {
            Box::new(smooth_linear)
        } else {
            Box::new(|ring, _values, _value, _dx, _dy| ring) // noop
        };
        self
    }
    pub fn smooth_cubic(mut self) -> Self {
        self.smooth = Box::new(|ring, _values, _value, _dx, _dy| {
            // TODO: Implement cubic smoothing
            ring // fallback to linear for now
        });
        self
    }
    pub fn compute(&self, data: &[Vec<f64>]) -> Vec<crate::geojson::GeoJsonFeature> {
        let [width, height] = self.size;
        let n = data.len();
        if n == 0 { return vec![]; }
        let mut grid = vec![0.0; width * height];
        let bw = self.bandwidth;
        let bw2 = bw * bw;
        let kernel = |dx: f64, dy: f64| {
            let r2 = dx * dx + dy * dy;
            if r2 > bw2 { 0.0 } else { (1.0 - r2 / bw2).max(0.0) }
        };
        // Compute density grid
        for j in 0..height {
            for i in 0..width {
                let x = i as f64;
                let y = j as f64;
                let mut sum = 0.0;
                let mut wsum = 0.0;
                for d in data {
                    let dx = x - (self.x)(d);
                    let dy = y - (self.y)(d);
                    let w = (self.weight)(d);
                    let k = kernel(dx, dy);
                    sum += k * w;
                    wsum += w;
                }
                grid[j * width + i] = if wsum > 0.0 { sum / wsum } else { 0.0 };
            }
        }
        // Use thresholds
        let tz = match &self.thresholds {
            Thresholds::Count(count) => {
                let e = extent(&grid).unwrap();
                ticks(nice(e.0, e.1, *count).0, nice(e.0, e.1, *count).1, *count)
            },
            Thresholds::Values(v) => v.clone(),
        };
        // Use marching squares to extract contours
        let generator = ContourGenerator::new().size([width, height]);
        tz.into_iter().map(|value| generator.contour(&grid, value)).collect()
    }
}

pub fn contour() -> ContourGenerator {
    ContourGenerator::new()
}

pub fn contour_density() -> ContourDensity {
    ContourDensity::new()
}

// Helper functions

fn polygon_area(ring: &Vec<[f64; 2]>) -> f64 {
    let n = ring.len();
    if n < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    for i in 0..n {
        let p1 = ring[i];
        let p2 = ring[(i + 1) % n];
        area += (p1[0] * p2[1]) - (p1[1] * p2[0]);
    }
    area / 2.0
}

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
    if polygon_area(outer) > 0.0 && polygon_area(inner) < 0.0 {
        contains(outer, &inner[0])
    } else {
        false
    }
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

#[allow(dead_code)]
fn index(point: &[f64; 2], dx: usize) -> usize {
    (point[0] * 2.0 + point[1] * (dx as f64 + 1.0) * 4.0) as usize
}

#[allow(dead_code)]
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
        // Only smooth if indices are in bounds
        if xt < dx && yt < dy {
            let v1 = valid(values[yt * dx + xt]);
            if x > 0.0 && x < dx as f64 && xt as f64 == x {
                if xt > 0 && (yt * dx + xt - 1) < values.len() {
                    point[0] = smooth1(x, valid(values[yt * dx + xt - 1]), v1, value);
                }
            }
            if y > 0.0 && y < dy as f64 && yt as f64 == y {
                if yt > 0 && ((yt - 1) * dx + xt) < values.len() {
                    point[1] = smooth1(y, valid(values[(yt - 1) * dx + xt]), v1, value);
                }
            }
        }
    });
    ring
}

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
        Self {
            dx: 0,
            dy: 0,
            threshold: Box::new(|_| vec![]),
            smooth: Box::new(smooth_linear),
        }
    }

    pub fn size(mut self, size: [usize; 2]) -> Self {
        self.dx = size[0];
        self.dy = size[1];
        self
    }

    pub fn thresholds(mut self, t: impl Into<crate::contour::Thresholds> + 'static) -> Self {
        match t.into() {
            crate::contour::Thresholds::Count(count) => {
                self.threshold = Box::new(move |values: &[f64]| {
                    let e = crate::array::extent::extent(values).expect("Extent could not be calculated");
                    let mut tz = crate::array::ticks::ticks(crate::array::nice::nice(e.0, e.1, count).0, crate::array::nice::nice(e.0, e.1, count).1, count);
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
            crate::contour::Thresholds::Values(values) => {
                self.threshold = Box::new(move |_values: &[f64]| {
                    let mut v = values.clone();
                    v.sort_by(|a, b| crate::array::ascending::ascending(a, b));
                    v
                });
            }
        }
        self
    }

    pub fn contour(&self, values: &[f64], value: f64) -> crate::geojson::GeoJsonFeature {
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

        crate::contour::marching_squares::isorings(values, v, dx, dy, |ring| {
            println!("DEBUG: Emitting ring: {:?}", ring);
            let smoothed_ring = (smooth_fn)(ring.clone(), values, v, dx, dy);
            if crate::contour::polygon_area(&smoothed_ring) > 0.0 {
                polygons.push(vec![smoothed_ring]);
            } else {
                holes.push(smoothed_ring);
            }
        });

        holes.into_iter().for_each(|hole| {
            for polygon in polygons.iter_mut() {
                if crate::contour::polygon_contains_polygon(&polygon[0], &hole) {
                    polygon.push(hole);
                    return;
                }
            }
        });

        let mut properties = std::collections::HashMap::new();
        properties.insert("value".to_string(), serde_json::json!(value));

        crate::geojson::GeoJsonFeature {
            _type: "Feature".to_string(),
            properties: Some(properties),
            geometry: crate::geojson::GeoJsonGeometry {
                _type: "MultiPolygon".to_string(),
                coordinates: serde_json::to_value(polygons).unwrap(),
            },
        }
    }

    pub fn contours(&self, values: &[f64]) -> Vec<crate::geojson::GeoJsonFeature> {
        let tz = (self.threshold)(values);
        tz.into_iter()
            .map(|value| self.contour(values, value))
            .collect()
    }
}