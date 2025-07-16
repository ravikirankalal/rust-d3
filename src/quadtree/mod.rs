//! d3-quadtree: Quadtree spatial index (Rust port, minimal)

#[derive(Debug)]
pub enum Node<T> {
    Leaf(Vec<(f64, f64, T)>),
    Internal(Box<[Option<Node<T>>; 4]>),
}

#[derive(Debug)]
pub struct Quadtree<T> {
    pub bounds: (f64, f64, f64, f64),
    pub root: Option<Node<T>>,
    pub bucket_size: usize,
}

impl<T> Quadtree<T> {
    pub fn new(bounds: (f64, f64, f64, f64)) -> Self {
        Self {
            bounds,
            root: None,
            bucket_size: 4,
        }
    }
    pub fn clear(&mut self) {
        self.root = None;
    }
    pub fn insert(&mut self, x: f64, y: f64, value: T) {
        let bounds = self.bounds;
        let bucket_size = self.bucket_size;
        let node = self.root.take();
        self.root = Some(Self::insert_rec(node, bounds, x, y, value, bucket_size));
    }
    fn insert_rec(
        node: Option<Node<T>>,
        bounds: (f64, f64, f64, f64),
        x: f64,
        y: f64,
        value: T,
        bucket_size: usize,
    ) -> Node<T> {
        match node {
            None => Node::Leaf(vec![(x, y, value)]),
            Some(Node::Leaf(mut pts)) => {
                if pts.len() < bucket_size {
                    pts.push((x, y, value));
                    Node::Leaf(pts)
                } else {
                    // Subdivide
                    let mut children: Box<[Option<Node<T>>; 4]> =
                        Box::new([None, None, None, None]);
                    for (px, py, pv) in pts.drain(..) {
                        let idx = Self::child_index(bounds, px, py);
                        let child_bounds = Self::child_bounds(bounds, idx);
                        children[idx] = Some(Self::insert_rec(
                            children[idx].take(),
                            child_bounds,
                            px,
                            py,
                            pv,
                            bucket_size,
                        ));
                    }
                    let idx = Self::child_index(bounds, x, y);
                    let child_bounds = Self::child_bounds(bounds, idx);
                    children[idx] = Some(Self::insert_rec(
                        children[idx].take(),
                        child_bounds,
                        x,
                        y,
                        value,
                        bucket_size,
                    ));
                    Node::Internal(children)
                }
            }
            Some(Node::Internal(mut children)) => {
                let idx = Self::child_index(bounds, x, y);
                let child_bounds = Self::child_bounds(bounds, idx);
                children[idx] = Some(Self::insert_rec(
                    children[idx].take(),
                    child_bounds,
                    x,
                    y,
                    value,
                    bucket_size,
                ));
                Node::Internal(children)
            }
        }
    }
    fn child_index((x0, y0, x1, y1): (f64, f64, f64, f64), x: f64, y: f64) -> usize {
        let xm = (x0 + x1) / 2.0;
        let ym = (y0 + y1) / 2.0;
        match (x >= xm, y >= ym) {
            (false, false) => 0, // NW
            (true, false) => 1,  // NE
            (false, true) => 2,  // SW
            (true, true) => 3,   // SE
        }
    }
    fn child_bounds((x0, y0, x1, y1): (f64, f64, f64, f64), idx: usize) -> (f64, f64, f64, f64) {
        let xm = (x0 + x1) / 2.0;
        let ym = (y0 + y1) / 2.0;
        match idx {
            0 => (x0, y0, xm, ym), // NW
            1 => (xm, y0, x1, ym), // NE
            2 => (x0, ym, xm, y1), // SW
            3 => (xm, ym, x1, y1), // SE
            _ => (x0, y0, x1, y1),
        }
    }
    pub fn len(&self) -> usize {
        fn count<T>(node: &Option<Node<T>>) -> usize {
            match node {
                None => 0,
                Some(Node::Leaf(pts)) => pts.len(),
                Some(Node::Internal(children)) => children.iter().map(|c| count(c)).sum(),
            }
        }
        count(&self.root)
    }
    pub fn visit<F: FnMut(&(f64, f64, T))>(&self, mut f: F) {
        fn visit_rec<T, F: FnMut(&(f64, f64, T))>(node: &Option<Node<T>>, f: &mut F) {
            match node {
                None => (),
                Some(Node::Leaf(pts)) => {
                    for p in pts {
                        f(p);
                    }
                }
                Some(Node::Internal(children)) => {
                    for c in children.iter() {
                        visit_rec(c, f);
                    }
                }
            }
        }
        visit_rec(&self.root, &mut f);
    }
    pub fn find<'a>(&'a self, x: f64, y: f64, radius: f64) -> Option<&'a (f64, f64, T)> {
        fn find_rec<'a, T>(
            node: &'a Option<Node<T>>,
            x: f64,
            y: f64,
            r2: f64,
        ) -> Option<&'a (f64, f64, T)> {
            match node {
                None => None,
                Some(Node::Leaf(pts)) => pts.iter().find(|(px, py, _)| {
                    let dx = px - x;
                    let dy = py - y;
                    dx * dx + dy * dy <= r2
                }),
                Some(Node::Internal(children)) => {
                    for c in children.iter() {
                        if let Some(found) = find_rec(c, x, y, r2) {
                            return Some(found);
                        }
                    }
                    None
                }
            }
        }
        find_rec(&self.root, x, y, radius * radius)
    }
    pub fn remove(&mut self, x: f64, y: f64, radius: f64) -> Option<T> {
        fn remove_rec<T>(node: &mut Option<Node<T>>, x: f64, y: f64, r2: f64) -> Option<T> {
            match node {
                None => None,
                Some(Node::Leaf(pts)) => {
                    if let Some(idx) = pts.iter().position(|(px, py, _)| {
                        let dx = px - x;
                        let dy = py - y;
                        dx * dx + dy * dy <= r2
                    }) {
                        Some(pts.remove(idx).2)
                    } else {
                        None
                    }
                }
                Some(Node::Internal(children)) => {
                    for c in children.iter_mut() {
                        if let Some(val) = remove_rec(c, x, y, r2) {
                            return Some(val);
                        }
                    }
                    None
                }
            }
        }
        remove_rec(&mut self.root, x, y, radius * radius)
    }
    pub fn query_range<'a>(&'a self, bounds: (f64, f64, f64, f64)) -> Vec<&'a (f64, f64, T)> {
        fn query_rec<'a, T>(
            node: &'a Option<Node<T>>,
            node_bounds: (f64, f64, f64, f64),
            bounds: (f64, f64, f64, f64),
            out: &mut Vec<&'a (f64, f64, T)>,
        ) {
            if !rects_intersect(node_bounds, bounds) {
                return;
            }
            match node {
                None => (),
                Some(Node::Leaf(pts)) => {
                    for p in pts {
                        if point_in_rect((p.0, p.1), bounds) {
                            out.push(p);
                        }
                    }
                }
                Some(Node::Internal(children)) => {
                    for (i, c) in children.iter().enumerate() {
                        let child_bounds = Quadtree::<T>::child_bounds(node_bounds, i);
                        query_rec(c, child_bounds, bounds, out);
                    }
                }
            }
        }
        fn rects_intersect(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
            a.0 < b.2 && a.2 > b.0 && a.1 < b.3 && a.3 > b.1
        }
        fn point_in_rect(p: (f64, f64), r: (f64, f64, f64, f64)) -> bool {
            p.0 >= r.0 && p.0 < r.2 && p.1 >= r.1 && p.1 < r.3
        }
        let mut out = Vec::new();
        query_rec(&self.root, self.bounds, bounds, &mut out);
        out
    }
}

#[cfg(test)]
mod tests;
