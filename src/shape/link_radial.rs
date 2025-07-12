// link_radial.rs
// Implements d3-shape's linkRadial: generates a radial link path string

pub struct LinkRadial;

impl LinkRadial {
    pub fn new() -> Self {
        LinkRadial
    }
    pub fn path(&self, source: (f64, f64), target: (f64, f64)) -> String {
        // Polar to cartesian conversion
        let (sr, sa) = source;
        let (tr, ta) = target;
        let sx = sr * sa.cos();
        let sy = sr * sa.sin();
        let tx = tr * ta.cos();
        let ty = tr * ta.sin();
        format!("M{:.3},{:.3}L{:.3},{:.3}", sx, sy, tx, ty)
    }
}
