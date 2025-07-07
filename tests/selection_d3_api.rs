//! Tests for D3.js-like API methods on Selection
use rust_d3::selection::{Selection, AttrSet, StyleSet, PropertySet, ClassedSet, TextSet, HtmlSet};

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
