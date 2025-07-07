//! Comprehensive tests for d3 Selection
use rust_d3::selection::{AttrSet, StyleSet, PropertySet, ClassedSet, TextSet, HtmlSet};
use rust_d3::selection::Selection;

#[derive(Clone, Debug, PartialEq)]
struct Node {
    id: String,
    class: String,
    style: String,
    property: String,
    text: String,
    html: String,
    classed: bool,
}

impl AttrSet for Node {
    fn set_attr(&mut self, name: &str, value: &str) {
        if name == "id" { self.id = value.to_string(); }
    }
}
impl StyleSet for Node {
    fn set_style(&mut self, name: &str, value: &str) {
        if name == "style" { self.style = value.to_string(); }
    }
}
impl PropertySet for Node {
    fn set_property(&mut self, name: &str, value: &str) {
        if name == "property" { self.property = value.to_string(); }
    }
}
impl ClassedSet for Node {
    fn set_classed(&mut self, name: &str, value: bool) {
        if name == "classed" { self.classed = value; }
    }
}
impl TextSet for Node {
    fn set_text(&mut self, value: &str) { self.text = value.to_string(); }
}
impl HtmlSet for Node {
    fn set_html(&mut self, value: &str) { self.html = value.to_string(); }
}

#[test]
fn test_attr_style_property_classed_text_html() {
    let mut sel = Selection::new(vec![Node {
        id: "foo".to_string(),
        class: "bar".to_string(),
        style: "color: red".to_string(),
        property: "checked".to_string(),
        text: "hello".to_string(),
        html: "<b>hi</b>".to_string(),
        classed: false,
    }]);
    sel.attr("id", "baz");
    sel.style("style", "color: blue");
    sel.property("property", "disabled");
    sel.classed("classed", true);
    sel.text("world");
    sel.html("<i>bye</i>");
    let node = &sel.data()[0];
    assert_eq!(node.id, "baz");
    assert_eq!(node.style, "color: blue");
    assert_eq!(node.property, "disabled");
    assert_eq!(node.classed, true);
    assert_eq!(node.text, "world");
    assert_eq!(node.html, "<i>bye</i>");
}

#[test]
fn test_map_filter() {
    let sel = Selection::new(vec![1, 2, 3]);
    let mapped = sel.map(|x| x * 2);
    assert_eq!(mapped.data(), &vec![2, 4, 6]);
    let filtered = sel.filter(|x| *x > 1);
    assert_eq!(filtered.data(), &vec![2, 3]);
}

#[test]
fn test_select_select_all() {
    let sel = Selection::new(vec![1, 2, 3, 4]);
    let s = sel.select(|x| *x == 2);
    assert_eq!(s.data(), &vec![2]);
    let all = sel.select_all(|x| *x % 2 == 0);
    assert_eq!(all.data(), &vec![2, 4]);
}

#[test]
fn test_size_empty_nodes_node() {
    let sel = Selection::new(vec![1, 2, 3]);
    assert_eq!(sel.size(), 3);
    assert!(!sel.empty());
    assert_eq!(sel.nodes(), vec![&1, &2, &3]);
    assert_eq!(sel.node(), Some(&1));
    let empty = Selection::<i32>::new(vec![]);
    assert!(empty.empty());
    assert_eq!(empty.node(), None);
}

#[test]
fn test_call_each() {
    let sel = Selection::new(vec![10, 20, 30]);
    let mut sum = 0;
    sel.each(|x, _i| sum += x);
    assert_eq!(sum, 60);
    let mut called = false;
    sel.call(|s| {
        assert_eq!(s.size(), 3);
        called = true;
    });
    assert!(called);
}

#[test]
fn test_sort_by_merge() {
    let sel = Selection::new(vec![3, 1, 2]);
    let sorted = sel.sort_by(|a, b| a.cmp(b));
    assert_eq!(sorted.data(), &vec![1, 2, 3]);
    let other = Selection::new(vec![4, 5]);
    let merged = sel.merge(&other);
    assert_eq!(merged.data(), &vec![3, 1, 2, 4, 5]);
}

#[test]
fn test_enter_exit_join() {
    let sel = Selection::new(vec![1, 2, 3]);
    let new_data = vec![2, 3, 4];
    let enter = sel.enter(new_data.clone());
    assert_eq!(enter.data(), &vec![4]);
    let exit = sel.exit(new_data.clone());
    assert_eq!(exit.data(), &vec![1]);
    let (enter2, update, exit2) = sel.join(new_data.clone());
    assert_eq!(enter2.data(), &Vec::<i32>::new()); // index-based join: no enter
    assert_eq!(update.data(), &vec![2, 3, 4]); // update is all new_data
    assert_eq!(exit2.data(), &Vec::<i32>::new()); // no exit
}
