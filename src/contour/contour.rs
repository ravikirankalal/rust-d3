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
