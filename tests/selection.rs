use rust_d3::Selection;

#[test]
fn test_selection_api_stubs() {
    let mut sel = Selection::select("svg");
    sel.attr("width", "400").attr("height", "300");
    sel.append("circle").attr("cx", "100").attr("cy", "100").attr("r", "50");
    let _ = sel.enter();
    let _ = sel.exit();
    sel.remove();
    // No panics, API is chainable
}

#[test]
fn test_selection_data_join_enter_exit() {
    let mut sel = Selection::select_all("rect");
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
    let mut sel = Selection::select_all("circle");
    sel.attr("fill", "red").style("stroke", "black");
    for node in &sel.nodes {
        assert_eq!(node.attributes.get("fill").unwrap(), "red");
        assert_eq!(node.styles.get("stroke").unwrap(), "black");
    }
}

#[test]
fn test_selection_append_and_children() {
    let mut sel = Selection::select("g");
    let _rect = sel.append("rect");
    let _circle = sel.append("circle");
    let children = sel.children();
    let tags: Vec<_> = children.nodes.iter().map(|n| n.tag.as_str()).collect();
    assert!(tags.contains(&"rect"));
    assert!(tags.contains(&"circle"));
    assert_eq!(children.nodes.len(), 2);
}

#[test]
fn test_selection_event_on_and_dispatch() {
    use std::sync::{Arc, Mutex};
    let mut sel = Selection::select("rect");
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
    let mut sel = Selection::select_all("rect");
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
    let mut sel = Selection::select_all("circle");
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
    let mut sel = Selection::select_all("rect");
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
    let mut sel = Selection::select("div");
    sel.text("hello");
    assert_eq!(sel.nodes[0].attributes.get("textContent").unwrap(), "hello");
    sel.html("<b>hi</b>");
    assert_eq!(sel.nodes[0].attributes.get("innerHTML").unwrap(), "<b>hi</b>");
}

#[test]
fn test_selection_datum() {
    let mut sel = Selection::select("circle");
    sel.datum("42");
    assert_eq!(sel.nodes[0].data.as_deref().unwrap(), "42");
}

#[test]
fn test_selection_insert_and_call() {
    let mut sel = Selection::select("g");
    sel.insert("rect");
    assert_eq!(sel.nodes[0].children[0].tag, "rect");
    sel.call(|s| { s.attr("foo", "bar"); });
    assert_eq!(sel.nodes[0].attributes.get("foo").unwrap(), "bar");
}

#[test]
fn test_selection_empty_node_size_nodes() {
    let mut sel = Selection::select_all("rect");
    assert!(!sel.empty());
    assert_eq!(sel.size(), 3);
    assert_eq!(sel.node().unwrap().tag, "rect");
    assert_eq!(sel.nodes().len(), 3);
    sel.remove();
    assert!(sel.empty());
}

#[test]
fn test_selection_select_child_and_children() {
    let mut sel = Selection::select("g");
    let rect = sel.append("rect");
    let circle = sel.append("circle");
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
    let sel = Selection::select("rect");
    let parent = sel.select_parent();
    assert!(parent.nodes.is_empty());
    let parents = sel.select_parents();
    assert!(parents.nodes.is_empty());
}

/// Tests for edge cases and advanced D3 selection API usage.
/// Covers empty selections, repeated calls, toggling, and data edge cases.

#[test]
fn test_selection_empty_and_repeated_calls() {
    let mut sel = Selection::select_all("rect");
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
    let mut sel = Selection::select("rect");
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
    let mut sel = Selection::select_all("rect");
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
    let mut sel = Selection::select_all("rect");
    sel.remove();
    let tags: Vec<String> = sel.map(|n| n.tag.clone());
    assert!(tags.is_empty());
    sel.each(|n| { n.attributes.insert("foo".to_string(), "bar".to_string()); });
    // Should not panic or insert anything
}

#[test]
fn test_selection_raise_and_lower() {
    let mut sel = Selection::select_all("rect");
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
    let mut sel = Selection::select_all("rect");
    sel.nodes[0].attributes.insert("id".to_string(), "b".to_string());
    sel.nodes[1].attributes.insert("id".to_string(), "c".to_string());
    sel.nodes[2].attributes.insert("id".to_string(), "a".to_string());
    sel.sort_by(|a, b| a.attributes["id"].cmp(&b.attributes["id"]));
    let ids: Vec<_> = sel.nodes.iter().map(|n| n.attributes.get("id").unwrap().clone()).collect();
    assert_eq!(ids, vec!["a", "b", "c"]);
}

#[test]
fn test_selection_order_noop() {
    let mut sel = Selection::select_all("rect");
    let before = sel.nodes.clone();
    sel.order();
    let after = sel.nodes.clone();
    assert_eq!(before, after); // No-op
}

/*
/// Example: Using selection API for a simulated SVG workflow
///
/// Note: In this simulated model, `append` adds a child to every node in the selection.
/// Thus, after two appends, each node may have both children, not just the root.
/// This test checks for the presence of expected tags and attributes, not strict order.
#[test]
fn test_selection_svg_workflow_example() {
    let mut svg = Selection::select("svg");
    svg.attr("width", "200").attr("height", "100");
    let mut rect = svg.append("rect");
    rect.attr("x", "10").attr("y", "10").attr("width", "50").attr("height", "20");
    let mut circle = svg.append("circle");
    circle.attr("cx", "100").attr("cy", "50").attr("r", "30");
    // Check that the appended rect and circle have the correct attributes
    assert_eq!(rect.nodes[0].tag, "rect");
    assert_eq!(rect.nodes[0].attributes.get("width").unwrap(), "50");
    assert_eq!(circle.nodes[0].tag, "circle");
    assert_eq!(circle.nodes[0].attributes.get("r").unwrap(), "30");
    // The svg's children should include both
    let children = svg.children();
    let tags: Vec<_> = children.nodes.iter().map(|n| n.tag.as_str()).collect();
    assert!(tags.contains(&"rect"));
    assert!(tags.contains(&"circle"));
    // Check that at least one rect has width="50" and one circle has r="30"
    let rect_found = children.nodes.iter().any(|n| n.tag == "rect" && n.attributes.get("width").map(|v| v == "50").unwrap_or(false));
    let circle_found = children.nodes.iter().any(|n| n.tag == "circle" && n.attributes.get("r").map(|v| v == "30").unwrap_or(false));
    assert!(rect_found, "No rect child with width=50 found");
    assert!(circle_found, "No circle child with r=30 found");
}
*/
