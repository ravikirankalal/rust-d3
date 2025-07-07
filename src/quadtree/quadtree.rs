#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Quadtree {
    pub bounds: (f64, f64, f64, f64), // (x0, y0, x1, y1)
    pub points: Vec<Point>,
}

impl Quadtree {
    pub fn new(bounds: (f64, f64, f64, f64)) -> Self {
        Self {
            bounds,
            points: Vec::new(),
        }
    }

    pub fn insert(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn query(&self, rect: (f64, f64, f64, f64)) -> Vec<&Point> {
        let (x0, y0, x1, y1) = rect;
        self.points
            .iter()
            .filter(|p| p.x >= x0 && p.x <= x1 && p.y >= y0 && p.y <= y1)
            .collect()
    }

    /// Add a point to the quadtree (alias for insert)
    pub fn add(&mut self, point: Point) {
        self.insert(point);
    }
    /// Remove a point from the quadtree (removes first matching point)
    pub fn remove(&mut self, point: &Point) {
        if let Some(idx) = self.points.iter().position(|p| p.x == point.x && p.y == point.y) {
            self.points.remove(idx);
        }
    }
    /// Find the closest point to (x, y) in the quadtree, only if within bounds
    pub fn find(&self, x: f64, y: f64) -> Option<&Point> {
        if x < self.bounds.0 || x > self.bounds.2 || y < self.bounds.1 || y > self.bounds.3 {
            return None;
        }
        self.points.iter().min_by(|a, b| {
            let da = (a.x - x).powi(2) + (a.y - y).powi(2);
            let db = (b.x - x).powi(2) + (b.y - y).powi(2);
            da.partial_cmp(&db).unwrap()
        })
    }
    /// Visit all points in the quadtree (flat implementation)
    pub fn visit<F>(&self, mut callback: F)
    where
        F: FnMut(&Point),
    {
        for p in &self.points {
            callback(p);
        }
    }
    /// Expand the bounds to cover (x, y) if needed
    pub fn cover(&mut self, x: f64, y: f64) {
        let (mut x0, mut y0, mut x1, mut y1) = self.bounds;
        if x < x0 { x0 = x; }
        if y < y0 { y0 = y; }
        if x > x1 { x1 = x; }
        if y > y1 { y1 = y; }
        self.bounds = (x0, y0, x1, y1);
    }
}
