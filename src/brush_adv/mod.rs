//! D3 Brush Advanced module
//! Advanced brushing, e.g., multi-brush, custom handles, etc.

/// Returns the union of two brush extents (min, max tuples).
pub fn brush_union(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0.min(b.0), a.1.max(b.1))
}
