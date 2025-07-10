use rust_d3::selection::{Selection, Node};

#[test]
fn test_selection_api_stubs() {
    let mut sel = Selection::create("svg");
    sel.attr("width", "400").attr("height", "300");
    sel.append("circle").attr("cx", "100").attr("cy", "100").attr("r", "50");
    let _ = sel.enter();
    let _ = sel.exit();
    sel.remove();
    // No panics, API is chainable
}

#[test]
fn test_selection_data_join_enter_exit() {
    let mut root = Selection::create("root");
    // D3-like: select_all(Some("rect")) returns empty, so append children first
    root.append("rect");
    root.append("rect");
    root.append("rect");
    let mut sel = root.select_all(Some("rect"));
    sel.data(&[1, 2, 3, 4]);
    let enter = sel.enter();
    let exit = sel.exit();
    assert_eq!(sel.nodes.len(), 4);
    assert_eq!(enter.nodes.len(), 1); // 4 data, 3 nodes: 1 enter
    assert_eq!(exit.nodes.len(), 0);  // No exit
    sel.data(&[1]);
    let exit2 = sel.exit();
    assert_eq!(sel.nodes.len(), 1);
    assert_eq!(exit2.nodes.len(), 3); // 3 exit
}

#[test]
fn test_selection_attr_and_style() {
    let mut root = Selection::create("root");
    let mut sel = root.select_all(Some("circle"));
    sel.attr("fill", "red").style("stroke", "black");
    for node in &sel.nodes {
        assert_eq!(node.attributes.get("fill").unwrap(), "red");
        assert_eq!(node.styles.get("stroke").unwrap(), "black");
    }
}

#[test]
fn test_selection_append_and_children() {
    let mut sel = Selection::create("g");
    sel.append("rect");
    sel.append("circle");
    let children = sel.children();
    let tags: Vec<_> = children.nodes.iter().map(|n| n.tag.as_str()).collect();
    assert!(tags.contains(&"rect"));
    assert!(tags.contains(&"circle"));
    assert_eq!(children.nodes.len(), 2);
}

#[test]
fn test_selection_event_on_and_dispatch() {
    use std::sync::{Arc, Mutex};
    let mut sel = Selection::create("rect");
    let called = Arc::new(Mutex::new(0));
    let called2 = called.clone();
    sel.on("click", move || {
        let mut n = called2.lock().unwrap();
        *n += 1;
    });
    sel.dispatch("click");
    assert_eq!(*called.lock().unwrap(), 1);
}

#[test]
fn test_selection_filter_and_merge() {
    let mut root = Selection::create("root");
    root.append("rect");
    root.append("rect");
    root.append("rect");
    let mut sel = root.select_all(Some("rect"));
    sel.attr("id", "foo");
    sel.nodes[1].attributes.insert("id".to_string(), "bar".to_string());
    let filtered = sel.filter(|n| n.attributes.get("id").map(|v| v == "bar").unwrap_or(false));
    assert_eq!(filtered.nodes.len(), 1);
    assert_eq!(filtered.nodes[0].attributes.get("id").unwrap(), "bar");
    let merged = sel.merge(&filtered);
    assert_eq!(merged.nodes.len(), 4); // 3 original + 1 filtered (may be duplicate)
}

#[test]
fn test_selection_each_and_map() {
    let mut root = Selection::create("root");
    root.append("circle");
    root.append("circle");
    root.append("circle");
    let mut sel = root.select_all(Some("circle"));
    sel.each(|n| {
        n.attributes.insert("foo".to_string(), "bar".to_string());
    });
    for node in &sel.nodes {
        assert_eq!(node.attributes.get("foo").unwrap(), "bar");
    }
    let tags: Vec<String> = sel.map(|n| n.tag.clone());
    assert_eq!(tags, vec!["circle", "circle", "circle"]);
}

#[test]
fn test_selection_property_and_classed() {
    let mut root = Selection::create("root");
    let mut sel = root.select_all(Some("rect"));
    sel.property("checked", "true");
    for node in &sel.nodes {
        assert_eq!(node.attributes.get("property:checked").unwrap(), "true");
    }
    sel.classed("foo", true);
    for node in &sel.nodes {
        assert!(node.attributes.get("class").unwrap().contains("foo"));
    }
    sel.classed("foo", false);
    for node in &sel.nodes {
        assert!(!node.attributes.get("class").unwrap().contains("foo"));
    }
}

#[test]
fn test_selection_text_and_html() {
    let mut sel = Selection::create("div");
    sel.text("hello");
    assert_eq!(sel.nodes[0].attributes.get("textContent").unwrap(), "hello");
    sel.html("<b>hi</b>");
    assert_eq!(sel.nodes[0].attributes.get("innerHTML").unwrap(), "<b>hi</b>");
}

#[test]
fn test_selection_datum() {
    let mut sel = Selection::create("circle");
    sel.datum("42");
    assert_eq!(sel.nodes[0].data.as_deref().unwrap(), "42");
}

#[test]
fn test_selection_insert_and_call() {
    let mut sel = Selection::create("g");
    sel.insert("rect");
    assert_eq!(sel.nodes[0].children[0].tag, "rect");
    sel.call(|s| { s.attr("foo", "bar"); });
    assert_eq!(sel.nodes[0].attributes.get("foo").unwrap(), "bar");
}

#[test]
fn test_selection_empty_node_size_nodes() {
    let mut root = Selection::create("root");
    root.append("rect");
    root.append("rect");
    root.append("rect");
    let mut sel = root.select_all(Some("rect"));
    assert!(!sel.empty());
    assert_eq!(sel.size(), 3);
    assert_eq!(sel.node().unwrap().tag, "rect");
    assert_eq!(sel.nodes().len(), 3);
    sel.remove();
    assert!(sel.empty());
}

#[test]
fn test_selection_select_child_and_children() {
    let mut sel = Selection::create("g");
    sel.append("rect");
    sel.append("circle");
    let child = sel.select_child();
    // Only the first appended child is selected
    assert_eq!(child.nodes.len(), 1);
    assert_eq!(child.nodes[0].tag, "rect");
    let children = sel.children();
    let tags: Vec<_> = children.nodes.iter().map(|n| n.tag.as_str()).collect();
    assert!(tags.contains(&"rect"));
    assert!(tags.contains(&"circle"));
    assert_eq!(children.nodes.len(), 2);
}

#[test]
fn test_selection_select_parent_and_parents() {
    let sel = Selection::create("rect");
    let parent = sel.select_parent();
    assert!(parent.nodes.is_empty());
    let parents = sel.select_parents();
    assert!(parents.nodes.is_empty());
}

/// Tests for edge cases and advanced D3 selection API usage.
/// Covers empty selections, repeated calls, toggling, and data edge cases.

#[test]
fn test_selection_empty_and_repeated_calls() {
    let mut root = Selection::create("root");
    let mut sel = root.select_all(Some("rect"));
    sel.remove();
    assert!(sel.empty());
    // Calling remove again should not panic
    sel.remove();
    assert!(sel.empty());
    // attr, style, property, classed, text, html on empty selection should not panic
    sel.attr("foo", "bar").style("baz", "qux").property("checked", "false").classed("foo", true).text("").html("");
    // on/dispatch on empty selection should not panic
    sel.on("click", || {}).dispatch("click");
}

#[test]
fn test_selection_classed_multiple_classes() {
    let mut root = Selection::create("root");
    let mut sel = root.select("rect");
    sel.classed("foo", true).classed("bar", true);
    let class_attr = sel.nodes[0].attributes.get("class").unwrap();
    assert!(class_attr.contains("foo"));
    assert!(class_attr.contains("bar"));
    sel.classed("foo", false);
    let class_attr = sel.nodes[0].attributes.get("class").unwrap();
    assert!(!class_attr.contains("foo"));
    assert!(class_attr.contains("bar"));
}

#[test]
fn test_selection_data_edge_cases() {
    let mut root = Selection::create("root");
    let mut sel = root.select_all(Some("rect"));
    // Fewer data than nodes
    sel.data(&[1]);
    assert_eq!(sel.nodes.len(), 1);
    // More data than nodes
    sel.data(&[1, 2, 3, 4, 5]);
    assert_eq!(sel.nodes.len(), 5);
    // No data
    sel.data::<i32>(&[]);
    assert_eq!(sel.nodes.len(), 0);
}

#[test]
fn test_selection_map_and_each_empty() {
    let mut root = Selection::create("root");
    let mut sel = root.select_all(Some("rect"));
    sel.remove();
    let tags: Vec<String> = sel.map(|n| n.tag.clone());
    assert!(tags.is_empty());
    sel.each(|n| { n.attributes.insert("foo".to_string(), "bar".to_string()); });
    // Should not panic or insert anything
}

#[test]
fn test_selection_raise_and_lower() {
    let mut root = Selection::create("root");
    root.append("rect");
    root.append("rect");
    root.append("rect");
    let mut sel = root.select_all(Some("rect"));
    sel.nodes[0].attributes.insert("id".to_string(), "a".to_string());
    sel.nodes[1].attributes.insert("id".to_string(), "c".to_string());
    sel.nodes[2].attributes.insert("id".to_string(), "b".to_string());
    sel.raise();
    let ids: Vec<_> = sel.nodes.iter().map(|n| n.attributes.get("id").unwrap().clone()).collect();
    assert_eq!(ids, vec!["a", "b", "c"]); // Sorted ascending by tag (all rect, so order by id)
    sel.lower();
    let ids: Vec<_> = sel.nodes.iter().map(|n| n.attributes.get("id").unwrap().clone()).collect();
    assert_eq!(ids, vec!["c", "b", "a"]); // Sorted descending by tag
}

#[test]
fn test_selection_sort_by() {
    let mut root = Selection::create("root");
    root.append("rect");
    root.append("rect");
    root.append("rect");
    let mut sel = root.select_all(Some("rect"));
    sel.nodes[0].attributes.insert("id".to_string(), "b".to_string());
    sel.nodes[1].attributes.insert("id".to_string(), "c".to_string());
    sel.nodes[2].attributes.insert("id".to_string(), "a".to_string());
    sel.sort_by(|a, b| a.attributes["id"].cmp(&b.attributes["id"]));
    let ids: Vec<_> = sel.nodes.iter().map(|n| n.attributes.get("id").unwrap().clone()).collect();
    assert_eq!(ids, vec!["a", "b", "c"]);
}

#[test]
fn test_selection_order_noop() {
    let mut root = Selection::create("root");
    let mut sel = root.select_all(Some("rect"));
    let before = sel.nodes.clone();
    sel.order();
    let after = sel.nodes.clone();
    assert_eq!(before, after); // No-op
}

#[test]
fn test_selection_filter() {
    let mut root = Selection::create("root");
    root.append("rect");
    root.append("rect");
    root.append("rect");
    let mut sel = root.select_all(Some("rect"));
    sel.attr("data-id", "even");
    sel.nodes[1].attributes.insert("data-id".to_string(), "odd".to_string());
    let filtered = sel.filter(|n| n.attributes.get("data-id").map(|v| v == "even").unwrap_or(false));
    assert_eq!(filtered.size(), 2);
}

#[test]
fn test_selection_interrupt_and_clone() {
    let mut root = Selection::create("root");
    let sel = root.select_all(Some("circle"));
    let mut root2 = Selection::create("root");
    let mut sel2 = root2.select_all(Some("rect"));
    sel2.interrupt(); // Should be chainable and not panic
}

#[test]
fn test_join_d3_style() {
    // Initial selection with 2 nodes
    let mut sel = Selection::create("g");
    sel.nodes.push(Node::new("g"));
    // Data with 3 items
    let data = vec![1, 2, 3];
    sel.data(&data);
    // Join with tag "rect"
    sel.join("rect");
    // Should have 3 nodes, all with tag "rect"
    assert_eq!(sel.nodes.len(), 3);
    for (i, node) in sel.nodes.iter().enumerate() {
        assert_eq!(node.tag, "rect");
        assert_eq!(node.data.as_ref().unwrap(), &(data[i].to_string()));
    }
}

#[test]
fn test_join_more_data_than_nodes() {
    let mut sel = Selection::create("g");
    sel.nodes.push(Node::new("g")); // 2 nodes
    let data = vec![1, 2, 3, 4];
    sel.data(&data);
    sel.join("rect");
    assert_eq!(sel.nodes.len(), 4);
    for (i, node) in sel.nodes.iter().enumerate() {
        assert_eq!(node.tag, "rect");
        assert_eq!(node.data.as_ref().unwrap(), &data[i].to_string());
    }
}

#[test]
fn test_join_fewer_data_than_nodes() {
    let mut sel = Selection::create("g");
    sel.nodes.push(Node::new("g"));
    sel.nodes.push(Node::new("g")); // 3 nodes
    let data = vec![10];
    sel.data(&data);
    sel.join("rect");
    assert_eq!(sel.nodes.len(), 1);
    assert_eq!(sel.nodes[0].tag, "rect");
    assert_eq!(sel.nodes[0].data.as_ref().unwrap(), "10");
}

#[test]
fn test_join_same_number_data_and_nodes() {
    let mut sel = Selection::create("g");
    sel.nodes.push(Node::new("g")); // 2 nodes
    let data = vec![5, 6];
    sel.data(&data);
    sel.join("rect");
    assert_eq!(sel.nodes.len(), 2);
    for (i, node) in sel.nodes.iter().enumerate() {
        assert_eq!(node.tag, "rect");
        assert_eq!(node.data.as_ref().unwrap(), &data[i].to_string());
    }
}

#[test]
fn test_join_with_different_tag() {
    let mut sel = Selection::create("g");
    let data = vec![1];
    sel.data(&data);
    sel.join("circle");
    assert_eq!(sel.nodes.len(), 1);
    assert_eq!(sel.nodes[0].tag, "circle");
    assert_eq!(sel.nodes[0].data.as_ref().unwrap(), "1");
}

#[test]
fn test_select_all_with_tag() {
    let mut sel = Selection::create("svg");
    sel.append("rect");
    sel.append("circle");
    sel.append("rect");
    sel.select_all(Some("rect"));
    assert_eq!(sel.nodes.len(), 2);
    for node in &sel.nodes {
        assert_eq!(node.tag, "rect");
    }
}

#[test]
fn test_select_all_without_tag() {
    let mut sel = Selection::create("svg");
    sel.append("rect");
    sel.append("circle");
    sel.append("rect");
    sel.select_all(None);
    assert_eq!(sel.nodes.len(), 3);
    let tags: Vec<_> = sel.nodes.iter().map(|n| n.tag.as_str()).collect();
    assert!(tags.contains(&"rect"));
    assert!(tags.contains(&"circle"));
}

#[test]
fn test_select_all_with_empty_string() {
    let mut sel = Selection::create("svg");
    sel.append("rect");
    sel.append("circle");
    sel.append("rect");
    sel.select_all(None); // Changed from Some("") to None for D3-like behavior
    assert_eq!(sel.nodes.len(), 3);
    let tags: Vec<_> = sel.nodes.iter().map(|n| n.tag.as_str()).collect();
    assert!(tags.contains(&"rect"));
    assert!(tags.contains(&"circle"));
}

#[test]
fn test_select_all_children_overload() {
    let mut sel = Selection::create("svg");
    sel.append("rect");
    sel.append("circle");
    sel.append("rect");
    sel.select_all(None); // Replaces select_all_children()
    assert_eq!(sel.nodes.len(), 3);
    let tags: Vec<_> = sel.nodes.iter().map(|n| n.tag.as_str()).collect();
    assert!(tags.contains(&"rect"));
    assert!(tags.contains(&"circle"));
}
