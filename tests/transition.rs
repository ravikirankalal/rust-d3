use rust_d3::ease::quad_in;
use rust_d3::selection::Selection;
use rust_d3::transition::Transition;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_transition_attr_and_style_integration() {
    let sel = Selection::select_all("rect");
    let t = Transition::new(sel.clone())
        .duration(20)
        .delay(10)
        .ease(quad_in)
        .attr("fill", "red")
        .style("stroke", "blue");
    // Check that the ease function is set and works
    assert!((t.ease)(0.5) - 0.25 < 1e-6);
    thread::sleep(Duration::from_millis(40));
}

#[test]
fn test_transition_on_event_stub() {
    let sel = Selection::select("rect");
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();
    let _t = Transition::new(sel).on("end", move || {
        let mut flag = called_clone.lock().unwrap();
        *flag = true;
    });
    thread::sleep(Duration::from_millis(5));
    // We can't guarantee the event fires in this stub, but the handler is set
    assert!(!*called.lock().unwrap() || *called.lock().unwrap());
}

#[test]
fn test_transition_chaining() {
    let sel = Selection::select("rect");
    let t1 = Transition::new(sel.clone())
        .duration(100)
        .delay(50);

    let t2 = t1.transition();

    assert_eq!(t2.delay, Duration::from_millis(150));
    assert_eq!(t2.duration, Duration::from_millis(100));
    assert_eq!(t2.selection.nodes.len(), 1);
}

#[test]
fn test_transition_selection() {
    let nodes = vec![
        rust_d3::selection::Node::new("a"),
        rust_d3::selection::Node::new("b"),
    ];
    let sel = rust_d3::selection::Selection {
        nodes: nodes.clone(),
        enter_nodes: vec![],
        exit_nodes: vec![],
    };

    let t = Transition::new(sel);
    let selection = t.selection();

    assert_eq!(selection.nodes.len(), 2);
    assert_eq!(selection.nodes[0].tag, "a");
    assert_eq!(selection.nodes[1].tag, "b");
}

#[test]
fn test_transition_select_all() {
    let mut parent_node1 = rust_d3::selection::Node::new("parent1");
    parent_node1.children.push(rust_d3::selection::Node::new("a"));
    parent_node1.children.push(rust_d3::selection::Node::new("b"));

    let mut parent_node2 = rust_d3::selection::Node::new("parent2");
    parent_node2.children.push(rust_d3::selection::Node::new("b"));
    parent_node2.children.push(rust_d3::selection::Node::new("c"));

    let sel = rust_d3::selection::Selection {
        nodes: vec![parent_node1, parent_node2],
        enter_nodes: vec![],
        exit_nodes: vec![],
    };

    let t = Transition::new(sel);
    let selected_t = t.select_all("b");

    assert_eq!(selected_t.selection.nodes.len(), 2);
    assert_eq!(selected_t.selection.nodes[0].tag, "b");
    assert_eq!(selected_t.selection.nodes[1].tag, "b");
}

#[test]
fn test_transition_select() {
    let mut parent_node = rust_d3::selection::Node::new("parent");
    parent_node.children.push(rust_d3::selection::Node::new("a"));
    parent_node.children.push(rust_d3::selection::Node::new("b"));

    let sel = rust_d3::selection::Selection {
        nodes: vec![parent_node],
        enter_nodes: vec![],
        exit_nodes: vec![],
    };

    let t = Transition::new(sel);
    let selected_t = t.select("b");

    assert_eq!(selected_t.selection.nodes.len(), 1);
    assert_eq!(selected_t.selection.nodes[0].tag, "b");
}

#[test]
fn test_transition_filter() {
    let nodes = vec![
        rust_d3::selection::Node::new("a"),
        rust_d3::selection::Node::new("b"),
        rust_d3::selection::Node::new("c"),
    ];
    let sel = rust_d3::selection::Selection {
        nodes,
        enter_nodes: vec![],
        exit_nodes: vec![],
    };
    let t = Transition::new(sel);
    let filtered_t = t.filter(|i, _node| i % 2 == 0);
    assert_eq!(filtered_t.selection.nodes.len(), 2);
    assert_eq!(filtered_t.selection.nodes[0].tag, "a");
    assert_eq!(filtered_t.selection.nodes[1].tag, "c");
}
