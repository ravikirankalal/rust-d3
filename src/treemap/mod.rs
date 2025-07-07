//! D3 Treemap module
//! Provides treemap layout for hierarchical data (see d3-treemap in JS).

use crate::hierarchy::Node;

/// Supported treemap tiling strategies (unified, D3.js-like)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreemapTiling {
    Slice,
    Dice,
    SliceDice,
    Squarify,
    Binary,
}

/// D3.js-like treemap tiling API (unified)
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

// --- Tiling algorithms (from treemap_adv) ---
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
    let _used = 0.0;
    for (i, &weight) in ws.iter().enumerate() {
        let frac = weight / sum;
        if w >= h {
            let rw = if i == n - 1 {
                w - (x0 - x) // fill remaining width
            } else {
                let v = rem_w * frac;
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
    if (weights.len() % 2) == 0 {
        slice(weights, x, y, w, h)
    } else {
        dice(weights, x, y, w, h)
    }
}

/// Treemap layout for hierarchical data (see d3-treemap in JS).
pub struct Treemap {
    pub tiling: TreemapTiling,
    pub width: f64,
    pub height: f64,
    pub padding: f64,
}

impl Treemap {
    pub fn new() -> Self {
        Self {
            tiling: TreemapTiling::SliceDice,
            width: 1.0,
            height: 1.0,
            padding: 0.0,
        }
    }
    pub fn tiling(mut self, tiling: TreemapTiling) -> Self {
        self.tiling = tiling;
        self
    }
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }
    pub fn layout<T: Copy + Default + Into<f64>>(&self, root: &mut Node<T>) {
        match self.tiling {
            TreemapTiling::Slice => layout_slice(root, 0.0, 0.0, self.width, self.height, self.padding),
            TreemapTiling::Dice => layout_dice(root, 0.0, 0.0, self.width, self.height, self.padding),
            TreemapTiling::SliceDice => layout_slicedice(root, 0.0, 0.0, self.width, self.height, 0, self.padding),
            TreemapTiling::Squarify => layout_squarify(root, 0.0, 0.0, self.width, self.height, self.padding),
            TreemapTiling::Binary => layout_binary(root, 0.0, 0.0, self.width, self.height, self.padding),
        }
    }
}

fn layout_slice<T: Copy + Default>(node: &mut Node<T>, x0: f64, y0: f64, x1: f64, y1: f64, padding: f64) {
    node.x0 = x0;
    node.y0 = y0;
    node.x1 = x1;
    node.y1 = y1;
    let n = node.children.len();
    if n > 0 {
        let h = (y1 - y0 - padding * (n as f64 - 1.0)).max(0.0);
        let ch = h / n as f64;
        let mut y = y0;
        for child in node.children.iter_mut() {
            layout_slice(child, x0, y, x1, y + ch, padding);
            y += ch + padding;
        }
    }
}

fn layout_dice<T: Copy + Default>(node: &mut Node<T>, x0: f64, y0: f64, x1: f64, y1: f64, padding: f64) {
    node.x0 = x0;
    node.y0 = y0;
    node.x1 = x1;
    node.y1 = y1;
    let n = node.children.len();
    if n > 0 {
        let w = (x1 - x0 - padding * (n as f64 - 1.0)).max(0.0);
        let cw = w / n as f64;
        let mut x = x0;
        for child in node.children.iter_mut() {
            layout_dice(child, x, y0, x + cw, y1, padding);
            x += cw + padding;
        }
    }
}

fn layout_slicedice<T: Copy + Default>(node: &mut Node<T>, x0: f64, y0: f64, x1: f64, y1: f64, depth: usize, padding: f64) {
    node.x0 = x0;
    node.y0 = y0;
    node.x1 = x1;
    node.y1 = y1;
    let n = node.children.len();
    if n > 0 {
        if depth % 2 == 0 {
            let w = (x1 - x0 - padding * (n as f64 - 1.0)).max(0.0);
            let cw = w / n as f64;
            let mut x = x0;
            for child in node.children.iter_mut() {
                layout_slicedice(child, x, y0, x + cw, y1, depth + 1, padding);
                x += cw + padding;
            }
        } else {
            let h = (y1 - y0 - padding * (n as f64 - 1.0)).max(0.0);
            let ch = h / n as f64;
            let mut y = y0;
            for child in node.children.iter_mut() {
                layout_slicedice(child, x0, y, x1, y + ch, depth + 1, padding);
                y += ch + padding;
            }
        }
    }
}

fn layout_squarify<T: Copy + Default + Into<f64>>(node: &mut Node<T>, x0: f64, y0: f64, x1: f64, y1: f64, padding: f64) {
    node.x0 = x0;
    node.y0 = y0;
    node.x1 = x1;
    node.y1 = y1;

    let children_weights: Vec<f64> = node.children.iter().map(|c| c.value.into()).collect();
    let child_rects = squarify(&children_weights, x0, y0, x1 - x0, y1 - y0);

    for (i, child) in node.children.iter_mut().enumerate() {
        if let Some(rect) = child_rects.get(i) {
            layout_squarify(child, rect.0, rect.1, rect.0 + rect.2, rect.1 + rect.3, padding);
        }
    }
}

fn layout_binary<T: Copy + Default + Into<f64>>(node: &mut Node<T>, x0: f64, y0: f64, x1: f64, y1: f64, padding: f64) {
    node.x0 = x0;
    node.y0 = y0;
    node.x1 = x1;
    node.y1 = y1;

    let children_weights: Vec<f64> = node.children.iter().map(|c| c.value.into()).collect();
    let child_rects = binary(&children_weights, x0, y0, x1 - x0, y1 - y0);

    for (i, child) in node.children.iter_mut().enumerate() {
        if let Some(rect) = child_rects.get(i) {
            layout_binary(child, rect.0, rect.1, rect.0 + rect.2, rect.1 + rect.3, padding);
        }
    }
}
