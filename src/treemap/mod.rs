//! D3 Treemap module
//! Provides treemap layout for hierarchical data (see d3-treemap in JS).

use crate::hierarchy::Node;

#[derive(Clone, Copy)]
pub enum TreemapTiling {
    Slice,
    Dice,
    SliceDice,
}

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
    pub fn layout<T: Copy + Default>(&self, root: &mut Node<T>) {
        match self.tiling {
            TreemapTiling::Slice => layout_slice(root, 0.0, 0.0, self.width, self.height, self.padding),
            TreemapTiling::Dice => layout_dice(root, 0.0, 0.0, self.width, self.height, self.padding),
            TreemapTiling::SliceDice => layout_slicedice(root, 0.0, 0.0, self.width, self.height, 0, self.padding),
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
