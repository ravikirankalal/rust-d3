//! D3 Contour module
//! Implements marching squares for contour line generation.

#[derive(Debug, Clone, PartialEq)]
pub struct ContourLine {
    pub points: Vec<(f64, f64)>,
    pub value: f64,
}

/// Generate contour lines for a 2D scalar field using marching squares.
pub fn contours(
    values: &[Vec<f64>],
    threshold: f64,
) -> Vec<ContourLine> {
    let rows = values.len();
    if rows == 0 { return vec![]; }
    let cols = values[0].len();
    let mut lines = Vec::new();
    for y in 0..rows-1 {
        for x in 0..cols-1 {
            let square = [
                values[y][x],
                values[y][x+1],
                values[y+1][x+1],
                values[y+1][x],
            ];
            let mut code = 0;
            for (i, &v) in square.iter().enumerate() {
                if v >= threshold { code |= 1 << i; }
            }
            // Handle all 16 cases for marching squares
            let mut points = Vec::new();
            match code {
                0 | 15 => {}, // No contour
                1 | 14 => {
                    points.push((x as f64, y as f64 + interp(square[0], square[3], threshold)));
                    points.push((x as f64 + interp(square[0], square[1], threshold), y as f64));
                },
                2 | 13 => {
                    points.push((x as f64 + interp(square[0], square[1], threshold), y as f64));
                    points.push((x as f64 + 1.0, y as f64 + interp(square[1], square[2], threshold)));
                },
                3 | 12 => {
                    points.push((x as f64, y as f64 + interp(square[0], square[3], threshold)));
                    points.push((x as f64 + 1.0, y as f64 + interp(square[1], square[2], threshold)));
                },
                4 | 11 => {
                    points.push((x as f64 + 1.0, y as f64 + interp(square[1], square[2], threshold)));
                    points.push((x as f64 + 1.0 - interp(square[2], square[3], threshold), y as f64 + 1.0));
                },
                5 => {
                    points.push((x as f64, y as f64 + interp(square[0], square[3], threshold)));
                    points.push((x as f64 + interp(square[0], square[1], threshold), y as f64));
                    points.push((x as f64 + 1.0, y as f64 + interp(square[1], square[2], threshold)));
                    points.push((x as f64 + 1.0 - interp(square[2], square[3], threshold), y as f64 + 1.0));
                },
                6 | 9 => {
                    points.push((x as f64 + interp(square[0], square[1], threshold), y as f64));
                    points.push((x as f64 + 1.0 - interp(square[2], square[3], threshold), y as f64 + 1.0));
                },
                7 | 8 => {
                    points.push((x as f64, y as f64 + interp(square[0], square[3], threshold)));
                    points.push((x as f64 + 1.0 - interp(square[2], square[3], threshold), y as f64 + 1.0));
                },
                10 => {
                    points.push((x as f64, y as f64 + interp(square[0], square[3], threshold)));
                    points.push((x as f64 + 1.0, y as f64 + interp(square[1], square[2], threshold)));
                },
                _ => {},
            }
            if !points.is_empty() {
                lines.push(ContourLine { points, value: threshold });
            }
        }
    }
    lines
}

fn interp(v0: f64, v1: f64, threshold: f64) -> f64 {
    if (v1 - v0).abs() < 1e-12 { 0.0 } else { (threshold - v0) / (v1 - v0) }
}

// New ContourDensity implementation
pub struct ContourDensity<T> {
    x_accessor: Box<dyn Fn(&T) -> f64>,
    y_accessor: Box<dyn Fn(&T) -> f64>,
    weight_accessor: Box<dyn Fn(&T) -> f64>,
    size: (usize, usize),
    cell_size: usize,
    thresholds: Vec<f64>,
    bandwidth: f64,
}

impl<T: 'static> ContourDensity<T> {
    pub fn new() -> Self {
        Self {
            x_accessor: Box::new(|d: &T| {
                let point = d as *const T as *const (f64, f64);
                unsafe { (*point).0 }
            }),
            y_accessor: Box::new(|d: &T| {
                let point = d as *const T as *const (f64, f64);
                unsafe { (*point).1 }
            }),
            weight_accessor: Box::new(|_: &T| 1.0),
            size: (960, 500), // Default size
            cell_size: 4,    // Default cell size
            thresholds: vec![], // Default empty, will be generated if not set
            bandwidth: 20.0, // Default bandwidth
        }
    }

    pub fn x<A>(mut self, accessor: A) -> Self
    where
        A: Fn(&T) -> f64 + 'static,
    {
        self.x_accessor = Box::new(accessor);
        self
    }

    pub fn y<A>(mut self, accessor: A) -> Self
    where
        A: Fn(&T) -> f64 + 'static,
    {
        self.y_accessor = Box::new(accessor);
        self
    }

    pub fn weight<A>(mut self, accessor: A) -> Self
    where
        A: Fn(&T) -> f64 + 'static,
    {
        self.weight_accessor = Box::new(accessor);
        self
    }

    pub fn size(mut self, size: (usize, usize)) -> Self {
        self.size = size;
        self
    }

    pub fn cell_size(mut self, cell_size: usize) -> Self {
        self.cell_size = cell_size;
        self
    }

    pub fn thresholds(mut self, thresholds: Vec<f64>) -> Self {
        self.thresholds = thresholds;
        self
    }

    pub fn bandwidth(mut self, bandwidth: f64) -> Self {
        self.bandwidth = bandwidth;
        self
    }

    pub fn contours(&self, data: &[T]) -> Vec<ContourLine> {
        let nx = (self.size.0 as f64 / self.cell_size as f64).ceil() as usize;
        let ny = (self.size.1 as f64 / self.cell_size as f64).ceil() as usize;

        let mut grid = vec![vec![0.0; nx]; ny];

        // Simple density accumulation (not full KDE)
        for d in data {
            let x = (self.x_accessor)(d);
            let y = (self.y_accessor)(d);
            let weight = (self.weight_accessor)(d);

            let gx = (x / self.cell_size as f64).floor() as usize;
            let gy = (y / self.cell_size as f64).floor() as usize;

            if gx < nx && gy < ny {
                grid[gy][gx] += weight;
            }
        }

        let mut all_lines = Vec::new();
        let thresholds_to_use = if self.thresholds.is_empty() {
            // Generate some default thresholds if not provided
            vec![1.0, 2.0, 3.0, 4.0, 5.0]
        } else {
            self.thresholds.clone()
        };

        for &threshold in &thresholds_to_use {
            all_lines.extend(contours(&grid, threshold));
        }
        all_lines
    }
}

pub fn contour_density<T: 'static>() -> ContourDensity<T> {
    ContourDensity::new()
}
