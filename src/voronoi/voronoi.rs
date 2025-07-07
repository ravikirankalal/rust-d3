use std::f64;

#[derive(Debug, Clone)]
pub struct VoronoiCell {
    pub site: (f64, f64),
    pub region: Vec<(f64, f64)>,
}

pub struct VoronoiDiagram {
    pub cells: Vec<VoronoiCell>,
}

impl VoronoiDiagram {
    pub fn new(sites: &[(f64, f64)]) -> Self {
        // Brute-force Voronoi: for each site, assign each grid point to the closest site
        // and compute convex hull of those points as the region. For simplicity, use a grid.
        let mut cells = Vec::new();
        let n = sites.len();
        if n == 0 {
            return Self { cells };
        }
        // Bounding box
        let (min_x, max_x, min_y, max_y) = sites.iter().fold(
            (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
            |(min_x, max_x, min_y, max_y), &(x, y)| {
                (
                    min_x.min(x),
                    max_x.max(x),
                    min_y.min(y),
                    max_y.max(y),
                )
            },
        );
        let _step = ((max_x - min_x).max(max_y - min_y) / 50.0).max(1e-3); // 50x50 grid
        let mut regions: Vec<Vec<(f64, f64)>> = vec![vec![]; n];
        for gx in 0..=50 {
            for gy in 0..=50 {
                let x = min_x + (max_x - min_x) * gx as f64 / 50.0;
                let y = min_y + (max_y - min_y) * gy as f64 / 50.0;
                let mut min_dist = f64::INFINITY;
                let mut min_idx = 0;
                for (i, &(sx, sy)) in sites.iter().enumerate() {
                    let d = (sx - x).powi(2) + (sy - y).powi(2);
                    if d < min_dist {
                        min_dist = d;
                        min_idx = i;
                    }
                }
                regions[min_idx].push((x, y));
            }
        }
        // For each region, compute convex hull as the cell boundary
        for (i, pts) in regions.into_iter().enumerate() {
            let hull = convex_hull(&pts);
            cells.push(VoronoiCell {
                site: sites[i],
                region: hull,
            });
        }
        Self { cells }
    }

    /// Find the cell whose site is closest to (x, y)
    pub fn find(&self, x: f64, y: f64) -> Option<&VoronoiCell> {
        self.cells.iter().min_by(|a, b| {
            let da = (a.site.0 - x).powi(2) + (a.site.1 - y).powi(2);
            let db = (b.site.0 - x).powi(2) + (b.site.1 - y).powi(2);
            da.partial_cmp(&db).unwrap()
        })
    }

    /// Return the polygons for each cell
    pub fn cell_polygons(&self) -> Vec<Vec<(f64, f64)>> {
        self.cells.iter().map(|c| c.region.clone()).collect()
    }

    /// Render: print cell polygons to stdout (for demo)
    pub fn render(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            println!("Cell {}: site={:?} region={:?}", i, cell.site, cell.region);
        }
    }
}

// Graham scan convex hull for 2D points
fn convex_hull(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut pts = points.to_vec();
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.partial_cmp(&b.1).unwrap()));
    let n = pts.len();
    if n < 3 {
        return pts;
    }
    let mut lower = Vec::new();
    for &p in &pts {
        while lower.len() >= 2 && cross(lower[lower.len()-2], lower[lower.len()-1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len()-2], upper[upper.len()-1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn cross(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}
