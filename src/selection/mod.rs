// Arena-based D3-like selection module for Rust
// This is a new module, not a drop-in replacement for the current mod.rs
// You can migrate your code to use this for true D3-like chaining and live selections

use slotmap::{SlotMap, new_key_type};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

new_key_type! { pub struct NodeKey; }

#[derive(Clone)]
pub struct Node {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub properties: HashMap<String, String>,
    pub data: Option<String>,
    pub children: Vec<NodeKey>,
    pub parent: Option<NodeKey>,
    pub text: Option<String>, // Add text field for node
}

pub struct Arena {
    pub nodes: SlotMap<NodeKey, Node>,
}

pub struct DataJoin {
    pub update: Selection,
    pub enter: Selection,
    pub exit: Selection,
}

pub struct Selection {
    arena: Rc<RefCell<Arena>>, // private
    keys: Vec<NodeKey>,  // private
    pending_data: Option<Vec<String>>, // store pending data for join
}

impl Node {
    pub fn new(tag: &str) -> Self {
        Node {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            properties: HashMap::new(),
            data: None,
            children: vec![],
            parent: None,
            text: None, // Initialize text field
        }
    }
}

impl Selection {
    /// Create a new selection from arena and keys (usually root node)
    pub fn new(arena: Rc<RefCell<Arena>>, keys: Vec<NodeKey>) -> Self {
        Selection { arena, keys, pending_data: None }
    }

    /// Create a root node and return a root selection
    pub fn root(arena: Rc<RefCell<Arena>>, tag: &str) -> Self {
        let root = Node {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            properties: HashMap::new(),
            data: None,
            children: vec![],
            parent: None,
            text: None, // Initialize text field
        };
        let root_key = arena.borrow_mut().nodes.insert(root);
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

    pub fn append(&mut self, tag: &str) -> Selection {
        let mut new_keys = Vec::new();
        for &key in &self.keys {
            let child = Node {
                tag: tag.to_string(),
                attributes: HashMap::new(),
                properties: HashMap::new(),
                data: None,
                children: vec![],
                parent: Some(key),
                text: None, // Initialize text field
            };
            let child_key = self.arena.borrow_mut().nodes.insert(child);
            self.arena.borrow_mut().nodes[key].children.push(child_key);
            new_keys.push(child_key);
        }
        Selection { arena: Rc::clone(&self.arena), keys: new_keys, pending_data: None }
    }

    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                arena.nodes[key].attributes.insert(name.to_string(), value.to_string());
            }
        }
        self
    }

    pub fn attr_fn<F>(&mut self, name: &str, mut f: F) -> &mut Self
    where
        F: FnMut(&Node, usize) -> String,
    {
        {
            let mut arena = self.arena.borrow_mut();
            for (i, &key) in self.keys.iter().enumerate() {
                let value = f(&arena.nodes[key], i);
                arena.nodes[key].attributes.insert(name.to_string(), value);
            }
        }
        self
    }

    pub fn select_all(&mut self, tag: Option<&str>) -> Selection {
        let mut found = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            for &child_key in &arena.nodes[key].children {
                if tag.map_or(true, |t| arena.nodes[child_key].tag == t) {
                    found.push(child_key);
                }
            }
        }
        Selection { arena: Rc::clone(&self.arena), keys: found, pending_data: None }
    }

    pub fn data<T: ToString>(&mut self, data: &[T]) -> DataJoin {
        let mut enter_keys = Vec::new();
        let mut update_keys = Vec::new();
        let mut exit_keys = Vec::new();

        let mut data_iter = data.iter().map(|d| d.to_string()).peekable();
        let mut node_iter = self.keys.iter().peekable();

        let mut arena = self.arena.borrow_mut();

        // Simple join by index for now, assuming data and nodes are ordered
        // This needs to be more sophisticated for true D3-like behavior (key function)
        let mut _i = 0;
        while let Some(d) = data_iter.next() {
            if let Some(&node_key) = node_iter.peek() {
                // If there's a node, it's an update
                arena.nodes[*node_key].data = Some(d);
                update_keys.push(*node_key);
                node_iter.next();
            } else {
                // No matching node, it's an enter placeholder
                // Create a placeholder node in the arena for the enter selection
                let placeholder_node = Node {
                    tag: "".to_string(), // Placeholder tag
                    attributes: HashMap::new(),
                    properties: HashMap::new(),
                    data: Some(d),
                    children: vec![],
                    parent: None,
                    text: None,
                };
                let placeholder_key = arena.nodes.insert(placeholder_node);
                enter_keys.push(placeholder_key);
            }
            _i += 1;
        }

        // Any remaining nodes are exit nodes
        while let Some(&node_key) = node_iter.next() {
            exit_keys.push(node_key);
        }

        // Create the selections
        let update_selection = Selection { arena: Rc::clone(&self.arena), keys: update_keys, pending_data: None };
        let enter_selection = Selection { arena: Rc::clone(&self.arena), keys: enter_keys, pending_data: None };
        let exit_selection = Selection { arena: Rc::clone(&self.arena), keys: exit_keys, pending_data: None };

        DataJoin { update: update_selection, enter: enter_selection, exit: exit_selection }
    }

    pub fn datum<T: ToString>(&mut self, value: T) -> &mut Self {
        {
            let data_str = value.to_string();
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                arena.nodes[key].data = Some(data_str.clone());
            }
        }
        self
    }

    pub fn join(&mut self, tag: &str) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            // If no children, use the current selection's node as parent
            let parent = if self.keys.is_empty() {
                // Use the first key from the previous selection (the parent group)
                // This assumes the selection is a single parent node
                arena.nodes.keys().next()
            } else {
                self.keys.get(0).and_then(|k| arena.nodes[*k].parent)
            };
            if let Some(parent_key) = parent {
                // Use pending_data if present, else fallback to current node data
                let data_vec: Vec<Option<String>> = if let Some(ref pd) = self.pending_data {
                    pd.iter().map(|d| Some(d.clone())).collect()
                } else {
                    self.keys.iter().map(|k| arena.nodes[*k].data.clone()).collect()
                };
                arena.nodes[parent_key].children.clear();
                let mut new_keys = Vec::new();
                for data in data_vec {
                    let node = Node {
                        tag: tag.to_string(),
                        attributes: HashMap::new(),
                        properties: HashMap::new(),
                        data,
                        children: vec![],
                        parent: Some(parent_key),
                        text: None, // Initialize text field
                    };
                    let new_key = arena.nodes.insert(node);
                    arena.nodes[parent_key].children.push(new_key);
                    new_keys.push(new_key);
                }
                self.keys = new_keys;
                self.pending_data = None;
            }
        }
        self
    }

    /// D3-like create constructor for tests
    pub fn create(tag: &str) -> Selection {
        // For test compatibility: create a new Arena and root node
        let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
        let root = Node::new(tag);
        let root_key = arena.borrow_mut().nodes.insert(root);
        Selection { arena, keys: vec![root_key], pending_data: None }
    }

    /// D3-like nodes accessor for tests
    pub fn nodes(&self) -> Vec<Node> {
        let arena = self.arena.borrow();
        self.keys.iter().map(|k| arena.nodes[*k].clone()).collect()
    }

    /// D3-like nodes accessor for tests (returns Vec<Node> for compatibility)
    pub fn nodes_ref(&self) -> Vec<Node> {
        let arena = self.arena.borrow();
        self.keys.iter().map(|k| arena.nodes[*k].clone()).collect()
    }
    pub fn remove(&mut self) -> &mut Self {
        println!("[Selection::remove] Arena nodes before: {}", self.arena.borrow().nodes.len());
        println!("[Selection::remove] Keys to remove: {:?}", self.keys);
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                // Debug: print node tag and class before removal
                if let Some(node) = arena.nodes.get(key) {
                    let tag = &node.tag;
                    let class = node.attributes.get("class").cloned().unwrap_or_default();
                    println!("[Selection::remove] Removing node: <{} class=\\\"{}\\\">", tag, class);
                }
                // Remove from parent's children
                if let Some(parent_key) = arena.nodes[key].parent {
                    let parent = &mut arena.nodes[parent_key];
                    parent.children.retain(|&c| c != key);
                }
                // Remove all children recursively
                remove_node_recursively(&mut arena, key);
            }
        }
        println!("[Selection::remove] Arena nodes after: {}", self.arena.borrow().nodes.len());
        self.keys.clear();
        self
    }
    pub fn style(&mut self, name: &str, value: &str) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                let node = &mut arena.nodes[key];
                let mut style_attr = node.attributes.entry("style".to_string()).or_insert_with(String::new);

                let mut styles: HashMap<String, String> = if style_attr.is_empty() {
                    HashMap::new()
                } else {
                    style_attr
                        .split(';')
                        .filter(|s| !s.is_empty())
                        .filter_map(|s| {
                            let mut parts = s.splitn(2, ':');
                            match (parts.next(), parts.next()) {
                                (Some(key), Some(val)) if !key.trim().is_empty() => {
                                    Some((key.trim().to_string(), val.trim().to_string()))
                                }
                                _ => None,
                            }
                        })
                        .collect()
                };

                if value.is_empty() {
                    styles.remove(name);
                } else {
                    styles.insert(name.to_string(), value.to_string());
                }

                *style_attr = styles
                    .into_iter()
                    .map(|(k, v)| format!("{}:{}", k, v))
                    .collect::<Vec<_>>()
                    .join(";");
            }
        }
        self
    }
    pub fn property(&mut self, name: &str, value: &str) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                arena.nodes[key].properties.insert(name.to_string(), value.to_string());
            }
        }
        self
    }
    pub fn classed(&mut self, name: &str, on: bool) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                let node = &mut arena.nodes[key];
                let mut classes: HashSet<String> = node.attributes.get("class").map_or_else(HashSet::new, |c| c.split_whitespace().map(|s| s.to_string()).collect());
                if on {
                    classes.insert(name.to_string());
                } else {
                    classes.remove(name);
                }
                let new_class = classes.into_iter().collect::<Vec<_>>().join(" ");
                node.attributes.insert("class".to_string(), new_class);
            }
        }
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for key in &self.keys {
                let node = &mut arena.nodes[*key];
                node.text = Some(value.to_string());
            }
        }
        self
    }
    pub fn html(&mut self, _value: &str) -> &mut Self { self }
    pub fn insert(&mut self, _tag: &str) -> &mut Self { self }
    pub fn call<F: FnOnce(&mut Self)>(&mut self, f: F) -> &mut Self { f(self); self }
    pub fn empty(&self) -> bool { self.keys.is_empty() }
    pub fn size(&self) -> usize { self.keys.len() }
    pub fn node(&self) -> Option<Node> {
        let arena = self.arena.borrow();
        self.keys.get(0).map(|k| arena.nodes[*k].clone())
    }

    pub fn each<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut Node),
    {
        {
            let mut arena = self.arena.borrow_mut();
            for key in &self.keys {
                f(&mut arena.nodes[*key]);
            }
        }
        self
    }

    pub fn map<F, T>(&self, mut f: F) -> Vec<T>
    where
        F: FnMut(&Node) -> T,
    {
        let arena = self.arena.borrow();
        self.keys.iter().map(|k| f(&arena.nodes[*k])).collect()
    }

    pub fn filter<F>(&mut self, mut f: F) -> Selection
    where
        F: FnMut(&Node) -> bool,
    {
        let arena = self.arena.borrow();
        let filtered: Vec<NodeKey> = self.keys.iter().cloned().filter(|k| f(&arena.nodes[*k])).collect();
        Selection { arena: Rc::clone(&self.arena), keys: filtered, pending_data: None }
    }

    pub fn merge(&mut self, other: &Selection) -> Selection {
        let mut merged = self.keys.clone();
        merged.extend(other.keys.iter().cloned());
        Selection { arena: Rc::clone(&self.arena), keys: merged, pending_data: None }
    }

    pub fn children(&mut self) -> Selection {
        let mut child_keys = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            child_keys.extend(arena.nodes[key].children.iter().cloned());
        }
        Selection { arena: Rc::clone(&self.arena), keys: child_keys, pending_data: None }
    }
    pub fn select_child(&mut self, tag: &str) -> Selection {
        let mut child_keys = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            for &child_key in &arena.nodes[key].children {
                if arena.nodes[child_key].tag == tag {
                    child_keys.push(child_key);
                }
            }
        }
        Selection { arena: Rc::clone(&self.arena), keys: child_keys, pending_data: None }
    }
    pub fn parent(&mut self) -> Selection {
        let mut parent_keys = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            if let Some(parent) = arena.nodes[key].parent {
                parent_keys.push(parent);
            }
        }
        Selection { arena: Rc::clone(&self.arena), keys: parent_keys, pending_data: None }
    }
    pub fn select_parent(&mut self, tag: &str) -> Selection {
        let mut parent_keys = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            if let Some(parent) = arena.nodes[key].parent {
                if arena.nodes[parent].tag == tag {
                    parent_keys.push(parent);
                }
            }
        }
        Selection { arena: Rc::clone(&self.arena), keys: parent_keys, pending_data: None }
    }
    pub fn clone_selection(&mut self) -> Selection {
        Selection { arena: Rc::clone(&self.arena), keys: self.keys.clone(), pending_data: None }
    }
    pub fn find_all(&mut self, tag: &str) -> Selection {
        let mut found = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            for &child_key in &arena.nodes[key].children {
                if arena.nodes[child_key].tag == tag {
                    found.push(child_key);
                }
            }
        }
        Selection { arena: Rc::clone(&self.arena), keys: found, pending_data: None }
    }

    /// D3-like select: select the first child with the given tag
    pub fn select(&mut self, tag: &str) -> Selection {
        let mut found = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            if let Some(&child_key) = arena.nodes[key].children.iter().find(|&&c| arena.nodes[c].tag == tag) {
                found.push(child_key);
            }
        }
        Selection { arena: Rc::clone(&self.arena), keys: found, pending_data: None }
    }

    /// D3-like selector: supports tag, .class, tag.class, and multiple classes
    /// Now supports recursive search for matching descendants
    pub fn select_by(&mut self, selector: &str) -> Selection {
        let mut found = Vec::new();
        let (tag, classes) = parse_selector(selector);
        find_matching_descendants(Rc::clone(&self.arena), &self.keys, &tag, &classes, &mut found);
        Selection { arena: Rc::clone(&self.arena), keys: found, pending_data: None }
    }

    /// D3-like sort_by: sort nodes by a comparator
    pub fn sort_by<F>(&mut self, mut cmp: F) -> &mut Self
    where
        F: FnMut(&Node, &Node) -> std::cmp::Ordering,
    {
        {
            let arena = self.arena.borrow();
            self.keys.sort_by(|a, b| cmp(&arena.nodes[*a], &arena.nodes[*b]));
        }
        self
    }

    /// D3-like order: restore document order (no-op for now)
        pub fn order(&mut self) -> &mut Self {
        self
    }

    pub fn raise(&mut self) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                if let Some(parent_key) = arena.nodes[key].parent {
                    let parent = &mut arena.nodes[parent_key];
                    parent.children.retain(|&c| c != key);
                    parent.children.push(key);
                }
            }
        }
        self
    }

    pub fn lower(&mut self) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                if let Some(parent_key) = arena.nodes[key].parent {
                    let parent = &mut arena.nodes[parent_key];
                    parent.children.retain(|&c| c != key);
                    parent.children.insert(0, key);
                }
            }
        }
        self
    }

    pub fn render_node(arena: &Rc<RefCell<Arena>>, key: NodeKey) -> String {
        let arena_borrow = arena.borrow();
        let node = &arena_borrow.nodes[key];
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
            s.push_str(&Self::render_node(&Rc::clone(arena), child));
        }
        s.push_str(&format!("</{}>", node.tag));
        s
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
        // Recursively search children
        find_matching_descendants(Rc::clone(&arena), &node.children, tag, classes, found);
    }
}
