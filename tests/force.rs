//! Unit tests for d3 force layout

use rust_d3::force::{ForceNode, ForceSimulation, force_manybody, force_center, force_link, force_collide, force_x, force_y, force_radial};

#[test]
fn test_force_simulation_alpha_decay() {
    let nodes = vec![
        ForceNode::new(0.0, 0.0),
    ];
    let mut simulation = ForceSimulation::new(nodes);
    simulation.tick();
    assert!((simulation.alpha - (1.0 + (simulation.alpha_target - 1.0) * simulation.alpha_decay)).abs() < 1e-9);
}

#[test]
fn test_force_simulation_alpha_target() {
    let nodes = vec![
        ForceNode::new(0.0, 0.0),
    ];
    let mut simulation = ForceSimulation::new(nodes).alpha_target(0.5);
    simulation.tick();
    assert!((simulation.alpha - (1.0 + (0.5 - 1.0) * simulation.alpha_decay)).abs() < 1e-9);
}

#[test]
fn test_force_simulation_velocity_decay() {
    let nodes = vec![
        ForceNode::new(0.0, 0.0),
    ];
    let mut simulation = ForceSimulation::new(nodes);
    simulation.nodes[0].vx = 1.0;
    simulation.nodes[0].vy = 1.0;
    simulation.tick();
    assert!((simulation.nodes[0].vx - (1.0 * (1.0 - simulation.velocity_decay))).abs() < 1e-9);
}

#[test]
fn test_force_manybody_two_nodes() {
    let nodes = vec![
        ForceNode::new(0.0, 0.0),
        ForceNode::new(1.0, 0.0),
    ];
    let initial_dist = (nodes[1].x - nodes[0].x).abs();
    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_manybody(-30.0));
    for _ in 0..200 {
        simulation.tick();
    }
    let final_dist = (simulation.nodes[1].x - simulation.nodes[0].x).abs();
    assert!(final_dist > initial_dist); 
}

#[test]
fn test_force_manybody_multiple_nodes() {
    let nodes = vec![
        ForceNode::new(0.0, 0.0),
        ForceNode::new(1.0, 0.0),
        ForceNode::new(0.0, 1.0),
    ];
    let initial_dist_0_1 = ((nodes[1].x - nodes[0].x).powi(2) + (nodes[1].y - nodes[0].y).powi(2)).sqrt();
    let initial_dist_0_2 = ((nodes[2].x - nodes[0].x).powi(2) + (nodes[2].y - nodes[0].y).powi(2)).sqrt();
    let initial_dist_1_2 = ((nodes[2].x - nodes[1].x).powi(2) + (nodes[2].y - nodes[1].y).powi(2)).sqrt();

    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_manybody(-10.0));
    for _ in 0..200 {
        simulation.tick();
    }
    let final_dist_0_1 = ((simulation.nodes[1].x - simulation.nodes[0].x).powi(2) + (simulation.nodes[1].y - simulation.nodes[0].y).powi(2)).sqrt();
    let final_dist_0_2 = ((simulation.nodes[2].x - simulation.nodes[0].x).powi(2) + (simulation.nodes[2].y - simulation.nodes[0].y).powi(2)).sqrt();
    let final_dist_1_2 = ((simulation.nodes[2].x - simulation.nodes[1].x).powi(2) + (simulation.nodes[2].y - simulation.nodes[1].y).powi(2)).sqrt();

    assert!(final_dist_0_1 > initial_dist_0_1); 
    assert!(final_dist_0_2 > initial_dist_0_2); 
    assert!(final_dist_1_2 > initial_dist_1_2); 
}

#[test]
fn test_force_center_multiple_nodes() {
    let nodes = vec![
        ForceNode::new(10.0, 10.0),
        ForceNode::new(20.0, 20.0),
        ForceNode::new(30.0, 30.0),
    ];
    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_center(0.0, 0.0, 0.1));
    for _ in 0..200 {
        simulation.tick();
    }
    assert!(simulation.nodes[0].x < 9.5 && simulation.nodes[0].y < 9.5);
    assert!(simulation.nodes[1].x < 19.5 && simulation.nodes[1].y < 19.5);
    assert!(simulation.nodes[2].x < 29.5 && simulation.nodes[2].y < 29.5);
}

#[test]
fn test_force_link_two_nodes() {
    let nodes = vec![
        ForceNode::new(0.0, 0.0),
        ForceNode::new(10.0, 0.0),
    ];
    let links = vec![(0, 1)];
    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_link(links));
    for _ in 0..200 {
        simulation.tick();
    }
    let initial_dist = ((nodes[1].x - nodes[0].x).powi(2) + (nodes[1].y - nodes[0].y).powi(2)).sqrt();
    let final_dist = ((simulation.nodes[1].x - simulation.nodes[0].x).powi(2) + (simulation.nodes[1].y - simulation.nodes[0].y).powi(2)).sqrt();
    assert!((final_dist - 30.0).abs() < (initial_dist - 30.0).abs()); 
}

#[test]
fn test_force_link_multiple_links() {
    let nodes = vec![
        ForceNode::new(0.0, 0.0),
        ForceNode::new(10.0, 0.0),
        ForceNode::new(0.0, 10.0),
    ];
    let links = vec![(0, 1), (0, 2)];
    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_link(links));
    for _ in 0..200 {
        simulation.tick();
    }
    let dist_0_1 = ((simulation.nodes[1].x - simulation.nodes[0].x).powi(2) + (simulation.nodes[1].y - simulation.nodes[0].y).powi(2)).sqrt();
    let dist_0_2 = ((simulation.nodes[2].x - simulation.nodes[0].x).powi(2) + (simulation.nodes[2].y - simulation.nodes[0].y).powi(2)).sqrt();
    assert!((dist_0_1 - 30.0).abs() < 10.0); 
    assert!((dist_0_2 - 30.0).abs() < 10.0); 
}

#[test]
fn test_force_collide_two_nodes() {
    let nodes = vec![
        ForceNode::new(0.0, 0.0),
        ForceNode::new(0.5, 0.0),
    ];
    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_collide(0.3));
    for _ in 0..200 {
        simulation.tick();
    }
    let dist = ((simulation.nodes[1].x - simulation.nodes[0].x).powi(2) + (simulation.nodes[1].y - simulation.nodes[0].y).powi(2)).sqrt();
    assert!(dist > 0.59); 
}

#[test]
fn test_force_collide_multiple_nodes() {
    let nodes = vec![
        ForceNode::new(0.0, 0.0),
        ForceNode::new(0.1, 0.0),
        ForceNode::new(0.0, 0.1),
    ];
    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_collide(0.1));
    for _ in 0..200 {
        simulation.tick();
    }
    let dist_0_1 = ((simulation.nodes[1].x - simulation.nodes[0].x).powi(2) + (simulation.nodes[1].y - simulation.nodes[0].y).powi(2)).sqrt();
    let dist_0_2 = ((simulation.nodes[2].x - simulation.nodes[0].x).powi(2) + (simulation.nodes[2].y - simulation.nodes[0].y).powi(2)).sqrt();
    assert!(dist_0_1 > 0.1); 
    assert!(dist_0_2 > 0.1); 
}

#[test]
fn test_force_x_multiple_nodes() {
    let nodes = vec![
        ForceNode::new(10.0, 0.0),
        ForceNode::new(20.0, 0.0),
    ];
    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_x(0.0, 0.5));
    for _ in 0..200 {
        simulation.tick();
    }
    assert!(simulation.nodes[0].x < 9.0);
    assert!(simulation.nodes[1].x < 19.0);
}

#[test]
fn test_force_y_multiple_nodes() {
    let nodes = vec![
        ForceNode::new(0.0, 10.0),
        ForceNode::new(0.0, 20.0),
    ];
    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_y(0.0, 0.5));
    for _ in 0..200 {
        simulation.tick();
    }
    assert!(simulation.nodes[0].y < 9.0);
    assert!(simulation.nodes[1].y < 19.0);
}

#[test]
fn test_force_radial_multiple_nodes() {
    let nodes = vec![
        ForceNode::new(10.0, 0.0),
        ForceNode::new(-10.0, 0.0),
    ];
    let mut simulation = ForceSimulation::new(nodes.clone()).add_force(force_radial(0.0, 0.0, 5.0, 0.1));
    for _ in 0..200 {
        simulation.tick();
    }
    let initial_dist = (nodes[0].x.powi(2) + nodes[0].y.powi(2)).sqrt();
    let final_dist = (simulation.nodes[0].x.powi(2) + simulation.nodes[0].y.powi(2)).sqrt();
    assert!((final_dist - 5.0).abs() < (initial_dist - 5.0).abs()); 
}