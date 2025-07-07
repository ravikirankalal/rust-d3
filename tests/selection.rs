//! Comprehensive tests for d3 Selection
use rust_d3::selection::{AttrSet, StyleSet, PropertySet, ClassedSet, TextSet, HtmlSet, NodeLike};
use rust_d3::selection::Selection;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

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

#[derive(Debug, Clone)]
struct TreeNode {
    value: i32,
    children: Vec<RcTreeNode>,
    parent: Option<Weak<RefCell<TreeNode>>>,
}

#[derive(Debug, Clone)]
struct RcTreeNode(Rc<RefCell<TreeNode>>);

impl RcTreeNode {
    fn new(value: i32) -> Self {
        RcTreeNode(Rc::new(RefCell::new(TreeNode { value, children: vec![], parent: None })))
    }
}

impl PartialEq for RcTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().value == other.0.borrow().value
    }
}

impl NodeLike for RcTreeNode {
    fn append(&mut self, child: Self) {
        child.0.borrow_mut().parent = Some(Rc::downgrade(&self.0));
        self.0.borrow_mut().children.push(child);
    }
    fn insert(&mut self, index: usize, child: Self) {
        child.0.borrow_mut().parent = Some(Rc::downgrade(&self.0));
        self.0.borrow_mut().children.insert(index, child);
    }
    fn remove(&mut self, index: usize) {
        self.0.borrow_mut().children.remove(index);
    }
    fn clone_node(&self) -> Self {
        RcTreeNode(Rc::new(RefCell::new(TreeNode {
            value: self.0.borrow().value,
            children: vec![],
            parent: None,
        })))
    }
    fn children(&self) -> Vec<Self> {
        self.0.borrow().children.clone()
    }
    fn parent(&self) -> Option<Self> {
        self.0.borrow().parent.as_ref().and_then(|w| w.upgrade()).map(RcTreeNode)
    }
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
    let mut sel = Selection::new(vec![3, 1, 2]);
    sel.sort_by(|a, b| a.cmp(b));
    assert_eq!(sel.data(), &vec![1, 2, 3]);
    let other = Selection::new(vec![4, 5]);
    let merged = sel.merge(&other);
    assert_eq!(merged.data(), &vec![1, 2, 3, 4, 5]);
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

#[test]
fn test_append_insert_remove_clone() {
    let mut root = RcTreeNode::new(1);
    let child1 = RcTreeNode::new(2);
    let child2 = RcTreeNode::new(3);
    root.append(child1.clone());
    root.insert(1, child2.clone());
    root.remove(0);
    let cloned = root.clone_node();
    assert_eq!(cloned.0.borrow().value, root.0.borrow().value);
}

#[test]
fn test_sort_by_merge_raise_lower_order() {
    let mut sel = Selection::new(vec![3, 1, 2]);
    sel.sort_by(|a, b| a.cmp(b));
    assert_eq!(sel.data(), &vec![1, 2, 3]);
    sel.raise(|x| *x == 2);
    assert_eq!(sel.data(), &vec![1, 3, 2]);
    sel.lower(|x| *x == 3);
    assert_eq!(sel.data(), &vec![3, 1, 2]);
    sel.order(); // No-op, but should not panic
    let sel2 = Selection::new(vec![4, 5]);
    let merged = sel.merge(&sel2);
    assert_eq!(merged.data(), &vec![3, 1, 2, 4, 5]);
}

#[test]
fn test_event_api_parity() {
    let mut sel = Selection::new(vec![1, 2, 3]);
    // Should be callable and not panic
    sel.on("click", |_| {});
    sel.dispatch("click");
}

#[test]
fn test_tree_traversal_api_parity() {
    let sel = Selection::new(vec![RcTreeNode::new(1)]);
    // Should be callable and not panic, even if stubs
    let _ = sel.children();
    let _ = sel.parent();
    let _ = sel.ancestors();
    let _ = sel.descendants();
}

#[test]
fn test_join_keyed() {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct Item { id: u32, value: &'static str }
    let old = vec![Item { id: 1, value: "a" }, Item { id: 2, value: "b" }];
    let new = vec![Item { id: 2, value: "b2" }, Item { id: 3, value: "c" }];
    let sel = Selection::new(old.clone());
    let (enter, update, exit) = sel.join_keyed(
        new.clone(),
        |d| d.id,
        |d| d.id,
    );
    assert_eq!(enter.data(), &vec![Item { id: 3, value: "c" }]);
    assert_eq!(update.data(), &vec![Item { id: 2, value: "b2" }]);
    assert_eq!(exit.data(), &vec![Item { id: 1, value: "a" }]);
}

#[test]
fn test_transition_interrupt_api_parity() {
    let mut sel = Selection::new(vec![1, 2, 3]);
    // Should be chainable and not panic
    sel.transition().interrupt();
}

#[test]
fn test_children_and_descendants() {
    let mut root = RcTreeNode::new(1);
    let child1 = RcTreeNode::new(2);
    let mut child2 = RcTreeNode::new(3);
    let grandchild = RcTreeNode::new(4);
    child2.append(grandchild.clone());
    root.append(child1.clone());
    root.append(child2.clone());
    let sel = Selection::new(vec![root.clone()]);
    let children = sel.children();
    assert_eq!(children.data().iter().map(|n| n.0.borrow().value).collect::<Vec<_>>(), vec![2, 3]);
    let descendants = sel.descendants();
    assert_eq!(descendants.data().iter().map(|n| n.0.borrow().value).collect::<Vec<_>>(), vec![2, 3, 4]);
}

#[test]
fn test_parent_and_ancestors() {
    let mut root = RcTreeNode::new(1);
    let mut child = RcTreeNode::new(2);
    let grandchild = RcTreeNode::new(3);
    child.append(grandchild.clone());
    root.append(child.clone());
    let sel = Selection::new(vec![grandchild.clone()]);
    let parent = sel.parent();
    assert_eq!(parent.data()[0].0.borrow().value, 2);
    let ancestors = sel.ancestors();
    let ancestor_values: Vec<i32> = ancestors.data().iter().map(|n| n.0.borrow().value).collect();
    assert!(ancestor_values.contains(&2));
    assert!(ancestor_values.contains(&1));
}
