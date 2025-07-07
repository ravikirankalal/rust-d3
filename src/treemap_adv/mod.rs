//! D3 Treemap Advanced module
//! More tiling, padding, etc.

/// Supported treemap tiling strategies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreemapTiling {
    Squarify,
    Binary,
    Dice,
    Slice,
    SliceDice,
}

/// D3.js-like treemap tiling API.
pub struct TreemapTiler {
    pub tiling: TreemapTiling,
}

impl TreemapTiler {
    pub fn new(tiling: TreemapTiling) -> Self {
        Self { tiling }
    }

    /// Tile a set of nodes (weights) into a rectangle (x, y, w, h).
    /// Returns a Vec<(x, y, w, h)> for each node.
    pub fn tile(&self, weights: &[f64], x: f64, y: f64, w: f64, h: f64) -> Vec<(f64, f64, f64, f64)> {
        match self.tiling {
            TreemapTiling::Squarify => squarify(weights, x, y, w, h),
            TreemapTiling::Binary => binary(weights, x, y, w, h),
            TreemapTiling::Dice => dice(weights, x, y, w, h),
            TreemapTiling::Slice => slice(weights, x, y, w, h),
            TreemapTiling::SliceDice => slice_dice(weights, x, y, w, h),
        }
    }
}

// --- Tiling algorithms ---

fn squarify(weights: &[f64], x: f64, y: f64, w: f64, h: f64) -> Vec<(f64, f64, f64, f64)> {
    let mut rects = Vec::new();
    let sum: f64 = weights.iter().sum();
    if sum == 0.0 || weights.is_empty() { return rects; }
    let mut x0 = x;
    let mut y0 = y;
    let mut rem_w = w;
    let mut rem_h = h;
    let mut ws = weights.to_vec();
    ws.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let n = ws.len();
    let mut used = 0.0;
    for (i, &weight) in ws.iter().enumerate() {
        let frac = weight / sum;
        if w >= h {
            let rw = if i == n - 1 {
                w - (x0 - x) // fill remaining width
            } else {
                let v = rem_w * frac;
                used += v;
                v
            };
            rects.push((x0, y0, rw, rem_h));
            x0 += rw;
            rem_w -= rw;
        } else {
            let rh = if i == n - 1 {
                h - (y0 - y) // fill remaining height
            } else {
                let v = rem_h * frac;
                used += v;
                v
            };
            rects.push((x0, y0, rem_w, rh));
            y0 += rh;
            rem_h -= rh;
        }
    }
    rects
}

fn binary(weights: &[f64], x: f64, y: f64, w: f64, h: f64) -> Vec<(f64, f64, f64, f64)> {
    // Simple binary split: left/right or top/bottom
    let sum: f64 = weights.iter().sum();
    if sum == 0.0 || weights.is_empty() { return Vec::new(); }
    let mut rects = Vec::new();
    let mut offset = 0.0;
    for &weight in weights {
        let frac = weight / sum;
        if w >= h {
            let rw = w * frac;
            rects.push((x + offset, y, rw, h));
            offset += rw;
        } else {
            let rh = h * frac;
            rects.push((x, y + offset, w, rh));
            offset += rh;
        }
    }
    rects
}

fn dice(weights: &[f64], x: f64, y: f64, w: f64, h: f64) -> Vec<(f64, f64, f64, f64)> {
    // Horizontal strips
    let sum: f64 = weights.iter().sum();
    if sum == 0.0 || weights.is_empty() { return Vec::new(); }
    let mut rects = Vec::new();
    let mut offset = 0.0;
    for &weight in weights {
        let frac = weight / sum;
        let rw = w * frac;
        rects.push((x + offset, y, rw, h));
        offset += rw;
    }
    rects
}

fn slice(weights: &[f64], x: f64, y: f64, w: f64, h: f64) -> Vec<(f64, f64, f64, f64)> {
    // Vertical strips
    let sum: f64 = weights.iter().sum();
    if sum == 0.0 || weights.is_empty() { return Vec::new(); }
    let mut rects = Vec::new();
    let mut offset = 0.0;
    for &weight in weights {
        let frac = weight / sum;
        let rh = h * frac;
        rects.push((x, y + offset, w, rh));
        offset += rh;
    }
    rects
}

fn slice_dice(weights: &[f64], x: f64, y: f64, w: f64, h: f64) -> Vec<(f64, f64, f64, f64)> {
    // Alternate between slice and dice
    if (weights.len() % 2) == 0 {
        slice(weights, x, y, w, h)
    } else {
        dice(weights, x, y, w, h)
    }
}
