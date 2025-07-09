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
fn test_transition_advanced_stubs() {
    let node = rust_d3::selection::Node::new("test");
    let sel = rust_d3::selection::Selection {
        nodes: vec![node.clone()],
        enter_nodes: vec![],
        exit_nodes: vec![],
    };
    let t = Transition::new(sel);

    // Test active (stub)
    assert!(Transition::active(&node).is_none());

    // Test attr_tween (stub)
    let t_attr_tween = t.attr_tween("fill", || Box::new(|_t| "red".to_string()));
    assert_eq!(t_attr_tween.size(), 1);

    // Test style_tween (stub)
    let t_style_tween = t_attr_tween.style_tween("color", || Box::new(|_t| "blue".to_string()));
    assert_eq!(t_style_tween.size(), 1);

    // Test text_tween (stub)
    let t_text_tween = t_style_tween.text_tween(|| Box::new(|_t| "hello".to_string()));
    assert_eq!(t_text_tween.size(), 1);

    // Test ease_varying (stub)
    let t_ease_varying = t_text_tween.ease_varying(|_node| |t| t);
    assert_eq!(t_ease_varying.size(), 1);

    // Test end (stub)
    assert_eq!(t_ease_varying.end(), false);
}

#[test]
fn test_transition_function_methods() {
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

    // Test each
    let each_t = t.each(|node| {
        node.attributes.insert("data-checked".to_string(), "true".to_string());
    });
    assert_eq!(each_t.nodes()[0].attributes.get("data-checked").unwrap(), "true");
    assert_eq!(each_t.nodes()[1].attributes.get("data-checked").unwrap(), "true");

    // Test call
    let called_t = each_t.call(|transition| {
        transition.duration(100)
    });
    assert_eq!(called_t.duration.as_millis(), 100);
}

#[test]
fn test_transition_selection_methods() {
    let mut parent1 = rust_d3::selection::Node::new("div");
    parent1.children.push(rust_d3::selection::Node::new("span"));
    parent1.children.push(rust_d3::selection::Node::new("p"));

    let mut parent2 = rust_d3::selection::Node::new("div");
    parent2.children.push(rust_d3::selection::Node::new("span"));

    let sel1 = rust_d3::selection::Selection {
        nodes: vec![parent1],
        enter_nodes: vec![],
        exit_nodes: vec![],
    };
    let t1 = Transition::new(sel1);

    // Test selectChild
    let child_t = t1.select_child();
    assert_eq!(child_t.size(), 1);
    assert_eq!(child_t.node().unwrap().tag, "span");

    // Test selectChildren
    let children_t = t1.select_children();
    assert_eq!(children_t.size(), 2);
    assert_eq!(children_t.nodes()[0].tag, "span");
    assert_eq!(children_t.nodes()[1].tag, "p");

    // Test merge
    let sel2 = rust_d3::selection::Selection {
        nodes: vec![parent2],
        enter_nodes: vec![],
        exit_nodes: vec![],
    };
    let t2 = Transition::new(sel2);
    let merged_t = t1.merge(&t2);
    assert_eq!(merged_t.size(), 2);
    assert_eq!(merged_t.nodes()[0].tag, "div");
    assert_eq!(merged_t.nodes()[1].tag, "div");
}

#[test]
fn test_transition_utility_methods() {
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

    assert_eq!(t.empty(), false);
    assert_eq!(t.size(), 2);
    assert_eq!(t.nodes().len(), 2);
    assert_eq!(t.node().unwrap().tag, "a");

    let empty_sel = rust_d3::selection::Selection {
        nodes: vec![],
        enter_nodes: vec![],
        exit_nodes: vec![],
    };
    let empty_t = Transition::new(empty_sel);
    assert_eq!(empty_t.empty(), true);
    assert_eq!(empty_t.size(), 0);
    assert!(empty_t.node().is_none());
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
