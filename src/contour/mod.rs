// src/contour/mod.rs

pub struct ContourGenerator {
    size: Option<[usize; 2]>,
    thresholds: Option<Vec<f64>>,
    // smooth: bool, // Not implementing smooth for now, as it's a rendering detail
    // value: Option<Box<dyn Fn(&[f64], usize, usize) -> f64>>, // Not implementing custom value accessor for now
}

impl ContourGenerator {
    pub fn new() -> Self {
        ContourGenerator {
            size: None,
            thresholds: None,
        }
    }

    pub fn size(mut self, s: [usize; 2]) -> Self {
        self.size = Some(s);
        self
    }

    pub fn thresholds(mut self, t: Vec<f64>) -> Self {
        self.thresholds = Some(t);
        self
    }

    pub fn contours(&self, data: &[f64]) -> Vec<Vec<[f64; 2]>> {
        let size = self.size.expect("Size must be set for contour generation.");
        let thresholds = self.thresholds.as_ref().expect("Thresholds must be set for contour generation.");

        let nx = size[0];
        let ny = size[1];

        let mut all_contours: Vec<Vec<[f64; 2]>> = Vec::new();

        // Marching squares lookup table
        // Each entry is a list of pairs of indices representing the edges to draw.
        // The indices correspond to the midpoints of the edges:
        // 0: top edge midpoint (0.5, 0)
        // 1: right edge midpoint (1, 0.5)
        // 2: bottom edge midpoint (0.5, 1)
        // 3: left edge midpoint (0, 0.5)
        const CASES: [[(usize, usize); 2]; 16] = [
            [(0, 0), (0, 0)], // 0: 0000
            [(3, 0), (0, 0)], // 1: 0001
            [(1, 0), (0, 0)], // 2: 0010
            [(3, 1), (0, 0)], // 3: 0011
            [(2, 1), (0, 0)], // 4: 0100
            [(3, 0), (2, 1)], // 5: 0101
            [(2, 0), (0, 0)], // 6: 0110
            [(3, 2), (0, 0)], // 7: 0111
            [(0, 3), (0, 0)], // 8: 1000
            [(0, 2), (0, 0)], // 9: 1001
            [(0, 3), (1, 2)], // 10: 1010
            [(0, 1), (0, 0)], // 11: 1011
            [(0, 1), (0, 0)], // 12: 1100
            [(0, 3), (0, 0)], // 13: 1101
            [(0, 1), (0, 0)], // 14: 1110
            [(0, 0), (0, 0)], // 15: 1111
        ];

        // Function to interpolate a line segment
        let line_interpolate = |p1: f64, p2: f64, threshold: f64| -> f64 {
            if (p1 - p2).abs() < 1e-6 { 0.5 } else { (threshold - p1) / (p2 - p1) }
        };

        for &threshold in thresholds.iter() {
            let mut contour_segments: Vec<Vec<[f64; 2]>> = Vec::new();

            for y in 0..ny - 1 {
                for x in 0..nx - 1 {
                    let p00 = data[y * nx + x];
                    let p10 = data[y * nx + x + 1];
                    let p01 = data[(y + 1) * nx + x];
                    let p11 = data[(y + 1) * nx + x + 1];

                    let case = ((p00 > threshold) as u8) << 3
                                 | ((p10 > threshold) as u8) << 2
                                 | ((p11 > threshold) as u8) << 1
                                 | ((p01 > threshold) as u8);

                    let segments_indices = CASES[case as usize];

                    // Interpolate points for each segment
                    for segment_pair in segments_indices.iter() {
                        if segment_pair.0 == 0 && segment_pair.1 == 0 { continue; } // Skip empty segments

                        let p_start_idx = segment_pair.0;
                        let p_end_idx = segment_pair.1;

                        let start_point = match p_start_idx {
                            0 => [x as f64 + line_interpolate(p00, p10, threshold), y as f64 + 0.0],
                            1 => [x as f64 + 1.0, y as f64 + line_interpolate(p10, p11, threshold)],
                            2 => [x as f64 + line_interpolate(p01, p11, threshold), y as f64 + 1.0],
                            3 => [x as f64 + 0.0, y as f64 + line_interpolate(p00, p01, threshold)],
                            _ => unreachable!(),
                        };

                        let end_point = match p_end_idx {
                            0 => [x as f64 + line_interpolate(p00, p10, threshold), y as f64 + 0.0],
                            1 => [x as f64 + 1.0, y as f64 + line_interpolate(p10, p11, threshold)],
                            2 => [x as f64 + line_interpolate(p01, p11, threshold), y as f64 + 1.0],
                            3 => [x as f64 + 0.0, y as f64 + line_interpolate(p00, p01, threshold)],
                            _ => unreachable!(),
                        };

                        // Instead of pushing both points into a flat vector, push as a polyline (segment)
                        contour_segments.push(vec![start_point, end_point]);
                    }
                }
            }
            if !contour_segments.is_empty() {
                all_contours.extend(contour_segments);
            }
        }
        all_contours
    }
}