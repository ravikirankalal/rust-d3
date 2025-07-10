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

        // Marching squares lookup table (simplified for lines, not polygons)
        // Each entry represents the segments for a given case (0-15)
        // The coordinates are relative to the cell (0,0) to (1,1)
        // 0: no segments
        // 1: bottom-left corner (0,0.5)-(0.5,0)
        // 2: bottom-right corner (0.5,0)-(1,0.5)
        // 3: bottom edge (0,0.5)-(1,0.5)
        // 4: top-right corner (0.5,1)-(1,0.5)
        // 5: two segments (0,0.5)-(0.5,0) and (0.5,1)-(1,0.5)
        // 6: right edge (0.5,0)-(0.5,1)
        // 7: three segments (0,0.5)-(1,0.5) and (0.5,1)-(0.5,0)
        // 8: top-left corner (0,0.5)-(0.5,1)
        // 9: left edge (0,0.5)-(0.5,1)
        // 10: two segments (0.5,0)-(1,0.5) and (0,0.5)-(0.5,1)
        // 11: three segments (0,0.5)-(1,0.5) and (0.5,0)-(0.5,1)
        // 12: top edge (0.5,0)-(0.5,1)
        // 13: three segments (0,0.5)-(0.5,0) and (0.5,1)-(0.5,0)
        // 14: three segments (0.5,0)-(1,0.5) and (0,0.5)-(0.5,1)
        // 15: no segments (all inside or all outside)
        let polygon_case_lookup: [[(f64, f64); 2]; 16] = [
            // Case 0: 0000 (no segments)
            [(0.0, 0.0), (0.0, 0.0)],
            // Case 1: 0001 (bottom-left)
            [(0.0, 0.5), (0.5, 0.0)],
            // Case 2: 0010 (bottom-right)
            [(0.5, 0.0), (1.0, 0.5)],
            // Case 3: 0011 (bottom edge)
            [(0.0, 0.5), (1.0, 0.5)],
            // Case 4: 0100 (top-right)
            [(0.5, 1.0), (1.0, 0.5)],
            // Case 5: 0101 (two segments)
            [(0.0, 0.5), (0.5, 0.0)], // Segment 1
            // Case 6: 0110 (right edge)
            [(0.5, 0.0), (0.5, 1.0)],
            // Case 7: 0111 (three segments)
            [(0.0, 0.5), (0.5, 1.0)],
            // Case 8: 1000 (top-left)
            [(0.0, 0.5), (0.5, 1.0)],
            // Case 9: 1001 (left edge)
            [(0.0, 0.5), (0.5, 1.0)],
            // Case 10: 1010 (two segments)
            [(0.5, 0.0), (1.0, 0.5)], // Segment 1
            // Case 11: 1011 (three segments)
            [(0.0, 0.5), (1.0, 0.5)],
            // Case 12: 1100 (top edge)
            [(0.5, 0.0), (0.5, 1.0)],
            // Case 13: 1101 (three segments)
            [(0.0, 0.5), (0.5, 0.0)],
            // Case 14: 1110 (three segments)
            [(0.5, 0.0), (1.0, 0.5)],
            // Case 15: 1111 (no segments)
            [(0.0, 0.0), (0.0, 0.0)],
        ];

        // Function to interpolate a line segment
        let line_interpolate = |p1: f64, p2: f64, threshold: f64| -> f64 {
            (threshold - p1) / (p2 - p1)
        };

        for &threshold in thresholds.iter() {
            let mut contour_segments: Vec<[f64; 2]> = Vec::new();

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

                    // Get segments from lookup table
                    let _segments = polygon_case_lookup[case as usize];

                    // Interpolate and add segments
                    if case != 0 && case != 15 {
                        // This is a simplified interpolation and segment generation
                        // A full implementation would handle all 16 cases and their specific segment configurations
                        // and interpolate points precisely.
                        let x_offset = x as f64;
                        let y_offset = y as f64;

                        // Example for case 1 (0001): (0,0.5)-(0.5,0)
                        if case == 1 {
                            let y_interp = line_interpolate(p00, p01, threshold);
                            let x_interp = line_interpolate(p00, p10, threshold);
                            contour_segments.push([x_offset + 0.0, y_offset + y_interp]);
                            contour_segments.push([x_offset + x_interp, y_offset + 0.0]);
                        }
                        // Add more cases here
                    }
                }
            }
            if !contour_segments.is_empty() {
                all_contours.push(contour_segments);
            }
        }
        all_contours
    }
}