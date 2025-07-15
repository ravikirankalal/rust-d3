use crate::selection::arena::{Arena, NodeKey};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub is_wildcard: bool,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            tag: None,
            id: None,
            classes: Vec::new(),
            is_wildcard: false,
        }
    }

    pub fn matches(&self, node_tag: &str, node_id: Option<&str>, node_classes: &[String]) -> bool {
        // Wildcard matches everything
        if self.is_wildcard {
            return true;
        }

        // Check tag match
        if let Some(ref tag) = self.tag {
            if node_tag != tag {
                return false;
            }
        }

        // Check ID match
        if let Some(ref id) = self.id {
            if node_id != Some(id) {
                return false;
            }
        }

        // Check class matches - all required classes must be present
        for class in &self.classes {
            if !node_classes.contains(class) {
                return false;
            }
        }

        true
    }
}

/// Parse a CSS selector string into a Selector struct
/// Supports:
/// - Tag selectors: "div", "rect", "g"
/// - Class selectors: ".class", "div.class", "rect.axis.major"
/// - ID selectors: "#id", "div#id", "rect#myid.class"
/// - Wildcard: "*"
/// - Compound selectors: "rect.axis.major#chart"
pub fn parse_selector(selector: &str) -> Selector {
    let mut result = Selector::new();

    // Handle wildcard
    if selector.trim() == "*" {
        result.is_wildcard = true;
        return result;
    }

    let mut remaining = selector;

    // Extract ID if present
    if let Some(id_pos) = remaining.find('#') {
        let (before_id, after_id) = remaining.split_at(id_pos);
        remaining = before_id;

        // Parse ID - everything after # until next . or end
        let id_part = &after_id[1..]; // skip the #
        if let Some(dot_pos) = id_part.find('.') {
            let (id, classes_part) = id_part.split_at(dot_pos);
            result.id = Some(id.to_string());
            // Add classes after ID
            for class in classes_part[1..].split('.') {
                if !class.is_empty() {
                    result.classes.push(class.to_string());
                }
            }
        } else {
            result.id = Some(id_part.to_string());
        }
    }

    // Parse tag and classes from remaining part
    let parts: Vec<&str> = remaining.split('.').collect();

    if !parts.is_empty() {
        let first_part = parts[0].trim();
        if !first_part.is_empty() {
            result.tag = Some(first_part.to_string());
        }

        // Collect all classes
        for part in &parts[1..] {
            if !part.is_empty() {
                result.classes.push(part.to_string());
            }
        }
    }

    result
}

/// Legacy compatibility function - returns (tag, classes) tuple
pub fn parse_selector_legacy(selector: &str) -> (Option<String>, Vec<String>) {
    let parsed = parse_selector(selector);
    (parsed.tag, parsed.classes)
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
    selector: &Selector,
    found: &mut Vec<NodeKey>,
) {
    let arena_borrow = arena.borrow();
    for &key in keys {
        let node = &arena_borrow.nodes[key];

        // Get node ID from attributes
        let node_id = node.attributes.get("id").map(|s| s.as_str());

        // Get node classes
        let node_classes: Vec<String> = node.attributes.get("class").map_or_else(Vec::new, |cls| {
            cls.split_whitespace().map(|s| s.to_string()).collect()
        });

        // Check if node matches selector
        if selector.matches(&node.tag, node_id, &node_classes) {
            found.push(key);
        }

        // Recursively search children
        find_matching_descendants(Rc::clone(&arena), &node.children, selector, found);
    }
}

/// Legacy compatibility function
pub fn find_matching_descendants_legacy(
    arena: Rc<RefCell<Arena>>,
    keys: &Vec<NodeKey>,
    tag: &Option<String>,
    classes: &Vec<String>,
    found: &mut Vec<NodeKey>,
) {
    let selector = Selector {
        tag: tag.clone(),
        id: None,
        classes: classes.clone(),
        is_wildcard: false,
    };
    find_matching_descendants(arena, keys, &selector, found);
}
