// Delaunay triangulation for D3.js parity
// Minimal implementation using Bowyer–Watson algorithm for small datasets

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [usize; 3], // indices into points
}

pub struct Delaunay {
    pub points: Vec<(f64, f64)>,
    pub triangles: Vec<Triangle>,
}

impl Delaunay {
    /// Create a new Delaunay triangulation from points
    pub fn new(points: &[(f64, f64)]) -> Self {
        let points = points.to_vec();
        let triangles = bowyer_watson(&points);
        Delaunay { points, triangles }
    }
    /// Find the closest point in the triangulation
    pub fn find(&self, x: f64, y: f64) -> Option<usize> {
        self.points.iter().enumerate().min_by(|(_, a), (_, b)| {
            let da = (a.0 - x).powi(2) + (a.1 - y).powi(2);
            let db = (b.0 - x).powi(2) + (b.1 - y).powi(2);
            da.partial_cmp(&db).unwrap()
        }).map(|(i, _)| i)
    }
    
}

// Bowyer–Watson algorithm for Delaunay triangulation (brute-force, O(n^3))
fn bowyer_watson(points: &[(f64, f64)]) -> Vec<Triangle> {
    let n = points.len();
    if n < 3 { return vec![]; }
    // Super-triangle that encompasses all points
    let min_x = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
    let dx = max_x - min_x;
    let dy = max_y - min_y;
    let dmax = dx.max(dy);
    let midx = (min_x + max_x) / 2.0;
    let midy = (min_y + max_y) / 2.0;
    let st = vec![
        (midx - 20.0 * dmax, midy - dmax),
        (midx, midy + 20.0 * dmax),
        (midx + 20.0 * dmax, midy - dmax),
    ];
    let mut pts = points.to_vec();
    pts.extend(st.iter().cloned());
    let mut triangles = vec![Triangle { vertices: [n, n+1, n+2] }];
    for (i, &p) in points.iter().enumerate() {
        let mut bad = vec![];
        let mut polygon = vec![];
        for (j, tri) in triangles.iter().enumerate() {
            if in_circumcircle(p, pts[tri.vertices[0]], pts[tri.vertices[1]], pts[tri.vertices[2]]) {
                bad.push(j);
                for &e in &[[0,1],[1,2],[2,0]] {
                    polygon.push((tri.vertices[e[0]], tri.vertices[e[1]]));
                }
            }
        }
        // Remove bad triangles
        for &j in bad.iter().rev() { triangles.remove(j); }
        // Remove duplicate edges (fix borrow checker)
        let unique: Vec<_> = polygon.iter()
            .filter(|&&(a, b)| {
                polygon.iter().filter(|&&(x, y)| (x, y) == (a, b) || (x, y) == (b, a)).count() == 1
            })
            .cloned()
            .collect();
        polygon = unique;
        // Add new triangles
        for &(a, b) in &polygon {
            triangles.push(Triangle { vertices: [a, b, i] });
        }
    }
    // Remove triangles with super-triangle vertices
    triangles.retain(|tri| tri.vertices.iter().all(|&v| v < n));
    triangles
}

fn in_circumcircle(p: (f64, f64), a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
    let ax = a.0 - p.0;
    let ay = a.1 - p.1;
    let bx = b.0 - p.0;
    let by = b.1 - p.1;
    let cx = c.0 - p.0;
    let cy = c.1 - p.1;
    let det = (ax * ax + ay * ay) * (bx * cy - cx * by)
        - (bx * bx + by * by) * (ax * cy - cx * ay)
        + (cx * cx + cy * cy) * (ax * by - bx * ay);
    det < 0.0
}
