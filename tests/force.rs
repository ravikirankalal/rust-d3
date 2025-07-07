//! Unit tests for d3 ForceSimulation
use rust_d3::force::{ForceNode, ForceSimulation};

#[test]
fn test_force_simulation() {
    let mut sim = ForceSimulation::new(vec![ForceNode::new(0.0, 0.0)]);
    sim.nodes[0].vx = 1.0;
    sim.nodes[0].vy = 2.0;
    sim.tick();
    assert!((sim.nodes[0].x - 1.0).abs() < 1e-6);
    assert!((sim.nodes[0].y - 2.0).abs() < 1e-6);
    let alpha_after = sim.alpha;
    sim.tick();
    assert!(sim.alpha < alpha_after);
}
