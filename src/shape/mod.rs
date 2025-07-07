// D3.js Shape module: unified API for all shape, and symbol features

// --- Shape Generators ---
mod shape;
pub use shape::{LineGenerator, PieGenerator, AreaGenerator, ArcGenerator, SymbolGenerator, RadialAreaGenerator, RadialLineGenerator};

// --- Advanced Shape Generators ---
// (from shape_adv)
pub fn area<T, F>(data: &[T], mut accessor: F) -> Vec<(f64, f64)>
where
    F: FnMut(&T) -> (f64, f64),
{
    data.iter().map(|d| accessor(d)).collect()
}

pub fn arc_points(_inner_radius: f64, outer_radius: f64, start_angle: f64, end_angle: f64) -> Vec<(f64, f64)> {
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

// --- Advanced 2 Placeholder ---
pub fn arc2_placeholder() -> &'static str {
    "arc2 not implemented"
}

// --- Shape Utilities ---
#[derive(Debug, Clone)]
pub struct PieSlice<T> {
    pub value: T,
    pub index: usize,
    pub start_angle: f64,
    pub end_angle: f64,
}

pub struct Pie;
impl Pie {
    pub fn layout<T: Copy + Into<f64>>(values: &[T]) -> Vec<PieSlice<T>> {
        let sum: f64 = values.iter().map(|&v| v.into()).sum();
        if sum == 0.0 || values.is_empty() {
            return Vec::new();
        }
        let mut start = 0.0;
        let mut slices = Vec::with_capacity(values.len());
        for (i, &v) in values.iter().enumerate() {
            let angle = v.into() / sum * std::f64::consts::TAU;
            let end = start + angle;
            slices.push(PieSlice {
                value: v,
                index: i,
                start_angle: start,
                end_angle: end,
            });
            start = end;
        }
        slices
    }
}

pub struct Arc;
impl Arc {
    pub fn path(
        inner_radius: f64,
        outer_radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> String {
        if (end_angle - start_angle).abs() < 1e-10 || outer_radius <= 0.0 {
            return String::new();
        }
        let (ir, or) = (inner_radius.max(0.0), outer_radius.max(0.0));
        let (sa, ea) = (start_angle, end_angle);
        let large_arc = if (ea - sa).abs() > std::f64::consts::PI { 1 } else { 0 };
        let sweep = if ea > sa { 1 } else { 0 };
        let (x0, y0) = (or * sa.cos(), or * sa.sin());
        let (x1, y1) = (or * ea.cos(), or * ea.sin());
        let (x2, y2) = (ir * ea.cos(), ir * ea.sin());
        let (x3, y3) = (ir * sa.cos(), ir * sa.sin());
        if ir == 0.0 {
            format!(
                "M0,0L{:.6},{:.6}A{:.6},{:.6} 0 {} {} {:.6},{:.6}Z",
                x0, y0, or, or, large_arc, sweep, x1, y1
            )
        } else {
            format!(
                "M{:.6},{:.6}A{:.6},{:.6} 0 {} {} {:.6},{:.6}L{:.6},{:.6}A{:.6},{:.6} 0 {} {} {:.6},{:.6}Z",
                x0, y0, or, or, large_arc, sweep, x1, y1,
                x2, y2, ir, ir, large_arc, 1 - sweep, x3, y3
            )
        }
    }
}

pub struct LineRadial {
    defined: Option<Box<dyn Fn(usize) -> bool + Send + Sync>>,
}
impl LineRadial {
    pub fn new() -> Self {
        Self { defined: None }
    }
    pub fn defined<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> bool + 'static + Send + Sync,
    {
        self.defined = Some(Box::new(f));
        self
    }
    pub fn path(&self, data: &[(f64, f64)]) -> String {
        let mut d = String::new();
        let mut started = false;
        for (i, &(a, r)) in data.iter().enumerate() {
            let is_defined = self.defined.as_ref().map_or(true, |f| f(i));
            if !is_defined || r < 0.0 {
                started = false;
                continue;
            }
            let (x, y) = (r * a.cos(), r * a.sin());
            if !started {
                d.push_str(&format!("M{:.6},{:.6}", x, y));
                started = true;
            } else {
                d.push_str(&format!("L{:.6},{:.6}", x, y));
            }
        }
        d
    }
}
pub fn line_radial() -> LineRadial {
    LineRadial::new()
}

pub fn shape_utils_placeholder() -> &'static str {
    "shape utils not implemented"
}

pub fn polygon_area(points: &[(f64, f64)]) -> f64 {
    let n = points.len();
    if n < 3 {
        return 0.0;
    }
    let mut area = 0.0;
    for i in 0..n {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % n];
        area += x0 * y1 - x1 * y0;
    }
    area.abs() * 0.5
}

pub fn polygon_centroid(points: &[(f64, f64)]) -> (f64, f64) {
    let n = points.len();
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut area = 0.0;
    for i in 0..n {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % n];
        let a = x0 * y1 - x1 * y0;
        cx += (x0 + x1) * a;
        cy += (y0 + y1) * a;
        area += a;
    }
    if area == 0.0 {
        return (0.0, 0.0);
    }
    (cx / (3.0 * area), cy / (3.0 * area))
}

pub fn polygon_bounds(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let (mut min_x, mut min_y) = (f64::INFINITY, f64::INFINITY);
    let (mut max_x, mut max_y) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    for &(x, y) in points {
        if x < min_x { min_x = x; }
        if y < min_y { min_y = y; }
        if x > max_x { max_x = x; }
        if y > max_y { max_y = y; }
    }
    ((min_x, min_y), (max_x, max_y))
}

// --- Stack Generators ---
pub fn stack<T: Copy + Into<f64>>(series: &[Vec<T>]) -> Vec<Vec<(f64, f64)>> {
    if series.is_empty() {
        return vec![];
    }
    let n = series[0].len();
    let mut result = vec![vec![(0.0, 0.0); n]; series.len()];
    for i in 0..n {
        let mut acc = 0.0;
        for (j, s) in series.iter().enumerate() {
            let v = s[i].into();
            result[j][i] = (acc, acc + v);
            acc += v;
        }
    }
    result
}

pub fn stack_order_none<T>() {
    // TODO: Implement stack order strategies (none, ascending, descending, reverse, etc.)
}

pub fn stack_offset_none<T>() {
    // TODO: Implement stack offset strategies (none, expand, silhouette, wiggle, diverging, etc.)
}

// --- Stack Advanced ---
pub fn stack_offset_expand(series: &mut [Vec<(f64, f64)>]) {
    if series.is_empty() { return; }
    let n = series[0].len();
    for i in 0..n {
        let sum: f64 = series.iter().map(|s| s[i].1 - s[i].0).sum();
        if sum != 0.0 {
            let mut acc = 0.0;
            for s in series.iter_mut() {
                let v = s[i].1 - s[i].0;
                s[i].0 = acc / sum;
                acc += v;
                s[i].1 = acc / sum;
            }
        } else {
            for s in series.iter_mut() {
                s[i].0 = 0.0;
                s[i].1 = 0.0;
            }
        }
    }
}

pub fn stack_offset_silhouette(series: &mut [Vec<(f64, f64)>]) {
    if series.is_empty() { return; }
    let n = series[0].len();
    for i in 0..n {
        let sum: f64 = series.iter().map(|s| s[i].1 - s[i].0).sum();
        let offset = -sum / 2.0;
        for s in series.iter_mut() {
            s[i].0 += offset;
            s[i].1 += offset;
        }
    }
}

// --- Stack Builder ---
use std::marker::PhantomData;
pub struct SeriesMeta<K> {
    pub key: K,
    pub index: usize,
}
pub struct StackedSeries<K, P> {
    pub meta: SeriesMeta<K>,
    pub points: Vec<(f64, f64, P)>,
}
pub struct Stack<D, K> {
    keys: Option<Vec<K>>,
    _phantom: PhantomData<D>,
}
impl<D, K> Stack<D, K>
where
    K: Clone + PartialEq + 'static,
    D: Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            keys: None,
            _phantom: PhantomData,
        }
    }
    pub fn keys(mut self, keys: Vec<K>) -> Self {
        self.keys = Some(keys);
        self
    }
    // ...existing builder pattern methods...
}

// --- Symbol Generators ---
pub fn symbol(symbol_type: &str, size: f64) -> String {
    match symbol_type {
        "circle" => symbol_circle(size),
        "cross" => symbol_cross(size),
        "diamond" => symbol_diamond(size),
        "square" => symbol_square(size),
        "star" => symbol_star(size),
        "triangle" => symbol_triangle(size),
        "wye" => symbol_wye(size),
        _ => String::new(),
    }
}

pub fn symbol_circle(size: f64) -> String {
    let r = (size / std::f64::consts::PI).sqrt();
    format!(
        "M{:.6} 0A{:.6} {:.6} 0 1 1 {:.6} 0A{:.6} {:.6} 0 1 1 {:.6} 0Z",
        r, r, r, -r, r, r, r
    )
}

pub fn symbol_cross(size: f64) -> String {
    let s = (size / 5.0).sqrt();
    format!("M{:.3},0L{:.3},0M0,{:.3}L0,{:.3}", -s, s, -s, s)
}

pub fn symbol_diamond(size: f64) -> String {
    let y = (size / (2.0 * (3.0f64).sqrt())).sqrt();
    let x = y * (3.0f64).sqrt();
    format!("M0,{:.3}L{:.3},0L0,{:.3}L{:.3},0Z", -y, x, y, -x)
}

pub fn symbol_square(size: f64) -> String {
    let w = size.sqrt() / 2.0;
    format!("M{:.3},{:.3}L{:.3},{:.3}L{:.3},{:.3}L{:.3},{:.3}Z", -w, -w, w, -w, w, w, -w, w)
}

pub fn symbol_star(size: f64) -> String {
    let r = (size * 1.25 / std::f64::consts::PI).sqrt();
    let mut path = String::new();
    for i in 0..10 {
        let a = i as f64 * std::f64::consts::PI / 5.0;
        let r_i = if i % 2 == 0 { r } else { r / 2.5 };
        let x = r_i * a.sin();
        let y = -r_i * a.cos();
        if i == 0 {
            path += &format!("M{},{}", x, y);
        } else {
            path += &format!("L{},{}", x, y);
        }
    }
    path + "Z"
}

pub fn symbol_triangle(size: f64) -> String {
    let h = (size * (4.0 / (3.0f64).sqrt())).sqrt();
    let y = -h / 2.0;
    let x = h / (2.0 * (3.0f64).sqrt());
    format!("M0,{}L{},{}L{},{}Z", y, x, -y, -x, -y)
}

pub fn symbol_wye(size: f64) -> String {
    let r = (size / ((3.0f64).sqrt() * 2.0)).sqrt();
    format!("M0,0L{:.3},{:.3}L{:.3},{:.3}Z", r, r, -r, r)
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_polygon_area() {
        let triangle = vec![(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
        assert_eq!(polygon_area(&triangle), 6.0);
    }
    #[test]
    fn test_polygon_centroid() {
        let triangle = vec![(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
        let (cx, cy) = polygon_centroid(&triangle);
        assert!((cx - 1.3333).abs() < 1e-3);
        assert!((cy - 1.0).abs() < 1e-3);
    }
    #[test]
    fn test_polygon_bounds() {
        let triangle = vec![(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
        let ((min_x, min_y), (max_x, max_y)) = polygon_bounds(&triangle);
        assert_eq!((min_x, min_y), (0.0, 0.0));
        assert_eq!((max_x, max_y), (4.0, 3.0));
    }
}
