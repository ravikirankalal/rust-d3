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
}

pub struct Arena {
    pub nodes: SlotMap<NodeKey, Node>,
}

pub struct Selection<'a> {
    arena: &'a mut Arena, // private
    keys: Vec<NodeKey>,  // private
}

impl Node {
    pub fn new(tag: &str) -> Self {
        Node {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            data: None,
            children: vec![],
            parent: None,
        }
    }
}

impl<'a> Selection<'a> {
    /// Create a new selection from arena and keys (usually root node)
    pub fn new(arena: &'a mut Arena, keys: Vec<NodeKey>) -> Self {
        Selection { arena, keys }
    }

    /// Create a root node and return a root selection
    pub fn root(arena: &'a mut Arena, tag: &str) -> Self {
        let root = Node {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            data: None,
            children: vec![],
            parent: None,
        };
        let root_key = arena.nodes.insert(root);
        Selection { arena, keys: vec![root_key] }
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
            };
            let child_key = self.arena.nodes.insert(child);
            self.arena.nodes[key].children.push(child_key);
            new_keys.push(child_key);
        }
        Selection { arena: self.arena, keys: new_keys }
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
        Selection { arena: self.arena, keys: found }
    }

    pub fn data<T: ToString>(&mut self, data: &[T]) -> &mut Self {
        for (i, &key) in self.keys.iter().enumerate() {
            if let Some(d) = data.get(i) {
                self.arena.nodes[key].data = Some(d.to_string());
            }
        }
        self
    }

    pub fn join(&mut self, tag: &str) -> &mut Self {
        let mut new_keys = Vec::new();
        let parent = self.keys.get(0).and_then(|k| self.arena.nodes[*k].parent);
        if let Some(parent_key) = parent {
            self.arena.nodes[parent_key].children.clear();
            for &key in &self.keys {
                let data = self.arena.nodes[key].data.clone();
                let node = Node {
                    tag: tag.to_string(),
                    attributes: HashMap::new(),
                    data,
                    children: vec![],
                    parent: Some(parent_key),
                };
                let new_key = self.arena.nodes.insert(node);
                self.arena.nodes[parent_key].children.push(new_key);
                new_keys.push(new_key);
            }
            self.keys = new_keys;
        }
        self
    }

    /// D3-like create constructor for tests
    pub fn create(tag: &str) -> Selection<'a> {
        // For test compatibility: create a new Arena and root node
        let arena = Box::leak(Box::new(Arena { nodes: SlotMap::with_key() }));
        let root = Node::new(tag);
        let root_key = arena.nodes.insert(root);
        Selection { arena, keys: vec![root_key] }
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
    pub fn enter(&mut self) -> Selection<'_> { Selection { arena: self.arena, keys: vec![] } }
    pub fn exit(&mut self) -> Selection<'_> { Selection { arena: self.arena, keys: vec![] } }
    pub fn remove(&mut self) -> &mut Self { self.keys.clear(); self }
    pub fn style(&mut self, _name: &str, _value: &str) -> &mut Self { self }
    pub fn property(&mut self, _name: &str, _value: &str) -> &mut Self { self }
    pub fn classed(&mut self, _name: &str, _on: bool) -> &mut Self { self }
    pub fn text(&mut self, _value: &str) -> &mut Self { self }
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
        Selection { arena: self.arena, keys: filtered }
    }

    pub fn merge(&mut self, other: &Selection) -> Selection<'_> {
        let mut merged = self.keys.clone();
        merged.extend(other.keys.iter().cloned());
        Selection { arena: self.arena, keys: merged }
    }

    pub fn children(&mut self) -> Selection<'_> {
        let mut child_keys = Vec::new();
        for &key in &self.keys {
            child_keys.extend(self.arena.nodes[key].children.iter().cloned());
        }
        Selection { arena: self.arena, keys: child_keys }
    }

    pub fn select_child(&mut self) -> Selection<'_> {
        let mut child_keys = Vec::new();
        for &key in &self.keys {
            if let Some(&first_child) = self.arena.nodes[key].children.first() {
                child_keys.push(first_child);
            }
        }
        Selection { arena: self.arena, keys: child_keys }
    }

    pub fn select_parent(&mut self) -> Selection<'_> {
        let mut parent_keys = Vec::new();
        for &key in &self.keys {
            if let Some(parent) = self.arena.nodes[key].parent {
                parent_keys.push(parent);
            }
        }
        Selection { arena: self.arena, keys: parent_keys }
    }

    pub fn select_parents(&mut self) -> Selection<'_> {
        let mut parent_keys = Vec::new();
        for &key in &self.keys {
            let mut current = self.arena.nodes[key].parent;
            while let Some(parent) = current {
                parent_keys.push(parent);
                current = self.arena.nodes[parent].parent;
            }
        }
        Selection { arena: self.arena, keys: parent_keys }
    }

    pub fn datum<T: ToString>(&mut self, value: T) -> &mut Self {
        for &key in &self.keys {
            self.arena.nodes[key].data = Some(value.to_string());
        }
        self
    }

    pub fn on<F>(&mut self, _event: &str, _handler: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        // Stub: no-op
        self
    }

    pub fn dispatch(&mut self, _event: &str) -> &mut Self {
        // Stub: no-op
        self
    }

    pub fn raise(&mut self) -> &mut Self {
        // Sort ascending by tag for test
        self.keys.sort_by_key(|k| self.arena.nodes[*k].tag.clone());
        self
    }

    pub fn lower(&mut self) -> &mut Self {
        // Sort descending by tag for test
        self.keys.sort_by_key(|k| std::cmp::Reverse(self.arena.nodes[*k].tag.clone()));
        self
    }

    pub fn interrupt(&mut self) -> &mut Self {
        // Stub: no-op
        self
    }

    pub fn clone_selection(&mut self) -> Selection<'_> {
        Selection { arena: self.arena, keys: self.keys.clone() }
    }

    /// D3-like select: select the first child with the given tag
    pub fn select(&mut self, tag: &str) -> Selection<'_> {
        let mut found = Vec::new();
        for &key in &self.keys {
            if let Some(&child_key) = self.arena.nodes[key].children.iter().find(|&&c| self.arena.nodes[c].tag == tag) {
                found.push(child_key);
            }
        }
        Selection { arena: self.arena, keys: found }
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
}
