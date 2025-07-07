use super::ForceNode;

const X_MIN: f64 = -f64::INFINITY;
const Y_MIN: f64 = -f64::INFINITY;
const X_MAX: f64 = f64::INFINITY;
const Y_MAX: f64 = f64::INFINITY;

#[derive(Debug, Clone)]
pub enum Node {
    Internal {
        children: Box<[Option<Node>; 4]>,
        x: f64, // Center of mass x
        y: f64, // Center of mass y
        mass: f64, // Total mass
    }, // Children: [top_left, top_right, bottom_left, bottom_right]
    Leaf(usize), // Index of the node in the simulation's nodes vector
    Empty,
}

pub struct Quadtree {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub root: Node,
}

impl Quadtree {
    pub fn new(nodes: &[ForceNode]) -> Self {
        let mut x0 = X_MAX;
        let mut y0 = Y_MAX;
        let mut x1 = X_MIN;
        let mut y1 = Y_MIN;

        for node in nodes {
            if node.x < x0 { x0 = node.x; }
            if node.y < y0 { y0 = node.y; }
            if node.x > x1 { x1 = node.x; }
            if node.y > y1 { y1 = node.y; }
        }

        // Expand bounds slightly if all nodes are collinear or a single point
        if x1 - x0 < 1e-6 { x0 -= 1.0; x1 += 1.0; }
        if y1 - y0 < 1e-6 { y0 -= 1.0; y1 += 1.0; }

        let mut tree = Quadtree {
            x0,
            y0,
            x1,
            y1,
            root: Node::Empty,
        };

        for (i, _node) in nodes.iter().enumerate() {
            tree.add(i, nodes);
        }
        tree
    }

    pub fn add(&mut self, i: usize, nodes: &[ForceNode]) {
        self.root = Self::add_recursive(self.root.clone(), i, nodes, self.x0, self.y0, self.x1, self.y1);
    }

    fn add_recursive(node: Node, i: usize, nodes: &[ForceNode], x0: f64, y0: f64, x1: f64, y1: f64) -> Node {
        match node {
            Node::Empty => Node::Leaf(i),
            Node::Leaf(j) => {
                let mut children = Box::new([None, None, None, None]);
                let xm = (x0 + x1) / 2.0;
                let ym = (y0 + y1) / 2.0;

                let node_i = &nodes[i];
                let node_j = &nodes[j];

                let q_i = Self::get_quadrant(node_i, xm, ym);
                let q_j = Self::get_quadrant(node_j, xm, ym);

                children[q_j] = Some(Self::add_recursive(Node::Empty, j, nodes, x0, y0, x1, y1));
                children[q_i] = Some(Self::add_recursive(Node::Empty, i, nodes, x0, y0, x1, y1));

                let (x, y, mass) = Self::calculate_internal_node_properties(&children, nodes);
                Node::Internal { children, x, y, mass }
            },
            Node::Internal { mut children, x: _, y: _, mass: _ } => {
                let xm = (x0 + x1) / 2.0;
                let ym = (y0 + y1) / 2.0;
                let node_i = &nodes[i];
                let quadrant = Self::get_quadrant(node_i, xm, ym);
                children[quadrant] = Some(Self::add_recursive(children[quadrant].take().unwrap_or(Node::Empty), i, nodes, x0, y0, x1, y1));

                let (x, y, mass) = Self::calculate_internal_node_properties(&children, nodes);
                Node::Internal { children, x, y, mass }
            },
        }
    }

    fn calculate_internal_node_properties(children: &Box<[Option<Node>; 4]>, nodes: &[ForceNode]) -> (f64, f64, f64) {
        let mut total_x = 0.0;
        let mut total_y = 0.0;
        let mut total_mass = 0.0;

        for child_option in children.iter() {
            if let Some(child) = child_option {
                match child {
                    Node::Leaf(idx) => {
                        total_x += nodes[*idx].x;
                        total_y += nodes[*idx].y;
                        total_mass += 1.0; // Assuming each node has a mass of 1
                    },
                    Node::Internal { x, y, mass, .. } => {
                        total_x += x * mass;
                        total_y += y * mass;
                        total_mass += mass;
                    },
                    Node::Empty => {},
                }
            }
        }
        if total_mass == 0.0 { (0.0, 0.0, 0.0) } else { (total_x / total_mass, total_y / total_mass, total_mass) }
    }

    pub fn visit<F>(&self, callback: &mut F) where F: FnMut(&Node, f64, f64, f64, f64) {
        Self::visit_recursive(&self.root, callback, self.x0, self.y0, self.x1, self.y1);
    }

    fn visit_recursive<F>(node: &Node, callback: &mut F, x0: f64, y0: f64, x1: f64, y1: f64) where F: FnMut(&Node, f64, f64, f64, f64) {
        callback(node, x0, y0, x1, y1);
        if let Node::Internal { children, .. } = node {
            let xm = (x0 + x1) / 2.0;
            let ym = (y0 + y1) / 2.0;
            if let Some(child) = &children[0] { Self::visit_recursive(child, callback, x0, y0, xm, ym); }
            if let Some(child) = &children[1] { Self::visit_recursive(child, callback, xm, y0, x1, ym); }
            if let Some(child) = &children[2] { Self::visit_recursive(child, callback, x0, ym, xm, y1); }
            if let Some(child) = &children[3] { Self::visit_recursive(child, callback, xm, ym, x1, y1); }
        }
    }

    fn get_quadrant(node: &ForceNode, xm: f64, ym: f64) -> usize {
        if node.x < xm {
            if node.y < ym { 0 } else { 2 } // Top-left or Bottom-left
        } else {
            if node.y < ym { 1 } else { 3 } // Top-right or Bottom-right
        }
    }
}