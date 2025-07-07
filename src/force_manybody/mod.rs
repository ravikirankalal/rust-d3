//! D3 Force ManyBody module
//! Many-body (n-body) force for force simulation.

use crate::force::ForceNode;

/// Applies a simple repulsive force between all nodes (like d3.forceManyBody).
pub fn force_manybody(nodes: &mut [ForceNode], strength: f64) {
    let n = nodes.len();
    for i in 0..n {
        let (xi, yi) = (nodes[i].x, nodes[i].y);
        let mut fx = 0.0;
        let mut fy = 0.0;
        for j in 0..n {
            if i == j { continue; }
            let dx = xi - nodes[j].x;
            let dy = yi - nodes[j].y;
            let dist2 = dx * dx + dy * dy + 1e-6; // avoid div by zero
            let f = strength / dist2;
            fx += dx * f;
            fy += dy * f;
        }
        nodes[i].vx += fx;
        nodes[i].vy += fy;
    }
}
