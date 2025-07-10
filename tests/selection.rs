use rust_d3::selection::{Arena, Selection};

#[test]
fn test_selection_root_and_attr() {
    let mut arena = Arena { nodes: slotmap::SlotMap::with_key() };
    let mut svg = Selection::root(&mut arena, "svg");
    svg.attr("width", "300").attr("height", "120");
    let node = svg.node().unwrap();
    assert_eq!(node.tag, "svg");
    assert_eq!(node.attributes["width"], "300");
    assert_eq!(node.attributes["height"], "120");
}

#[test]
fn test_append_and_attr_fn() {
    let mut arena = Arena { nodes: slotmap::SlotMap::with_key() };
    let mut svg = Selection::root(&mut arena, "svg");
    let mut group = svg.append("g");
    let mut rects = group.append("rect");
    let data = vec![10, 20, 30];
    rects.data(&data)
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
    let mut arena = Arena { nodes: slotmap::SlotMap::with_key() };
    let mut svg = Selection::root(&mut arena, "svg");
    let mut group = svg.append("g");
    // Collect keys of appended rects to avoid multiple mutable borrows
    let mut rect_keys = Vec::new();
    for _ in 0..3 {
        let rect_sel = group.append("rect");
        rect_keys.extend(rect_sel.iter().cloned());
    }
    let mut rects = Selection::new(&mut arena, rect_keys);
    let data = vec![10, 20, 30];
    rects.data(&data)
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
    let mut arena = Arena { nodes: slotmap::SlotMap::with_key() };
    let mut svg = Selection::root(&mut arena, "svg");
    svg.append("g").append("rect");
    let root_key = svg.iter().next().copied().unwrap();
    let root_node = &arena.nodes[root_key];
    assert_eq!(root_node.tag, "svg");
    assert_eq!(root_node.children.len(), 1);
    let g_key = root_node.children[0];
    let g_node = &arena.nodes[g_key];
    assert_eq!(g_node.tag, "g");
    assert_eq!(g_node.children.len(), 1);
    let rect_key = g_node.children[0];
    let rect_node = &arena.nodes[rect_key];
    assert_eq!(rect_node.tag, "rect");
}

#[test]
fn test_join_basic() {
    let mut arena = Arena { nodes: slotmap::SlotMap::with_key() };
    let mut svg = Selection::root(&mut arena, "svg");
    let mut group = svg.append("g");
    // Start with 2 rects
    let mut rect_keys = Vec::new();
    for _ in 0..2 {
        let rect_sel = group.append("rect");
        rect_keys.extend(rect_sel.iter().cloned());
    }
    let mut rects = Selection::new(&mut arena, rect_keys);
    // Join with 3 data items (should create 1 new rect)
    let data = vec!["A", "B", "C"];
    let mut joined = rects.data(&data);
    let mut updated = joined.join("rect");
    let nodes = updated.nodes();
    assert_eq!(nodes.len(), 3);
    for (i, node) in nodes.iter().enumerate() {
        assert_eq!(node.tag, "rect");
        assert_eq!(node.data.as_ref().unwrap(), &data[i]);
    }
    // Now join with 1 data item (should remove 2 rects)
    let data2 = vec!["Z"];
    let mut joined2 = updated.data(&data2);
    let mut updated2 = joined2.join("rect");
    let nodes2 = updated2.nodes();
    assert_eq!(nodes2.len(), 1);
    assert_eq!(nodes2[0].data.as_ref().unwrap(), "Z");
}

#[test]
fn test_attr_fn_callback_behavior() {
    let mut arena = Arena { nodes: slotmap::SlotMap::with_key() };
    let mut svg = Selection::root(&mut arena, "svg");
    let mut group = svg.append("g");
    let mut rect_keys = Vec::new();
    for _ in 0..4 {
        let rect_sel = group.append("rect");
        rect_keys.extend(rect_sel.iter().cloned());
    }
    let mut rects = Selection::new(&mut arena, rect_keys);
    let data = vec![5, 10, 15, 20];
    rects.data(&data)
        .attr_fn("custom", |n, i| {
            // Test that node data is correct and index is as expected
            let d = n.data.as_ref().unwrap().parse::<i32>().unwrap();
            format!("{}-{}", d, i)
        });
    let nodes = rects.nodes();
    for (i, node) in nodes.iter().enumerate() {
        assert_eq!(node.attributes["custom"], format!("{}-{}", data[i], i));
    }
}
