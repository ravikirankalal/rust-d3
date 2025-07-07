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

pub struct ForceSimulation {
    pub nodes: Vec<ForceNode>,
    pub alpha: f64,
    pub alpha_decay: f64,
}

impl ForceSimulation {
    pub fn new(nodes: Vec<ForceNode>) -> Self {
        Self {
            nodes,
            alpha: 1.0,
            alpha_decay: 0.01,
        }
    }

    pub fn tick(&mut self) {
        for node in &mut self.nodes {
            node.x += node.vx * self.alpha;
            node.y += node.vy * self.alpha;
        }
        self.alpha *= 1.0 - self.alpha_decay;
    }
}
