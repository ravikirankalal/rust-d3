use super::quadtree_force::{Quadtree, Node as QuadtreeNode};

#[derive(Debug, Clone)]
pub struct ForceNode {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

impl ForceNode {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, vx: 0.0, vy: 0.0 }
    }
}

pub trait Force {
    fn apply(&self, nodes: &mut [ForceNode]);
}

pub struct ForceSimulation {
    pub nodes: Vec<ForceNode>,
    pub alpha: f64,
    pub alpha_min: f64,
    pub alpha_decay: f64,
    pub alpha_target: f64,
    pub velocity_decay: f64,
    pub forces: Vec<Box<dyn Force + Send + Sync>>,
}

impl ForceSimulation {
    pub fn new(nodes: Vec<ForceNode>) -> Self {
        Self {
            nodes,
            alpha: 1.0,
            alpha_min: 0.001,
            alpha_decay: 1.0 - (0.001_f64).powf(1.0 / 300.0),
            alpha_target: 0.0,
            velocity_decay: 0.6,
            forces: Vec::new(),
        }
    }

    pub fn add_force<F: Force + Send + Sync + 'static>(mut self, force: F) -> Self {
        self.forces.push(Box::new(force));
        self
    }

    pub fn alpha_target(mut self, target: f64) -> Self {
        self.alpha_target = target;
        self
    }

    pub fn alpha_min(mut self, min: f64) -> Self {
        self.alpha_min = min;
        self
    }

    pub fn alpha_decay(mut self, decay: f64) -> Self {
        self.alpha_decay = decay;
        self
    }

    pub fn velocity_decay(mut self, decay: f64) -> Self {
        self.velocity_decay = decay;
        self
    }

    pub fn tick(&mut self) {
        self.alpha += (self.alpha_target - self.alpha) * self.alpha_decay;

        if self.alpha < self.alpha_min {
            self.alpha = self.alpha_min;
        }

        for force in &self.forces {
            force.apply(&mut self.nodes);
        }

        for node in &mut self.nodes {
            node.x += node.vx * self.alpha;
            node.y += node.vy * self.alpha;

            node.vx *= 1.0 - self.velocity_decay;
            node.vy *= 1.0 - self.velocity_decay;
        }
    }
}

pub struct ManyBodyForce {
    pub strength: f64,
    pub theta: f64,
    pub distance_min: f64,
    pub distance_max: f64,
}

impl Force for ManyBodyForce {
    fn apply(&self, nodes: &mut [ForceNode]) {
        let quadtree = Quadtree::new(nodes);

        for i in 0..nodes.len() {
            let mut fx = 0.0;
            let mut fy = 0.0;
            let node_i = &nodes[i];

            quadtree.visit(&mut |quadtree_node, x0, _y0, x1, _y1| {
                match quadtree_node {
                    QuadtreeNode::Leaf(j) => {
                        if i == *j { return; } // Don't apply force to self

                        let node_j = &nodes[*j];
                        let dx = node_j.x - node_i.x;
                        let dy = node_j.y - node_i.y;
                        let mut dist_sq = dx * dx + dy * dy;

                        if dist_sq < self.distance_min * self.distance_min {
                            dist_sq = self.distance_min * self.distance_min;
                        }
                        if dist_sq > self.distance_max * self.distance_max {
                            return; // Ignore forces beyond max distance
                        }

                        let force = self.strength / dist_sq;
                        fx += force * dx;
                        fy += force * dy;
                    },
                    QuadtreeNode::Internal { x: cx, y: cy, mass: m, .. } => {
                        let s = x1 - x0; // Size of the quadrant
                        let d = ((cx - node_i.x).powi(2) + (cy - node_i.y).powi(2)).sqrt(); // Distance to center of quadrant

                        if s / d < self.theta {
                            let dx = cx - node_i.x;
                            let dy = cy - node_i.y;
                            let mut dist_sq = dx * dx + dy * dy;

                            if dist_sq < self.distance_min * self.distance_min {
                                dist_sq = self.distance_min * self.distance_min;
                            }
                            if dist_sq > self.distance_max * self.distance_max {
                                return; // Ignore forces beyond max distance
                            }

                            let force = self.strength * m / dist_sq;
                            fx += force * dx;
                            fy += force * dy;
                            return; // Stop descending
                        }
                    },
                    QuadtreeNode::Empty => {
                        return;
                    },
                }
            });
            nodes[i].vx += fx;
            nodes[i].vy += fy;
        }
    }
}

pub fn force_manybody(strength: f64) -> ManyBodyForce {
    ManyBodyForce { strength, theta: 0.9, distance_min: 1.0, distance_max: f64::INFINITY }
}

pub struct CenterForce {
    pub x: f64,
    pub y: f64,
    pub strength: f64,
}

impl Force for CenterForce {
    fn apply(&self, nodes: &mut [ForceNode]) {
        let sx = nodes.iter().map(|n| n.x).sum::<f64>();
        let sy = nodes.iter().map(|n| n.y).sum::<f64>();
        let fx = (sx / nodes.len() as f64 - self.x) * self.strength;
        let fy = (sy / nodes.len() as f64 - self.y) * self.strength;
        for node in nodes {
            node.vx -= fx;
            node.vy -= fy;
        }
    }
}

pub fn force_center(x: f64, y: f64, strength: f64) -> CenterForce {
    CenterForce { x, y, strength }
}

pub struct LinkForce<F, G> {
    pub links: Vec<(usize, usize)>,
    pub strength: F,
    pub distance: G,
    pub iterations: usize,
}

impl<F, G> Force for LinkForce<F, G>
where
    F: Fn(usize) -> f64 + Send + Sync + 'static,
    G: Fn(usize) -> f64 + Send + Sync + 'static,
{
    fn apply(&self, nodes: &mut [ForceNode]) {
        for (i, &(source_idx, target_idx)) in self.links.iter().enumerate() {
            let dx = nodes[target_idx].x - nodes[source_idx].x;
            let dy = nodes[target_idx].y - nodes[source_idx].y;
            let dist = (dx * dx + dy * dy).sqrt();
            let desired_dist = (self.distance)(i);
            let strength = (self.strength)(i);

            let force = (dist - desired_dist) / dist * strength;
            let fx = force * dx;
            let fy = force * dy;

            nodes[source_idx].vx += fx;
            nodes[source_idx].vy += fy;
            nodes[target_idx].vx -= fx;
            nodes[target_idx].vy -= fy;
        }
    }
}

pub fn force_link(links: Vec<(usize, usize)>) -> LinkForce<Box<dyn Fn(usize) -> f64 + Send + Sync>, Box<dyn Fn(usize) -> f64 + Send + Sync>> {
    LinkForce {
        links,
        strength: Box::new(|_| 1.0), // Default strength
        distance: Box::new(|_| 30.0), // Default distance
        iterations: 1, // Default iterations
    }
}

pub struct CollideForce<F> {
    pub radius: F,
    pub strength: f64,
    pub iterations: usize,
}

impl<F> Force for CollideForce<F>
where
    F: Fn(usize) -> f64 + Send + Sync + 'static,
{
    fn apply(&self, nodes: &mut [ForceNode]) {
        for _k in 0..self.iterations {
            let quadtree = Quadtree::new(nodes);
            let mut forces_to_apply: Vec<(usize, f64, f64)> = Vec::new();

            for i in 0..nodes.len() {
                let node_i = &nodes[i];
                let r_i = (self.radius)(i);
                let _r_i_sq = r_i * r_i;

                quadtree.visit(&mut |quadtree_node, x0, _y0, x1, _y1| {
                    match quadtree_node {
                        QuadtreeNode::Leaf(j) => {
                            if i == *j { return; } // Don't apply force to self

                            let node_j = &nodes[*j];
                            let r_j = (self.radius)(*j);
                            let r = r_i + r_j;
                            let r_sq = r * r;

                            let dx = node_j.x - node_i.x;
                            let dy = node_j.y - node_i.y;
                            let dist_sq = dx * dx + dy * dy;

                            if dist_sq < r_sq {
                                let dist = dist_sq.sqrt();
                                let overlap = (r - dist) / dist * self.strength;
                                let fx = dx * overlap;
                                let fy = dy * overlap;
                                forces_to_apply.push((i, -fx, -fy));
                                forces_to_apply.push((*j, fx, fy));
                            }
                        },
                        QuadtreeNode::Internal { x: cx, y: cy, .. } => {
                            let r = (self.radius)(i) + (x1 - x0).sqrt(); // Approximate radius of the quadrant
                            let r_sq = r * r;

                            let dx = cx - node_i.x;
                            let dy = cy - node_i.y;
                            let dist_sq = dx * dx + dy * dy;

                            if dist_sq >= r_sq {
                                return; // No collision possible, prune branch
                            }
                        },
                        QuadtreeNode::Empty => {
                            return;
                        },
                    }
                });
            }
            for (idx, fx, fy) in forces_to_apply {
                nodes[idx].vx += fx;
                nodes[idx].vy += fy;
            }
        }
    }
}

pub fn force_collide(radius: f64) -> CollideForce<Box<dyn Fn(usize) -> f64 + Send + Sync>> {
    CollideForce { radius: Box::new(move |_| radius), strength: 1.0, iterations: 1 }
}

pub struct XForce {
    pub x: f64,
    pub strength: f64,
}

impl Force for XForce {
    fn apply(&self, nodes: &mut [ForceNode]) {
        for node in nodes {
            node.vx += (self.x - node.x) * self.strength;
        }
    }
}

pub fn force_x(x: f64, strength: f64) -> XForce {
    XForce { x, strength }
}

pub struct YForce {
    pub y: f64,
    pub strength: f64,
}

impl Force for YForce {
    fn apply(&self, nodes: &mut [ForceNode]) {
        for node in nodes {
            node.vy += (self.y - node.y) * self.strength;
        }
    }
}

pub fn force_y(y: f64, strength: f64) -> YForce {
    YForce { y, strength }
}

pub struct RadialForce {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub strength: f64,
}

impl Force for RadialForce {
    fn apply(&self, nodes: &mut [ForceNode]) {
        for node in nodes {
            let dx = node.x - self.x;
            let dy = node.y - self.y;
            let dist = (dx * dx + dy * dy).sqrt();
            let force = (self.radius - dist) * self.strength / dist;
            node.vx += dx * force;
            node.vy += dy * force;
        }
    }
}

pub fn force_radial(x: f64, y: f64, radius: f64, strength: f64) -> RadialForce {
    RadialForce { x, y, radius, strength }
}
