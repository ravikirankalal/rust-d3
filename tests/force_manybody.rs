//! Unit test for d3 force_manybody (n-body force)
use rust_d3::{force::ForceNode};
use rust_d3::force_manybody::force_manybody;

#[test]
fn test_force_manybody_repulsion() {
    let mut nodes = vec![
        ForceNode::new(0.0, 0.0),
        ForceNode::new(1.0, 0.0),
        ForceNode::new(0.0, 1.0),
    ];
    force_manybody(&mut nodes, 1.0);
    // All nodes should have nonzero velocities after repulsion
    for node in &nodes {
        assert!(node.vx.abs() > 0.0 || node.vy.abs() > 0.0);
    }
    // The velocities should be in the direction away from other nodes
    // (not checked in detail here, just that repulsion is applied)
}
