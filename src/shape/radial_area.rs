// radial_area.rs
// Implements d3-shape's radialArea/areaRadial: generates a radial area path string

pub struct RadialArea;

impl RadialArea {
    pub fn new() -> Self { RadialArea }
    pub fn path(&self, points: &[(f64, f64)]) -> String {
        if points.is_empty() { return String::new(); }
        let mut d = String::new();
        for (i, &(r, a)) in points.iter().enumerate() {
            let x = r * a.cos();
            let y = r * a.sin();
            if i == 0 {
                d += &format!("M{:.3},{:.3}", x, y);
            } else {
                d += &format!("L{:.3},{:.3}", x, y);
            }
        }
        d + "Z"
    }
}
