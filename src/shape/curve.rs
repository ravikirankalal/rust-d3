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
    fn line_to(&mut self, _path: &mut String, x: f64, y: f64, _first: bool) {
        self.points.push((x, y));
    }
    fn end(&mut self, path: &mut String) {
        let n = self.points.len();
        eprintln!("[BasisCurve] points: {:?}", self.points);
        if n == 0 {
            eprintln!("[BasisCurve] n == 0, returning");
            return;
        }
        if n == 1 {
            let (x, y) = self.points[0];
            eprintln!("[BasisCurve] n == 1, M{},{}", x, y);
            path.push_str(&format!("M{},{}", x, y));
            return;
        }
        if n == 2 {
            let (x0, y0) = self.points[0];
            let (x1, y1) = self.points[1];
            eprintln!("[BasisCurve] n == 2, M{},{}L{},{}", x0, y0, x1, y1);
            path.push_str(&format!("M{},{}L{},{}", x0, y0, x1, y1));
            return;
        }
        if n == 3 {
            let (x0, y0) = self.points[0];
            let (x1, y1) = self.points[1];
            let (x2, y2) = self.points[2];
            eprintln!("[BasisCurve] n == 3, points: ({},{}) ({},{}) ({},{})", x0, y0, x1, y1, x2, y2);
            // First segment: endpoint is (x1, y1)
            let c1x1 = (2.0 * x0 + x1) / 3.0;
            let c1y1 = (2.0 * y0 + y1) / 3.0;
            let c2x1 = (x0 + 2.0 * x1) / 3.0;
            let c2y1 = (y0 + 2.0 * y1) / 3.0;
            let ex1 = x1;
            let ey1 = y1;
            eprintln!("[BasisCurve] seg 0: C1: {},{} C2: {},{} E: {},{}", c1x1, c1y1, c2x1, c2y1, ex1, ey1);
            // Second segment: endpoint is (x2, y2)
            let c1x2 = (2.0 * x1 + x2) / 3.0;
            let c1y2 = (2.0 * y1 + y2) / 3.0;
            let c2x2 = (x1 + 2.0 * x2) / 3.0;
            let c2y2 = (y1 + 2.0 * y2) / 3.0;
            let ex2 = x2;
            let ey2 = y2;
            eprintln!("[BasisCurve] seg 1: C1: {},{} C2: {},{} E: {},{}", c1x2, c1y2, c2x2, c2y2, ex2, ey2);
            path.push_str(&format!("M{},{}", x0, y0));
            path.push_str(&format!("C{},{} {},{} {},{}", c1x1, c1y1, c2x1, c2y1, ex1, ey1));
            path.push_str(&format!("C{},{} {},{} {},{}", c1x2, c1y2, c2x2, c2y2, ex2, ey2));
            return;
        }
        // D3 basis: for >3 points, pad [p0, p0, p1, ..., pn-1, pn-1]
        let mut pts = Vec::with_capacity(n + 2);
        pts.push(self.points[0]);
        pts.push(self.points[0]);
        pts.extend(self.points.iter().cloned());
        pts.push(self.points[n - 1]);
        pts.push(self.points[n - 1]);
        eprintln!("[BasisCurve] padded pts: {:?}", pts);
        path.push_str(&format!("M{},{}", self.points[0].0, self.points[0].1));
        // D3's basis.js: for (i = 0; i < n - 1; ++i)
        for i in 0..(n - 1) {
            let (_p0x, _p0y) = pts[i];
            let (p1x, p1y) = pts[i + 1];
            let (p2x, p2y) = pts[i + 2];
            let (p3x, p3y) = pts[i + 3];
            let c1x = (2.0 * p1x + p2x) / 3.0;
            let c1y = (2.0 * p1y + p2y) / 3.0;
            let c2x = (p1x + 2.0 * p2x) / 3.0;
            let c2y = (p1y + 2.0 * p2y) / 3.0;
            let ex = (p1x + 4.0 * p2x + p3x) / 6.0;
            let ey = (p1y + 4.0 * p2y + p3y) / 6.0;
            eprintln!("[BasisCurve] seg {}: C1: {:?},{:?} C2: {:?},{:?} E: {:?},{:?}", i, c1x, c1y, c2x, c2y, ex, ey);
            path.push_str(&format!("C{},{} {},{} {},{}", c1x, c1y, c2x, c2y, ex, ey));
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
        println!("[CardinalCurve] points: {:?}", pts);
        path.push_str(&format!("M{},{}", pts[0].0, pts[0].1));
        if n == 3 {
            // D3 logic for 3 points: use endpoint extrapolation for phantom points
            let (x0, y0) = pts[0];
            let (x1, y1) = pts[1];
            let (x2, y2) = pts[2];
            // Extrapolate phantom points
            let (xp, yp) = (2.0 * x0 - x1, 2.0 * y0 - y1); // before x0
            let (xn, yn) = (2.0 * x2 - x1, 2.0 * y2 - y1); // after x2
            // First segment: use xp as previous, x2 as next
            let c1x1 = x0 + t * (x1 - xp);
            let c1y1 = y0 + t * (y1 - yp);
            let c2x1 = x1 - t * (x2 - x0);
            let c2y1 = y1 - t * (y2 - y0);
            println!("[CardinalCurve] seg 0: C1: {},{} C2: {},{} E: {},{}", c1x1, c1y1, c2x1, c2y1, x1, y1);
            path.push_str(&format!("C{},{} {},{} {},{}", c1x1, c1y1, c2x1, c2y1, x1, y1));
            // Second segment: use x0 as previous, xn as next
            let c1x2 = x1 + t * (x2 - x0);
            let c1y2 = y1 + t * (y2 - y0);
            let c2x2 = x2 - t * (xn - x1);
            let c2y2 = y2 - t * (yn - y1);
            println!("[CardinalCurve] seg 1: C1: {},{} C2: {},{} E: {},{}", c1x2, c1y2, c2x2, c2y2, x2, y2);
            path.push_str(&format!("C{},{} {},{} {},{}", c1x2, c1y2, c2x2, c2y2, x2, y2));
            return;
        }
        for i in 0..n - 1 {
            let (x0, y0) = if i == 0 { pts[0] } else { pts[i - 1] };
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[i + 1];
            let (x3, y3) = if i + 2 < n { pts[i + 2] } else { pts[n - 1] };
            let c1x = x1 + t * (x2 - x0);
            let c1y = y1 + t * (y2 - y0);
            let c2x = x2 - t * (x3 - x1);
            let c2y = y2 - t * (y3 - y1);
            println!("[CardinalCurve] seg {}: C1: {},{} C2: {},{} E: {},{}", i, c1x, c1y, c2x, c2y, x2, y2);
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
        println!("[MonotoneCurve] points: {:?}", pts);
        println!("[MonotoneCurve] dx: {:?}", dx);
        println!("[MonotoneCurve] dy: {:?}", dy);
        println!("[MonotoneCurve] m: {:?}", m);
        println!("[MonotoneCurve] t: {:?}", t);
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
            println!("[MonotoneCurve] seg {}: C1: {},{} C2: {},{} E: {},{}", i, c1x, c1y, c2x, c2y, x1, y1);
            path.push_str(&format!("C{},{} {},{} {},{}", c1x, c1y, c2x, c2y, x1, y1));
        }
    }
}
