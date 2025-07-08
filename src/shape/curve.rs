// d3-shape: curve trait and implementations

pub trait Curve {
    fn begin(&mut self, path: &mut String);
    fn line_to(&mut self, path: &mut String, x: f64, y: f64, first: bool);
    fn end(&mut self, path: &mut String);
}

#[derive(Clone, Default)]
pub struct LinearCurve;

impl Curve for LinearCurve {
    fn begin(&mut self, _path: &mut String) {}
    fn line_to(&mut self, path: &mut String, x: f64, y: f64, first: bool) {
        if first {
            path.push_str(&format!("M{},{}", x, y));
        } else {
            path.push_str(&format!("L{},{}", x, y));
        }
    }
    fn end(&mut self, _path: &mut String) {}
}

#[derive(Clone, Default)]
pub struct StepCurve {
    prev_x: Option<f64>,
    prev_y: Option<f64>,
}

impl Curve for StepCurve {
    fn begin(&mut self, _path: &mut String) {
        self.prev_x = None;
        self.prev_y = None;
    }
    fn line_to(&mut self, path: &mut String, x: f64, y: f64, first: bool) {
        if first {
            path.push_str(&format!("M{},{}", x, y));
        } else {
            path.push_str(&format!("H{}", x));
            path.push_str(&format!("V{}", y));
        }
        self.prev_x = Some(x);
        self.prev_y = Some(y);
    }
    fn end(&mut self, _path: &mut String) {}
}

#[derive(Clone, Default)]
pub struct BasisCurve {
    points: Vec<(f64, f64)>,
}

impl Curve for BasisCurve {
    fn begin(&mut self, _path: &mut String) {
        self.points.clear();
    }
    fn line_to(&mut self, path: &mut String, x: f64, y: f64, _first: bool) {
        self.points.push((x, y));
    }
    fn end(&mut self, path: &mut String) {
        if self.points.is_empty() {
            return;
        }
        match self.points.len() {
            0 => return,
            1 => {
                let (x, y) = self.points[0];
                path.push_str(&format!("M{},{}", x, y));
            },
            2 => {
                let (x0, y0) = self.points[0];
                let (x1, y1) = self.points[1];
                path.push_str(&format!("M{},{}L{},{}", x0, y0, x1, y1));
            },
            3 => {
                // D3 basis for 3 points: single cubic segment
                let (x0, y0) = self.points[0];
                let (x1, y1) = self.points[1];
                let (x2, y2) = self.points[2];
                path.push_str(&format!("M{},{}", x0, y0));
                let c1x = (x0 + 4.0 * x1 + x2) / 6.0;
                let c1y = (y0 + 4.0 * y1 + y2) / 6.0;
                let c2x = (x0 + 2.0 * x1 + 3.0 * x2) / 6.0;
                let c2y = (y0 + 2.0 * y1 + 3.0 * y2) / 6.0;
                let ex = (x0 + 4.0 * x2 + x2) / 6.0;
                let ey = (y0 + 4.0 * y2 + y2) / 6.0;
                path.push_str(&format!("C{},{} {},{} {},{}", c1x, c1y, c2x, c2y, ex, ey));
            },
            _ => {
                // D3 basis: B-spline interpolation for 4+ points
                let mut pts = self.points.iter();
                let (mut x0, mut y0) = *pts.next().unwrap();
                let (mut x1, mut y1) = *pts.next().unwrap();
                let (mut x2, mut y2) = *pts.next().unwrap();
                path.push_str(&format!("M{},{}", x0, y0));
                for &(x3, y3) in pts {
                    let c1x = (x0 + 4.0 * x1 + x2) / 6.0;
                    let c1y = (y0 + 4.0 * y1 + y2) / 6.0;
                    let c2x = (x1 + 4.0 * x2 + x3) / 6.0;
                    let c2y = (y1 + 4.0 * y2 + y3) / 6.0;
                    let ex = (x2 + 4.0 * x3 + x3) / 6.0;
                    let ey = (y2 + 4.0 * y3 + y3) / 6.0;
                    path.push_str(&format!("C{},{} {},{} {},{}", c1x, c1y, c2x, c2y, ex, ey));
                    x0 = x1;
                    y0 = y1;
                    x1 = x2;
                    y1 = y2;
                    x2 = x3;
                    y2 = y3;
                }
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct CardinalCurve {
    points: Vec<(f64, f64)>,
    tension: f64,
}

impl CardinalCurve {
    pub fn with_tension(tension: f64) -> Self {
        Self { points: Vec::new(), tension }
    }
}

impl Curve for CardinalCurve {
    fn begin(&mut self, _path: &mut String) {
        self.points.clear();
        if self.tension < 0.0 { self.tension = 0.0; }
        if self.tension > 1.0 { self.tension = 1.0; }
    }
    fn line_to(&mut self, _path: &mut String, x: f64, y: f64, _first: bool) {
        self.points.push((x, y));
    }
    fn end(&mut self, path: &mut String) {
        if self.points.is_empty() {
            return;
        }
        let n = self.points.len();
        if n < 2 {
            for (i, &(x, y)) in self.points.iter().enumerate() {
                if i == 0 {
                    path.push_str(&format!("M{},{}", x, y));
                } else {
                    path.push_str(&format!("L{},{}", x, y));
                }
            }
            return;
        }
        let t = (1.0 - self.tension) / 6.0;
        let pts = &self.points;
        path.push_str(&format!("M{},{}", pts[0].0, pts[0].1));
        for i in 0..n - 1 {
            // D3 clamps endpoints for cardinal spline
            let (x0, y0) = if i == 0 { pts[0] } else { pts[i - 1] };
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[i + 1];
            let (x3, y3) = if i + 2 < n { pts[i + 2] } else { pts[n - 1] };
            let c1x = x1 + t * (x2 - x0);
            let c1y = y1 + t * (y2 - y0);
            let c2x = x2 - t * (x3 - x1);
            let c2y = y2 - t * (y3 - y1);
            path.push_str(&format!("C{},{} {},{} {},{}", c1x, c1y, c2x, c2y, x2, y2));
        }
    }
}

#[derive(Clone, Default)]
pub struct MonotoneCurve {
    points: Vec<(f64, f64)>,
}

impl Curve for MonotoneCurve {
    fn begin(&mut self, _path: &mut String) {
        self.points.clear();
    }
    fn line_to(&mut self, _path: &mut String, x: f64, y: f64, _first: bool) {
        self.points.push((x, y));
    }
    fn end(&mut self, path: &mut String) {
        if self.points.is_empty() {
            return;
        }
        let n = self.points.len();
        if n < 2 {
            for (i, &(x, y)) in self.points.iter().enumerate() {
                if i == 0 {
                    path.push_str(&format!("M{},{}", x, y));
                } else {
                    path.push_str(&format!("L{},{}", x, y));
                }
            }
            return;
        }
        // D3 monotone cubic interpolation
        let pts = &self.points;
        let mut dx = vec![0.0; n - 1];
        let mut dy = vec![0.0; n - 1];
        let mut m = vec![0.0; n - 1];
        for i in 0..n - 1 {
            dx[i] = pts[i + 1].0 - pts[i].0;
            dy[i] = pts[i + 1].1 - pts[i].1;
            m[i] = if dx[i] != 0.0 { dy[i] / dx[i] } else { 0.0 };
        }
        let mut t = vec![0.0; n];
        t[0] = m[0];
        t[n - 1] = m[n - 2];
        for i in 1..n - 1 {
            if m[i - 1] * m[i] > 0.0 {
                t[i] = (m[i - 1] + m[i]) / 2.0;
            } else {
                t[i] = 0.0;
            }
        }
        path.push_str(&format!("M{},{}", pts[0].0, pts[0].1));
        for i in 0..n - 1 {
            let h = dx[i];
            let x0 = pts[i].0;
            let y0 = pts[i].1;
            let x1 = pts[i + 1].0;
            let y1 = pts[i + 1].1;
            let c1x = x0 + h / 3.0;
            let c1y = y0 + t[i] * h / 3.0;
            let c2x = x1 - h / 3.0;
            let c2y = y1 - t[i + 1] * h / 3.0;
            path.push_str(&format!("C{},{} {},{} {},{}", c1x, c1y, c2x, c2y, x1, y1));
        }
    }
}
