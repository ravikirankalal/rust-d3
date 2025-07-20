use rust_d3::selection::{Arena, Node, Selection};
use slotmap::SlotMap;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[test]
fn test_selection_root_and_attr() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    svg.attr("width", "300").attr("height", "120");
    let node = svg.node().unwrap();
    assert_eq!(node.tag, "svg");
    assert_eq!(node.attributes["width"], "300");
    assert_eq!(node.attributes["height"], "120");
}

#[test]
fn test_append_and_attr_fn() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut group = svg.append("g");
    let mut rects = group.append("rect");
    let data = vec![10, 20, 30];
    rects
        .data(&data)
        .update
        .attr_fn("x", |_, i, _| (i * 10).to_string())
        .attr_fn("height", |n, _, _| n.data.as_ref().unwrap().clone());
    let nodes = rects.nodes();
    assert_eq!(nodes.len(), 1); // Only one rect appended per append call
    let rect = &nodes[0];
    assert_eq!(rect.tag, "rect");
    assert_eq!(rect.attributes["x"], "0");
    assert_eq!(rect.attributes["height"], "10");
}

#[test]
fn test_data_and_attr_fn_multiple() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut group = svg.append("g");
    // Collect keys of appended rects to avoid multiple mutable borrows
    let mut rect_keys = Vec::new();
    for _ in 0..3 {
        let rect_sel = group.append("rect");
        rect_keys.extend(rect_sel.iter().cloned());
    }
    let mut rects = Selection::new(Rc::clone(&arena), rect_keys);
    let data = vec![10, 20, 30];
    rects
        .data(&data)
        .update
        .attr_fn("x", |_, i, _| (i * 10).to_string())
        .attr_fn("height", |n, _, _| n.data.as_ref().unwrap().clone());
    let nodes = rects.nodes();
    assert_eq!(nodes.len(), 3);
    for (i, node) in nodes.iter().enumerate() {
        assert_eq!(node.tag, "rect");
        assert_eq!(node.attributes["x"], (i * 10).to_string());
        assert_eq!(node.attributes["height"], data[i].to_string());
    }
}

#[test]
fn test_iter_and_append_chain() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    svg.append("g").append("rect");
    let root_key = *svg.iter().next().unwrap();
    let root_node = &arena.borrow().nodes[root_key];
    assert_eq!(root_node.tag, "svg");
    assert_eq!(root_node.children.len(), 1);
    let g_key = root_node.children[0];
    let g_node = &arena.borrow().nodes[g_key];
    assert_eq!(g_node.tag, "g");
    assert_eq!(g_node.children.len(), 1);
    let rect_key = g_node.children[0];
    let rect_node = &arena.borrow().nodes[rect_key];
    assert_eq!(rect_node.tag, "rect");
}

#[test]
fn test_join_basic() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut group = svg.append("g");
    // Start with 2 rects
    let mut rect_keys = Vec::new();
    for _ in 0..2 {
        let rect_sel = group.append("rect");
        rect_keys.extend(rect_sel.iter().cloned());
    }
    let mut rects = Selection::new(Rc::clone(&arena), rect_keys);
    // Join with 3 data items (should create 1 new rect)
    let data = vec!["A", "B", "C"];
    let joined = rects.data(&data);
    let mut entered = joined.enter;
    entered.join("rect");
    let _updated = joined.update;
    let nodes = svg.select_all(Some("rect")).nodes();
    assert_eq!(nodes.len(), 3);
    
    // Check that first node has first data item
    // The order should be preserved from the data array
    let first_node_data = nodes[0].data.as_ref().unwrap();
    assert_eq!(first_node_data, "A", "First node should have data 'A', but got '{}'.", first_node_data);
    
    // Check the second node
    let second_node_data = nodes[1].data.as_ref().unwrap();
    assert_eq!(second_node_data, "B", "Second node should have data 'B', but got '{}'.", second_node_data);
    
    // Check the third node
    let third_node_data = nodes[2].data.as_ref().unwrap();
    assert_eq!(third_node_data, "C", "Third node should have data 'C', but got '{}'.", third_node_data);
    
    // Now join with 1 data item (should remove 2 rects)
    let data2 = vec!["Z"];
    let mut joined2 = svg.select_all(Some("rect")).data(&data2);
    assert_eq!(joined2.update.nodes().len(), 1);
    assert_eq!(joined2.exit.nodes().len(), 2);
    joined2.exit.remove();
    let nodes2 = svg.select_all(Some("rect")).nodes();
    assert_eq!(nodes2.len(), 1);
    assert_eq!(nodes2[0].data.as_ref().unwrap(), "Z");
}

#[test]
fn test_attr_fn_callback_behavior() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut group = svg.append("g");
    let mut rect_keys = Vec::new();
    for _ in 0..4 {
        let rect_sel = group.append("rect");
        rect_keys.extend(rect_sel.iter().cloned());
    }
    let mut rects = Selection::new(Rc::clone(&arena), rect_keys);
    let data = vec![5, 10, 15, 20];
    let mut joined = rects.data(&data);
    joined.update.attr_fn("custom", |n, i, _| {
        // Test that node data is correct and index is as expected
        let d = n.data.as_ref().unwrap().parse::<i32>().unwrap();
        format!("{}-{}", d, i)
    });
    let nodes = rects.nodes();
    assert_eq!(nodes.len(), 4);
    for (i, node) in nodes.iter().enumerate() {
        assert_eq!(node.attributes["custom"], format!("{}-{}", data[i], i));
    }
}

#[test]
fn test_render_node_svg() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    svg.attr("width", "100").attr("height", "100");
    let mut g = svg.append("g");
    g.attr("fill", "red");
    let mut rect = g.append("rect");
    rect.attr("x", "10")
        .attr("y", "20")
        .attr("width", "30")
        .attr("height", "40");
    let root_key = *svg.iter().next().unwrap();
    let svg_str = Selection::render_node(&arena, root_key);
    assert!(svg_str.contains("<svg"));
    assert!(svg_str.contains("<g"));
    assert!(svg_str.contains("<rect"));
    assert!(svg_str.contains("width=\"100\""));
    assert!(svg_str.contains("fill=\"red\""));
    assert!(svg_str.contains("x=\"10\""));
    assert!(svg_str.contains("y=\"20\""));
    assert!(svg_str.contains("</svg>"));
}

#[test]
fn test_raise_and_lower() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut selection = Selection::root(Rc::clone(&arena), "div");
    selection.append("p").attr("class", "first");
    selection.append("p").attr("class", "second");

    // Raise the first <p>
    {
        let mut children_to_raise = selection.children();
        children_to_raise
            .filter(|n: &Node| n.attributes.get("class") == Some(&"first".to_string()))
            .raise();
    }

    let children_after_raise = selection.children();
    let classes_after_raise: Vec<_> = children_after_raise
        .nodes_ref()
        .iter()
        .map(|n| n.attributes.get("class").unwrap().clone())
        .collect();
    assert_eq!(classes_after_raise, vec!["second", "first"]);

    // Lower the first <p> (which is now the second element)
    {
        let mut children_to_lower = selection.children();
        children_to_lower
            .filter(|n: &Node| n.attributes.get("class") == Some(&"first".to_string()))
            .lower();
    }

    let children_after_lower = selection.children();
    let classes_after_lower: Vec<_> = children_after_lower
        .nodes_ref()
        .iter()
        .map(|n| n.attributes.get("class").unwrap().clone())
        .collect();
    assert_eq!(classes_after_lower, vec!["first", "second"]);
}

#[test]
fn test_clone_d3_parity() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    svg.attr("width", "100").attr("height", "100");

    // Add a child element with attributes and data
    let mut rect = svg.append("rect");
    rect.attr("x", "10")
        .attr("y", "20")
        .attr("width", "30")
        .attr("height", "40")
        .attr("fill", "red")
        .datum("test-data");

    // Clone the rect selection
    let cloned_rect = rect.clone();

    // Verify the cloned selection has the same properties
    assert_eq!(cloned_rect.len(), 1);
    let cloned_node = cloned_rect.node().unwrap();
    assert_eq!(cloned_node.tag, "rect");
    assert_eq!(cloned_node.attributes.get("x"), Some(&"10".to_string()));
    assert_eq!(cloned_node.attributes.get("y"), Some(&"20".to_string()));
    assert_eq!(cloned_node.attributes.get("width"), Some(&"30".to_string()));
    assert_eq!(
        cloned_node.attributes.get("height"),
        Some(&"40".to_string())
    );
    assert_eq!(cloned_node.attributes.get("fill"), Some(&"red".to_string()));
    assert_eq!(cloned_node.data, Some("test-data".to_string()));

    // Verify the original still exists and is unchanged
    let original_node = rect.node().unwrap();
    assert_eq!(original_node.tag, "rect");
    assert_eq!(
        original_node.attributes.get("fill"),
        Some(&"red".to_string())
    );
    assert_eq!(original_node.data, Some("test-data".to_string()));

    // Verify the parent (svg) now has 2 children (original + cloned)
    let svg_node = svg.node().unwrap();
    assert_eq!(svg_node.children.len(), 2);

    // Verify the cloned node was inserted after the original
    let arena_borrow = arena.borrow();
    let svg_key = *svg.iter().next().unwrap();
    let svg_children = &arena_borrow.nodes[svg_key].children;

    // First child should be the original rect
    let first_child = &arena_borrow.nodes[svg_children[0]];
    assert_eq!(first_child.tag, "rect");
    assert_eq!(first_child.attributes.get("fill"), Some(&"red".to_string()));

    // Second child should be the cloned rect
    let second_child = &arena_borrow.nodes[svg_children[1]];
    assert_eq!(second_child.tag, "rect");
    assert_eq!(
        second_child.attributes.get("fill"),
        Some(&"red".to_string())
    );

    // They should be different nodes (different keys)
    assert_ne!(svg_children[0], svg_children[1]);
}

#[test]
fn test_clone_shallow_vs_deep() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut rect = svg.append("rect");
    rect.attr("fill", "blue").datum("original");

    // Test shallow clone
    let shallow_clone = rect.clone_shallow();
    assert_eq!(shallow_clone.len(), 1);

    // Shallow clone should reference the same node
    let shallow_node = shallow_clone.node().unwrap();
    let original_node = rect.node().unwrap();
    assert_eq!(shallow_node.tag, original_node.tag);
    assert_eq!(
        shallow_node.attributes.get("fill"),
        original_node.attributes.get("fill")
    );

    // The svg should still only have 1 child (no new node was created)
    let svg_node = svg.node().unwrap();
    assert_eq!(svg_node.children.len(), 1);

    // Test deep clone (this is what .clone() does)
    let deep_clone = rect.clone();
    assert_eq!(deep_clone.len(), 1);

    // Deep clone should create a new node
    let deep_node = deep_clone.node().unwrap();
    assert_eq!(deep_node.tag, "rect");
    assert_eq!(deep_node.attributes.get("fill"), Some(&"blue".to_string()));
    assert_eq!(deep_node.data, Some("original".to_string()));

    // The svg should now have 2 children (original + cloned)
    let svg_node = svg.node().unwrap();
    assert_eq!(svg_node.children.len(), 2);
}

#[test]
fn test_clone_event_handlers_not_cloned() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut rect = svg.append("rect");

    // Add event handler to original
    rect.on("click", |_node| { /* handler */ });

    // Verify the event handler was added
    {
        let arena_borrow = arena.borrow();
        let rect_key = *rect.iter().next().unwrap();
        let rect_node = &arena_borrow.nodes[rect_key];
        assert!(!rect_node.event_handlers.is_empty());
        assert!(rect_node.event_handlers.contains_key("click"));
    }

    // Clone the rect
    let cloned_rect = rect.clone();

    // Check that the cloned node doesn't have event handlers
    let cloned_node = cloned_rect.node().unwrap();
    assert!(cloned_node.event_handlers.is_empty());

    // Check that the original still has event handlers (in the arena)
    // Note: node() returns a clone, and Clone for Node doesn't clone event handlers
    // So we need to check directly in the arena
    {
        let arena_borrow = arena.borrow();
        let rect_key = *rect.iter().next().unwrap();
        let rect_node = &arena_borrow.nodes[rect_key];
        assert!(!rect_node.event_handlers.is_empty());
        assert!(rect_node.event_handlers.contains_key("click"));
    }
}

#[test]
fn test_clone_multiple_elements() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");

    // Create multiple elements
    let mut rect1 = svg.append("rect");
    rect1.attr("x", "10").attr("fill", "red");

    let mut rect2 = svg.append("rect");
    rect2.attr("x", "20").attr("fill", "blue");

    // Select all rects and clone them
    let rects = svg.select_all(Some("rect"));
    let cloned_rects = rects.clone();

    // Should have cloned both rects
    assert_eq!(cloned_rects.len(), 2);

    // The svg should now have 4 children (2 original + 2 cloned)
    let svg_node = svg.node().unwrap();
    assert_eq!(svg_node.children.len(), 4);

    // Check that cloned nodes have the right attributes
    let cloned_nodes = cloned_rects.nodes();

    // Both cloned nodes should be rects
    for node in &cloned_nodes {
        assert_eq!(node.tag, "rect");
        assert!(node.attributes.contains_key("x"));
        assert!(node.attributes.contains_key("fill"));
    }

    // Check that one is red and one is blue (order might vary)
    let fills: Vec<&String> = cloned_nodes
        .iter()
        .map(|n| n.attributes.get("fill").unwrap())
        .collect();
    assert!(fills.contains(&&"red".to_string()));
    assert!(fills.contains(&&"blue".to_string()));
}

#[test]
fn test_datum() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut selection = Selection::root(Rc::clone(&arena), "div");
    selection.append("p");
    selection.append("p");

    selection.datum(10);

    for node in selection.nodes() {
        assert_eq!(node.data, Some("10".to_string()));
    }
}

#[test]
fn test_data_enter_exit() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut root = Selection::root(Rc::clone(&arena), "div");
    root.append("p").attr("id", "p1");
    root.append("p").attr("id", "p2");

    // Test with more data than elements (enter selection)
    let data1 = vec!["a", "b", "c"];
    let data_join1 = root.select_all(Some("p")).data(&data1);
    assert_eq!(data_join1.enter.len(), 1); // One new element should be in enter
    assert_eq!(data_join1.update.len(), 2); // Two existing elements in update

    // Test with less data than elements (exit selection)
    let data2 = vec!["d"];
    let data_join2 = root.select_all(Some("p")).data(&data2);
    assert_eq!(data_join2.exit.len(), 1); // One element should be in exit
    assert_eq!(data_join2.update.len(), 1); // One existing element in update

    // Test with same amount of data and elements (no enter or exit)
    let data3 = vec!["e", "f"];
    let data_join3 = root.select_all(Some("p")).data(&data3);
    assert_eq!(data_join3.enter.len(), 0);
    assert_eq!(data_join3.exit.len(), 0);
    assert_eq!(data_join3.update.len(), 2);
}

#[test]
fn test_classed() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut selection = Selection::root(Rc::clone(&arena), "div");
    selection.append("p");

    selection.classed("foo", true);
    let node = selection.node().unwrap();
    assert_eq!(node.attributes.get("class"), Some(&"foo".to_string()));

    selection.classed("bar", true);
    let node = selection.node().unwrap();
    let classes: HashSet<String> = node
        .attributes
        .get("class")
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    assert!(classes.contains("foo"));
    assert!(classes.contains("bar"));

    selection.classed("foo", false);
    let node = selection.node().unwrap();
    let classes: HashSet<String> = node
        .attributes
        .get("class")
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    assert!(!classes.contains("foo"));
    assert!(classes.contains("bar"));
}

#[test]
fn test_style() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut selection = Selection::root(Rc::clone(&arena), "div");
    selection.append("p");

    selection.style("color", "red");
    let node = selection.node().unwrap();
    assert_eq!(node.attributes.get("style"), Some(&"color:red".to_string()));

    selection.style("font-size", "12px");
    let node = selection.node().unwrap();
    let style = node.attributes.get("style").unwrap();
    assert!(style.contains("color:red"));
    assert!(style.contains("font-size:12px"));

    selection.style("color", "");
    let node = selection.node().unwrap();
    let style = node.attributes.get("style").unwrap();
    assert!(!style.contains("color"));
    assert!(style.contains("font-size:12px"));
}

#[test]
fn test_property() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut selection = Selection::root(Rc::clone(&arena), "div");
    selection.append("p");

    selection.property("foo", "bar");
    let node = selection.node().unwrap();
    assert_eq!(node.properties.get("foo"), Some(&"bar".to_string()));
}

#[test]
fn test_data_join_keyed_vs_unkeyed() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut root = Selection::root(Rc::clone(&arena), "div");

    // Create initial elements
    root.append("p").attr("id", "p1");
    root.append("p").attr("id", "p2");

    // Test unkeyed join (positional)
    let data1 = vec!["A", "B", "C"];
    let join1 = root.select_all(Some("p")).data(&data1);
    assert_eq!(join1.enter.len(), 1); // One new element
    assert_eq!(join1.update.len(), 2); // Two existing elements
    assert_eq!(join1.exit.len(), 0);

    // Test keyed join with same data
    let key_fn = |d: &&str, _: usize| d.to_string();
    let _join2 = root.select_all(Some("p")).data_with_key(&data1, key_fn);

    // Test keyed join with same data (using IDs)
    let data2 = vec!["A", "B", "C"];
    let join3 = root.select_all(Some("p")).data_with_key(&data2, |d, _| d.to_string());

    // Original nodes match, only new elements should enter
    assert_eq!(join3.enter.len(), 1); // C is new
    assert_eq!(join3.update.len(), 2); // A and B match
    assert_eq!(join3.exit.len(), 0); // Nothing to exit
}

#[test]
fn test_data_join_mismatch_counts() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut root = Selection::root(Rc::clone(&arena), "div");

    // Test 0→n (empty selection to n elements)
    let data1 = vec!["x", "y", "z"];
    let mut join1 = root.select_all(Some("p")).data(&data1);
    assert_eq!(join1.enter.len(), 3);
    assert_eq!(join1.update.len(), 0);
    assert_eq!(join1.exit.len(), 0);

    // Create actual elements for n→0 test
    join1.enter.join("p");

    // Test n→0 (n elements to empty data)
    let data2: Vec<&str> = vec![];
    let join2 = root.select_all(Some("p")).data(&data2);
    assert_eq!(join2.enter.len(), 0);
    assert_eq!(join2.update.len(), 0);
    assert_eq!(join2.exit.len(), 3);
}

#[test]
fn test_data_join_order_preservation() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut root = Selection::root(Rc::clone(&arena), "div");

    // Create elements with data
    let data1 = vec!["first", "second", "third"];
    let mut join1 = root.select_all(Some("p")).data(&data1);
    join1.enter.join("p");

    // Check that elements are in correct order
    let nodes = root.select_all(Some("p")).nodes();
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0].data.as_ref().unwrap(), "first");
    assert_eq!(nodes[1].data.as_ref().unwrap(), "second");
    assert_eq!(nodes[2].data.as_ref().unwrap(), "third");

    // Update with reordered data
    let data2 = vec!["third", "first", "second"];
    let join2 = root.select_all(Some("p")).data(&data2);

    // Order should be preserved according to data order
    let update_nodes = join2.update.nodes();
    assert_eq!(update_nodes.len(), 3);
    assert_eq!(update_nodes[0].data.as_ref().unwrap(), "third");
    assert_eq!(update_nodes[1].data.as_ref().unwrap(), "first");
    assert_eq!(update_nodes[2].data.as_ref().unwrap(), "second");
}

#[test]
fn test_data_join_keyed_order_preservation() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut root = Selection::root(Rc::clone(&arena), "div");

    // Create elements with keyed data
    let data1 = vec!["A", "B", "C"];
    let key_fn = |d: &&str, _: usize| d.to_string();
    let mut join1 = root.select_all(Some("p")).data_with_key(&data1, key_fn);
    join1.enter.join("p");

    // Update with different order and mixed enter/update/exit
    let data2 = vec!["B", "D", "A"];
    let mut join2 = root.select_all(Some("p")).data_with_key(&data2, key_fn);

    // Should have B and A in update (existing), D in enter (new), C in exit (removed)
    assert_eq!(join2.enter.len(), 1); // D is new
    assert_eq!(join2.update.len(), 2); // B, A exist
    assert_eq!(join2.exit.len(), 1); // C is removed

    // Execute the join
    join2.enter.join("p");
    join2.exit.remove();

    // Check final order matches data order (for updates)
    let update_nodes = join2.update.nodes();
    assert_eq!(update_nodes.len(), 2);
    assert_eq!(update_nodes[0].data.as_ref().unwrap(), "B");
    assert_eq!(update_nodes[1].data.as_ref().unwrap(), "A");
}

#[test]
fn test_data_join_join() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut rects = svg.append("rect");
    let data = vec![10, 20, 30];
    let joined = rects.data(&data);
    let mut entered = joined.enter;
    entered.join("rect");
    let nodes = svg.select_all(Some("rect")).nodes();
    assert_eq!(nodes.len(), 3);
    for (i, node) in nodes.iter().enumerate() {
        assert_eq!(node.tag, "rect");
        // The data is joined to the enter selection, so check the data there
        assert_eq!(node.data.as_ref().unwrap(), &data[i].to_string());
    }
    // Now join with 1 data item (should remove 2 rects)
    let data2 = vec!["Z"];
    let mut joined2 = svg.select_all(Some("rect")).data(&data2);
    assert_eq!(joined2.update.nodes().len(), 1);
    assert_eq!(joined2.exit.nodes().len(), 2);
    joined2.exit.remove();
    let nodes2 = svg.select_all(Some("rect")).nodes();
    assert_eq!(nodes2.len(), 1);
    assert_eq!(nodes2[0].data.as_ref().unwrap(), "Z");
}

#[test]
fn test_data_join_chained_attr() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut rects = svg.append("rect");
    let data = vec![10, 20, 30];
    let mut joined = rects.data(&data);
    joined
        .enter
        .append("rect")
        .attr_fn("custom", |n, i, _| format!("{}-{}", n.tag, i));

    let nodes = svg.select_all(Some("rect")).nodes();
    assert_eq!(nodes.len(), 3);
    
    // The first rect is the original one (update selection) - it has data "10" but no custom attr
    assert_eq!(nodes[0].data.as_ref().unwrap(), "10");
    assert_eq!(nodes[0].attributes.get("custom"), None);
    
    // The second and third rects are from the enter selection - they have custom attrs
    assert_eq!(nodes[1].data.as_ref().unwrap(), "20");
    assert_eq!(nodes[1].attributes.get("custom"), Some(&"rect-0".to_string()));
    
    assert_eq!(nodes[2].data.as_ref().unwrap(), "30");
    assert_eq!(nodes[2].attributes.get("custom"), Some(&"rect-1".to_string()));
}

#[test]
fn test_empty_selection_no_op() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");

    // Create empty selection
    let mut empty_selection = svg.select_all(Some("nonexistent"));
    assert!(empty_selection.is_empty());

    // All operations on empty selection should be no-ops and return self
    let result = empty_selection
        .attr("width", "100")
        .style("color", "red")
        .property("value", "test")
        .classed("active", true)
        .text("content")
        .html("<span>test</span>");

    // Should still be empty and chainable
    assert!(result.is_empty());
    assert_eq!(result.len(), 0);

    // Remove on empty selection should also be no-op
    result.remove();
    assert!(result.is_empty());

    // Verify the root svg node is unaffected
    let svg_node = svg.node().unwrap();
    assert_eq!(svg_node.tag, "svg");
    assert!(svg_node.attributes.is_empty());
}

#[test]
fn test_attr_none_value_removes_attribute() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");

    // Set an attribute
    svg.attr("width", "100").attr("height", "200");
    let mut node = svg.node().unwrap();
    assert_eq!(node.attributes.get("width"), Some(&"100".to_string()));
    assert_eq!(node.attributes.get("height"), Some(&"200".to_string()));

    // Remove attribute by setting empty string (D3 behavior)
    svg.attr("width", "");
    node = svg.node().unwrap();

    // In D3, empty string removes the attribute
    // Our implementation should match this behavior
    assert_eq!(node.attributes.get("width"), None);
    assert_eq!(node.attributes.get("height"), Some(&"200".to_string()));
}

#[test]
fn test_style_none_value_removes_style() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");

    // Set multiple styles
    svg.style("color", "red").style("font-size", "12px");
    let mut node = svg.node().unwrap();
    let style = node.attributes.get("style").unwrap();
    assert!(style.contains("color:red"));
    assert!(style.contains("font-size:12px"));

    // Remove style by setting empty string
    svg.style("color", "");
    node = svg.node().unwrap();
    let style = node.attributes.get("style").unwrap();
    assert!(!style.contains("color"));
    assert!(style.contains("font-size:12px"));
}

#[test]
fn test_attr_fn_with_previous_value() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");

    // Set initial attribute
    svg.attr("width", "100");

    // Use attr_fn with previous value (D3 signature: node, index, previous_value)
    svg.attr_fn("width", |node, index, prev_value| {
        let prev = prev_value.unwrap_or_default();
        format!("{}-{}-{}", node.tag, index, prev)
    });

    let node = svg.node().unwrap();
    assert_eq!(node.attributes.get("width"), Some(&"svg-0-100".to_string()));
}

#[test]
fn test_style_fn_with_previous_value() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");

    // Set initial style
    svg.style("width", "100px");

    // Use style_fn with previous value (D3 signature: node, index, previous_value)
    svg.style_fn("width", |node, index, prev_value| {
        let prev = prev_value.unwrap_or_default();
        format!("{}-{}-{}", node.tag, index, prev)
    });

    let node = svg.node().unwrap();
    let style = node.attributes.get("style").unwrap();
    assert!(style.contains("width:svg-0-100px"));
}

#[test]
fn test_order_method() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");

    // Create multiple child elements and store keys
    let rect1_key = *svg
        .append("rect")
        .attr("id", "rect1")
        .iter()
        .next()
        .unwrap();
    let rect2_key = *svg
        .append("rect")
        .attr("id", "rect2")
        .iter()
        .next()
        .unwrap();
    let rect3_key = *svg
        .append("rect")
        .attr("id", "rect3")
        .iter()
        .next()
        .unwrap();

    // Initially children should be in order: rect1, rect2, rect3
    let initial_children = svg.children();
    let initial_ids: Vec<_> = initial_children
        .nodes()
        .iter()
        .map(|n| n.attributes.get("id").unwrap().clone())
        .collect();
    assert_eq!(initial_ids, vec!["rect1", "rect2", "rect3"]);

    // Create a selection with reversed order: rect3, rect2, rect1
    let mut reversed_selection =
        Selection::new(Rc::clone(&arena), vec![rect3_key, rect2_key, rect1_key]);

    // Apply order() - should reorder DOM elements to match selection order
    reversed_selection.order();

    // Check that DOM order now matches selection order
    let reordered_children = svg.children();
    let reordered_ids: Vec<_> = reordered_children
        .nodes()
        .iter()
        .map(|n| n.attributes.get("id").unwrap().clone())
        .collect();
    assert_eq!(reordered_ids, vec!["rect3", "rect2", "rect1"]);
}

#[test]
fn test_attr_micro_performance_optimization() {
    use std::cell::RefCell;
    
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    
    // Set an attribute first time
    svg.attr("width", "100");
    let node = svg.node().unwrap();
    assert_eq!(node.attributes.get("width"), Some(&"100".to_string()));
    
    // Set the same value again - should be optimized to skip insertion
    svg.attr("width", "100");
    let node = svg.node().unwrap();
    assert_eq!(node.attributes.get("width"), Some(&"100".to_string()));
    
    // Set different value - should update
    svg.attr("width", "200");
    let node = svg.node().unwrap();
    assert_eq!(node.attributes.get("width"), Some(&"200".to_string()));
    
    // Set empty value - should remove attribute
    svg.attr("width", "");
    let node = svg.node().unwrap();
    assert_eq!(node.attributes.get("width"), None);
}

#[test]
fn test_attr_fn_micro_performance_optimization() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    
    // Set an attribute first time using attr_fn
    svg.attr_fn("test-attr", |_, _, _| "value1".to_string());
    let node = svg.node().unwrap();
    assert_eq!(node.attributes.get("test-attr"), Some(&"value1".to_string()));
    
    // Set the same value again using attr_fn - should be optimized to skip insertion
    svg.attr_fn("test-attr", |_, _, prev| {
        assert_eq!(prev, Some("value1".to_string()));
        "value1".to_string() // Same value
    });
    let node = svg.node().unwrap();
    assert_eq!(node.attributes.get("test-attr"), Some(&"value1".to_string()));
    
    // Set different value using attr_fn - should update
    svg.attr_fn("test-attr", |_, _, prev| {
        assert_eq!(prev, Some("value1".to_string()));
        "value2".to_string() // Different value
    });
    let node = svg.node().unwrap();
    assert_eq!(node.attributes.get("test-attr"), Some(&"value2".to_string()));
}

#[test]
fn test_attr_px_convenience_method() {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    
    // Test attr_px with various f64 values
    svg.attr_px("width", 100.0)
        .attr_px("height", 50.5)
        .attr_px("x", 10.333333)
        .attr_px("y", 0.0)
        .attr_px("r", 1.0/3.0); // Should be rounded to 6 decimal places
    
    let node = svg.node().unwrap();
    
    // Check that px() formatting is applied correctly
    assert_eq!(node.attributes.get("width"), Some(&"100".to_string()));
    assert_eq!(node.attributes.get("height"), Some(&"50.5".to_string()));
    assert_eq!(node.attributes.get("x"), Some(&"10.333333".to_string()));
    assert_eq!(node.attributes.get("y"), Some(&"0".to_string()));
    assert_eq!(node.attributes.get("r"), Some(&"0.333333".to_string()));
}

#[test]
fn test_attr_px_vs_manual_px_consistency() {
    use rust_d3::px;
    
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    
    // Create two identical elements
    let mut rect1 = svg.append("rect");
    let mut rect2 = svg.append("rect");
    
    // Set attributes using manual px() conversion
    rect1.attr("x", &px(25.75))
         .attr("y", &px(30.0))
         .attr("width", &px(100.5));
    
    // Set attributes using convenience attr_px() method
    rect2.attr_px("x", 25.75)
         .attr_px("y", 30.0)
         .attr_px("width", 100.5);
    
    let rect1_node = rect1.node().unwrap();
    let rect2_node = rect2.node().unwrap();
    
    // Both approaches should produce identical results
    assert_eq!(rect1_node.attributes.get("x"), rect2_node.attributes.get("x"));
    assert_eq!(rect1_node.attributes.get("y"), rect2_node.attributes.get("y"));
    assert_eq!(rect1_node.attributes.get("width"), rect2_node.attributes.get("width"));
    
    // Verify actual values
    assert_eq!(rect1_node.attributes.get("x"), Some(&"25.75".to_string()));
    assert_eq!(rect1_node.attributes.get("y"), Some(&"30".to_string()));
    assert_eq!(rect1_node.attributes.get("width"), Some(&"100.5".to_string()));
}
