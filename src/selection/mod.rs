// Arena-based D3-like selection module for Rust
// This is a new module, not a drop-in replacement for the current mod.rs
// You can migrate your code to use this for true D3-like chaining and live selections

use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;

new_key_type! { pub struct NodeKey; }

#[derive(Clone)]
pub struct Node {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub data: Option<String>,
    pub children: Vec<NodeKey>,
    pub parent: Option<NodeKey>,
    pub text: Option<String>, // Add text field for node
}

pub struct Arena {
    pub nodes: SlotMap<NodeKey, Node>,
}

pub struct Selection<'a> {
    arena: &'a mut Arena, // private
    keys: Vec<NodeKey>,  // private
    pending_data: Option<Vec<String>>, // store pending data for join
}

impl Node {
    pub fn new(tag: &str) -> Self {
        Node {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            data: None,
            children: vec![],
            parent: None,
            text: None, // Initialize text field
        }
    }
}

impl<'a> Selection<'a> {
    /// Create a new selection from arena and keys (usually root node)
    pub fn new(arena: &'a mut Arena, keys: Vec<NodeKey>) -> Self {
        Selection { arena, keys, pending_data: None }
    }

    /// Create a root node and return a root selection
    pub fn root(arena: &'a mut Arena, tag: &str) -> Self {
        let root = Node {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            data: None,
            children: vec![],
            parent: None,
            text: None, // Initialize text field
        };
        let root_key = arena.nodes.insert(root);
        Selection { arena, keys: vec![root_key], pending_data: None }
    }

    /// Get the number of selected nodes
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
    // Optionally, expose an iterator for read-only traversal
    pub fn iter(&self) -> impl Iterator<Item = &NodeKey> {
        self.keys.iter()
    }

    pub fn append(&mut self, tag: &str) -> Selection<'_> {
        let mut new_keys = Vec::new();
        for &key in &self.keys {
            let child = Node {
                tag: tag.to_string(),
                attributes: HashMap::new(),
                data: None,
                children: vec![],
                parent: Some(key),
                text: None, // Initialize text field
            };
            let child_key = self.arena.nodes.insert(child);
            self.arena.nodes[key].children.push(child_key);
            new_keys.push(child_key);
        }
        Selection { arena: self.arena, keys: new_keys, pending_data: None }
    }

    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        for &key in &self.keys {
            self.arena.nodes[key].attributes.insert(name.to_string(), value.to_string());
        }
        self
    }

    pub fn attr_fn<F>(&mut self, name: &str, mut f: F) -> &mut Self
    where
        F: FnMut(&Node, usize) -> String,
    {
        for (i, &key) in self.keys.iter().enumerate() {
            let value = f(&self.arena.nodes[key], i);
            self.arena.nodes[key].attributes.insert(name.to_string(), value);
        }
        self
    }

    pub fn select_all(&mut self, tag: Option<&str>) -> Selection<'_> {
        let mut found = Vec::new();
        for &key in &self.keys {
            for &child_key in &self.arena.nodes[key].children {
                if tag.map_or(true, |t| self.arena.nodes[child_key].tag == t) {
                    found.push(child_key);
                }
            }
        }
        Selection { arena: self.arena, keys: found, pending_data: None }
    }

    pub fn data<T: ToString>(&mut self, data: &[T]) -> &mut Self {
        self.pending_data = Some(data.iter().map(|d| d.to_string()).collect());
        // For compatibility, update existing nodes' data as before
        for (i, &key) in self.keys.iter().enumerate() {
            if let Some(d) = data.get(i) {
                self.arena.nodes[key].data = Some(d.to_string());
            }
        }
        self
    }

    pub fn join(&mut self, tag: &str) -> &mut Self {
        // If no children, use the current selection's node as parent
        let parent = if self.keys.is_empty() {
            // Use the first key from the previous selection (the parent group)
            // This assumes the selection is a single parent node
            self.arena.nodes.keys().next()
        } else {
            self.keys.get(0).and_then(|k| self.arena.nodes[*k].parent)
        };
        if let Some(parent_key) = parent {
            // Use pending_data if present, else fallback to current node data
            let data_vec: Vec<Option<String>> = if let Some(ref pd) = self.pending_data {
                pd.iter().map(|d| Some(d.clone())).collect()
            } else {
                self.keys.iter().map(|k| self.arena.nodes[*k].data.clone()).collect()
            };
            self.arena.nodes[parent_key].children.clear();
            let mut new_keys = Vec::new();
            for data in data_vec {
                let node = Node {
                    tag: tag.to_string(),
                    attributes: HashMap::new(),
                    data,
                    children: vec![],
                    parent: Some(parent_key),
                    text: None, // Initialize text field
                };
                let new_key = self.arena.nodes.insert(node);
                self.arena.nodes[parent_key].children.push(new_key);
                new_keys.push(new_key);
            }
            self.keys = new_keys;
            self.pending_data = None;
        }
        self
    }

    /// D3-like create constructor for tests
    pub fn create(tag: &str) -> Selection<'a> {
        // For test compatibility: create a new Arena and root node
        let arena = Box::leak(Box::new(Arena { nodes: SlotMap::with_key() }));
        let root = Node::new(tag);
        let root_key = arena.nodes.insert(root);
        Selection { arena, keys: vec![root_key], pending_data: None }
    }

    /// D3-like nodes accessor for tests
    pub fn nodes(&self) -> Vec<Node> {
        self.keys.iter().map(|k| self.arena.nodes[*k].clone()).collect()
    }

    /// D3-like nodes accessor for tests (returns Vec<&Node> for compatibility)
    pub fn nodes_ref(&self) -> Vec<&Node> {
        self.keys.iter().map(|k| &self.arena.nodes[*k]).collect()
    }

    // Stub D3-like methods for test compatibility
    pub fn enter(&mut self) -> Selection<'_> { Selection { arena: self.arena, keys: vec![], pending_data: None } }
    pub fn exit(&mut self) -> Selection<'_> { Selection { arena: self.arena, keys: vec![], pending_data: None } }
    pub fn remove(&mut self) -> &mut Self {
        println!("[Selection::remove] Arena nodes before: {}", self.arena.nodes.len());
        println!("[Selection::remove] Keys to remove: {:?}", self.keys);
        for &key in &self.keys {
            // Debug: print node tag and class before removal
            if let Some(node) = self.arena.nodes.get(key) {
                let tag = &node.tag;
                let class = node.attributes.get("class").cloned().unwrap_or_default();
                println!("[Selection::remove] Removing node: <{} class=\"{}\">", tag, class);
            }
            // Remove from parent's children
            if let Some(parent_key) = self.arena.nodes[key].parent {
                let parent = &mut self.arena.nodes[parent_key];
                parent.children.retain(|&c| c != key);
            }
            // Remove all children recursively
            remove_node_recursively(self.arena, key);
        }
        println!("[Selection::remove] Arena nodes after: {}", self.arena.nodes.len());
        self.keys.clear();
        self
    }
    pub fn style(&mut self, _name: &str, _value: &str) -> &mut Self { self }
    pub fn property(&mut self, _name: &str, _value: &str) -> &mut Self { self }
    pub fn classed(&mut self, _name: &str, _on: bool) -> &mut Self { self }
    pub fn text(&mut self, value: &str) -> &mut Self {
        for key in &self.keys {
            let node = &mut self.arena.nodes[*key];
            node.text = Some(value.to_string());
        }
        self
    }
    pub fn html(&mut self, _value: &str) -> &mut Self { self }
    pub fn insert(&mut self, _tag: &str) -> &mut Self { self }
    pub fn call<F: FnOnce(&mut Self)>(&mut self, f: F) -> &mut Self { f(self); self }
    pub fn empty(&self) -> bool { self.keys.is_empty() }
    pub fn size(&self) -> usize { self.keys.len() }
    pub fn node(&self) -> Option<&Node> { self.keys.get(0).map(|k| &self.arena.nodes[*k]) }

    pub fn each<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut Node),
    {
        for key in &self.keys {
            f(&mut self.arena.nodes[*key]);
        }
        self
    }

    pub fn map<F, T>(&self, mut f: F) -> Vec<T>
    where
        F: FnMut(&Node) -> T,
    {
        self.keys.iter().map(|k| f(&self.arena.nodes[*k])).collect()
    }

    pub fn filter<F>(&mut self, mut f: F) -> Selection<'_>
    where
        F: FnMut(&Node) -> bool,
    {
        let filtered: Vec<NodeKey> = self.keys.iter().cloned().filter(|k| f(&self.arena.nodes[*k])).collect();
        Selection { arena: self.arena, keys: filtered, pending_data: None }
    }

    pub fn merge(&mut self, other: &Selection) -> Selection<'_> {
        let mut merged = self.keys.clone();
        merged.extend(other.keys.iter().cloned());
        Selection { arena: self.arena, keys: merged, pending_data: None }
    }

    pub fn children(&mut self) -> Selection<'_> {
        let mut child_keys = Vec::new();
        for &key in &self.keys {
            child_keys.extend(self.arena.nodes[key].children.iter().cloned());
        }
        Selection { arena: self.arena, keys: child_keys, pending_data: None }
    }
    pub fn select_child(&mut self, tag: &str) -> Selection<'_> {
        let mut child_keys = Vec::new();
        for &key in &self.keys {
            for &child_key in &self.arena.nodes[key].children {
                if self.arena.nodes[child_key].tag == tag {
                    child_keys.push(child_key);
                }
            }
        }
        Selection { arena: self.arena, keys: child_keys, pending_data: None }
    }
    pub fn parent(&mut self) -> Selection<'_> {
        let mut parent_keys = Vec::new();
        for &key in &self.keys {
            if let Some(parent) = self.arena.nodes[key].parent {
                parent_keys.push(parent);
            }
        }
        Selection { arena: self.arena, keys: parent_keys, pending_data: None }
    }
    pub fn select_parent(&mut self, tag: &str) -> Selection<'_> {
        let mut parent_keys = Vec::new();
        for &key in &self.keys {
            if let Some(parent) = self.arena.nodes[key].parent {
                if self.arena.nodes[parent].tag == tag {
                    parent_keys.push(parent);
                }
            }
        }
        Selection { arena: self.arena, keys: parent_keys, pending_data: None }
    }
    pub fn clone_selection(&mut self) -> Selection<'_> {
        Selection { arena: self.arena, keys: self.keys.clone(), pending_data: None }
    }
    pub fn find_all(&mut self, tag: &str) -> Selection<'_> {
        let mut found = Vec::new();
        for &key in &self.keys {
            for &child_key in &self.arena.nodes[key].children {
                if self.arena.nodes[child_key].tag == tag {
                    found.push(child_key);
                }
            }
        }
        Selection { arena: self.arena, keys: found, pending_data: None }
    }

    /// D3-like select: select the first child with the given tag
    pub fn select(&mut self, tag: &str) -> Selection<'_> {
        let mut found = Vec::new();
        for &key in &self.keys {
            if let Some(&child_key) = self.arena.nodes[key].children.iter().find(|&&c| self.arena.nodes[c].tag == tag) {
                found.push(child_key);
            }
        }
        Selection { arena: self.arena, keys: found, pending_data: None }
    }

    /// D3-like selector: supports tag, .class, tag.class, and multiple classes
    /// Now supports recursive search for matching descendants
    pub fn select_by(&mut self, selector: &str) -> Selection<'_> {
        let mut found = Vec::new();
        let (tag, classes) = parse_selector(selector);
        for &key in &self.keys {
            find_matching_descendants(self.arena, key, &tag, &classes, &mut found);
        }
        Selection { arena: self.arena, keys: found, pending_data: None }
    }

    /// D3-like sort_by: sort nodes by a comparator
    pub fn sort_by<F>(&mut self, mut cmp: F) -> &mut Self
    where
        F: FnMut(&Node, &Node) -> std::cmp::Ordering,
    {
        self.keys.sort_by(|a, b| cmp(&self.arena.nodes[*a], &self.arena.nodes[*b]));
        self
    }

    /// D3-like order: restore document order (no-op for now)
    pub fn order(&mut self) -> &mut Self {
        self
    }

    pub fn render_node(arena: &Arena, key: NodeKey) -> String {
        let node = &arena.nodes[key];
        let mut s = String::new();
        s.push('<');
        s.push_str(&node.tag);
        for (k, v) in &node.attributes {
            s.push(' ');
            s.push_str(k);
            s.push_str("=\"");
            s.push_str(v);
            s.push('"');
        }
        if node.children.is_empty() && node.text.is_none() {
            s.push_str("/>");
            return s;
        }
        s.push('>');
        // Insert text content if present
        if let Some(ref text) = node.text {
            s.push_str(text);
        }
        for &child in &node.children {
            s.push_str(&Self::render_node(arena, child));
        }
        s.push_str(&format!("</{}>", node.tag));
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_render_node_svg() {
        let mut arena = Arena { nodes: SlotMap::with_key() };
        let mut svg = Selection::root(&mut arena, "svg");
        svg.attr("width", "100").attr("height", "100");
        let mut g = svg.append("g");
        g.attr("fill", "red");
        let mut rect = g.append("rect");
        rect.attr("x", "10").attr("y", "20").attr("width", "30").attr("height", "40");
        let root_key = svg.iter().next().copied().unwrap();
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
}

/// Parse selector string into tag and classes
fn parse_selector(selector: &str) -> (Option<String>, Vec<String>) {
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

/// Recursively remove node and its children from arena
fn remove_node_recursively(arena: &mut Arena, key: NodeKey) {
    let children = arena.nodes[key].children.clone();
    for child_key in children {
        remove_node_recursively(arena, child_key);
    }
    arena.nodes.remove(key);
}

/// Recursively find matching descendants by tag and classes
fn find_matching_descendants(
    arena: &Arena,
    key: NodeKey,
    tag: &Option<String>,
    classes: &Vec<String>,
    found: &mut Vec<NodeKey>,
) {
    let node = &arena.nodes[key];
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
    for &child_key in &node.children {
        find_matching_descendants(arena, child_key, tag, classes, found);
    }
}
