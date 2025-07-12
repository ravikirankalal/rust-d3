use crate::selection::arena::{Arena, NodeKey};
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse_selector(selector: &str) -> (Option<String>, Vec<String>) {
    let mut tag = None;
    let mut classes = Vec::new();
    for part in selector.split('.') {
        if tag.is_none() {
            if !part.is_empty() {
                tag = Some(part.to_string());
            }
        } else {
            if !part.is_empty() {
                classes.push(part.to_string());
            }
        }
    }
    (tag, classes)
}

pub fn remove_node_recursively(arena: &mut Arena, key: NodeKey) {
    let children = arena.nodes[key].children.clone();
    for child_key in children {
        remove_node_recursively(arena, child_key);
    }
    arena.nodes.remove(key);
}

pub fn find_matching_descendants(
    arena: Rc<RefCell<Arena>>,
    keys: &Vec<NodeKey>,
    tag: &Option<String>,
    classes: &Vec<String>,
    found: &mut Vec<NodeKey>,
) {
    let arena_borrow = arena.borrow();
    for &key in keys {
        let node = &arena_borrow.nodes[key];
        let tag_match = tag.as_ref().map_or(true, |t| &node.tag == t);
        let class_match = if classes.is_empty() {
            true
        } else {
            node.attributes.get("class").map_or(false, |cls| {
                let node_classes: Vec<&str> = cls.split_whitespace().collect();
                classes.iter().all(|c| node_classes.contains(&c.as_str()))
            })
        };
        if tag_match && class_match {
            found.push(key);
        }
        find_matching_descendants(Rc::clone(&arena), &node.children, tag, classes, found);
    }
}
