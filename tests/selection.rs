use slotmap::SlotMap;
use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;
use rust_d3::selection::{Node, Arena, Selection};

#[test]
fn test_selection_root_and_attr() {
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    svg.attr("width", "300").attr("height", "120");
    let node = svg.node().unwrap();
    assert_eq!(node.tag, "svg");
    assert_eq!(node.attributes["width"], "300");
    assert_eq!(node.attributes["height"], "120");
}

#[test]
fn test_append_and_attr_fn() {
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut group = svg.append("g");
    let mut rects = group.append("rect");
    let data = vec![10, 20, 30];
    rects.data(&data)
        .update
        .attr_fn("x", |_, i| (i * 10).to_string())
        .attr_fn("height", |n, _| n.data.as_ref().unwrap().clone());
    let nodes = rects.nodes();
    assert_eq!(nodes.len(), 1); // Only one rect appended per append call
    let rect = &nodes[0];
    assert_eq!(rect.tag, "rect");
    assert_eq!(rect.attributes["x"], "0");
    assert_eq!(rect.attributes["height"], "10");
}

#[test]
fn test_data_and_attr_fn_multiple() {
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
    rects.data(&data)
        .update
        .attr_fn("x", |_, i| (i * 10).to_string())
        .attr_fn("height", |n, _| n.data.as_ref().unwrap().clone());
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
    let mut joined = rects.data(&data);
    let mut entered = joined.enter;
    entered.join("rect");
    let updated = joined.update;
    let nodes = svg.select_all(Some("rect")).nodes();
    assert_eq!(nodes.len(), 3);
    for (i, node) in nodes.iter().enumerate() {
        assert_eq!(node.tag, "rect");
        // The data is joined to the enter selection, so check the data there
        assert_eq!(node.data.as_ref().unwrap(), &data[i]);
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
fn test_attr_fn_callback_behavior() {
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
    joined.update
        .attr_fn("custom", |n, i| {
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    svg.attr("width", "100").attr("height", "100");
    let mut g = svg.append("g");
    g.attr("fill", "red");
    let mut rect = g.append("rect");
    rect.attr("x", "10").attr("y", "20").attr("width", "30").attr("height", "40");
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    let mut selection = Selection::root(Rc::clone(&arena), "div");
    selection.append("p").attr("class", "first");
    selection.append("p").attr("class", "second");

    // Raise the first <p>
    {
        let mut children_to_raise = selection.select_all(None);
        children_to_raise.filter(|n: &Node| n.attributes.get("class") == Some(&"first".to_string())).raise();
    }

    let children_after_raise = selection.select_all(None);
    let classes_after_raise: Vec<_> = children_after_raise.nodes_ref().iter().map(|n| n.attributes.get("class").unwrap().clone()).collect();
    assert_eq!(classes_after_raise, vec!["second", "first"]);

    // Lower the first <p> (which is now the second element)
    {
        let mut children_to_lower = selection.select_all(None);
        children_to_lower.filter(|n: &Node| n.attributes.get("class") == Some(&"first".to_string())).lower();
    }

    let children_after_lower = selection.select_all(None);
    let classes_after_lower: Vec<_> = children_after_lower.nodes_ref().iter().map(|n| n.attributes.get("class").unwrap().clone()).collect();
    assert_eq!(classes_after_lower, vec!["first", "second"]);
}

#[test]
fn test_datum() {
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    let mut selection = Selection::root(Rc::clone(&arena), "div");
    selection.append("p");

    selection.classed("foo", true);
    let node = selection.node().unwrap();
    assert_eq!(node.attributes.get("class"), Some(&"foo".to_string()));

    selection.classed("bar", true);
    let node = selection.node().unwrap();
    let classes: HashSet<String> = node.attributes.get("class").unwrap().split_whitespace().map(|s| s.to_string()).collect();
    assert!(classes.contains("foo"));
    assert!(classes.contains("bar"));

    selection.classed("foo", false);
    let node = selection.node().unwrap();
    let classes: HashSet<String> = node.attributes.get("class").unwrap().split_whitespace().map(|s| s.to_string()).collect();
    assert!(!classes.contains("foo"));
    assert!(classes.contains("bar"));
}

#[test]
fn test_style() {
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    let mut selection = Selection::root(Rc::clone(&arena), "div");
    selection.append("p");

    selection.property("foo", "bar");
    let node = selection.node().unwrap();
    assert_eq!(node.properties.get("foo"), Some(&"bar".to_string()));
}

#[test]
fn test_data_join_attr() {
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut rects = svg.append("rect");
    let data = vec![10, 20, 30];
    let mut joined = rects.data(&data);
    joined.enter.append("rect")
        .attr_fn("x", |_, i| (i * 10).to_string());

    let nodes = svg.select_all(Some("rect")).nodes();
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0].attributes.get("x"), Some(&"0".to_string()));
    assert_eq!(nodes[1].attributes.get("x"), Some(&"10".to_string()));
    assert_eq!(nodes[2].attributes.get("x"), Some(&"20".to_string()));
}

#[test]
fn test_data_join_join() {
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut rects = svg.append("rect");
    let data = vec![10, 20, 30];
    let mut joined = rects.data(&data);
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    let mut rects = svg.append("rect");
    let data = vec![10, 20, 30];
    let mut joined = rects.data(&data);
    joined.enter.append("rect")
        .attr_fn("custom", |n, i| {
            format!("{}-{}", n.tag, i)
        });

    let nodes = svg.select_all(Some("rect")).nodes();
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0].attributes.get("custom"), Some(&"rect-0".to_string()));
    assert_eq!(nodes[1].attributes.get("custom"), Some(&"rect-1".to_string()));
    assert_eq!(nodes[2].attributes.get("custom"), Some(&"rect-2".to_string()));
}

#[test]
fn test_empty_selection_no_op() {
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
    let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
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
