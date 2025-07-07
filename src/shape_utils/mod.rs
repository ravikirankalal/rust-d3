//! D3 Shape Utilities module
//! Advanced shape utilities for D3.js API parity.

/// D3.js-like pie layout generator.
#[derive(Debug, Clone)]
pub struct PieSlice<T> {
    pub value: T,
    pub index: usize,
    pub start_angle: f64,
    pub end_angle: f64,
}

pub struct Pie;

impl Pie {
    /// Compute pie slices for a sequence of values.
    /// Returns a Vec<PieSlice<T>> with start/end angles in radians.
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

/// D3.js-like arc SVG path generator.
pub struct Arc;

impl Arc {
    /// Generate SVG path data for an arc/sector.
    /// Angles in radians, measured from x-axis.
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
            // Pie sector
            format!(
                "M0,0L{:.6},{:.6}A{:.6},{:.6} 0 {} {} {:.6},{:.6}Z",
                x0, y0, or, or, large_arc, sweep, x1, y1
            )
        } else {
            // Annular sector
            format!(
                "M{:.6},{:.6}A{:.6},{:.6} 0 {} {} {:.6},{:.6}L{:.6},{:.6}A{:.6},{:.6} 0 {} {} {:.6},{:.6}Z",
                x0, y0, or, or, large_arc, sweep, x1, y1,
                x2, y2, ir, ir, large_arc, 1 - sweep, x3, y3
            )
        }
    }
}

/// D3.js-like lineRadial generator (polar line to SVG path)
pub struct LineRadial {
    defined: Option<Box<dyn Fn(usize) -> bool + Send + Sync>>,
}

impl LineRadial {
    pub fn new() -> Self {
        Self { defined: None }
    }

    /// Set a custom defined accessor (optional)
    pub fn defined<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> bool + 'static + Send + Sync,
    {
        self.defined = Some(Box::new(f));
        self
    }

    /// Generate SVG path from (angle, radius) data
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

/// D3.js: d3.lineRadial (now implemented)
pub fn line_radial() -> LineRadial {
    LineRadial::new()
}

/// Placeholder for shape utilities.
pub fn shape_utils_placeholder() -> &'static str {
    "shape utils not implemented"
}
