//! d3-selection parity module (improved parity)
//! 
//! This module ports the D3.js d3-selection API to Rust. 
//! It provides a Selection struct and methods for DOM-like selection, data binding, and manipulation.
//!
//! # Parity Goals
//! - select, select_all
//! - attr, style
//! - data, enter, exit
//! - append, remove
//! - event handling (on, dispatch)
//! - Integration with other d3 modules

use std::collections::HashMap;

pub struct Node {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub styles: HashMap<String, String>,
    pub data: Option<String>, // Each node can have one datum
    pub children: Vec<Node>,
    pub event_handlers: HashMap<String, Vec<Box<dyn Fn() + Send + Sync>>>,
}

impl Node {
    pub fn new(tag: &str) -> Self {
        Node {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            styles: HashMap::new(),
            data: None,
            children: vec![],
            event_handlers: HashMap::new(),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            tag: self.tag.clone(),
            attributes: self.attributes.clone(),
            styles: self.styles.clone(),
            data: self.data.clone(),
            children: self.children.clone(),
            event_handlers: HashMap::new(),
        }
    }
}

pub struct Selection {
    pub nodes: Vec<Node>,
    pub enter_nodes: Vec<Node>,
    pub exit_nodes: Vec<Node>,
}

impl Selection {
    /// Select a single node by tag (stub: just creates a new node)
    pub fn select(selector: &str) -> Self {
        Selection {
            nodes: vec![Node::new(selector)],
            enter_nodes: vec![],
            exit_nodes: vec![],
        }
    }
    /// Select multiple nodes by tag (stub: creates 3 nodes)
    pub fn select_all(selector: &str) -> Self {
        Selection {
            nodes: vec![Node::new(selector), Node::new(selector), Node::new(selector)],
            enter_nodes: vec![],
            exit_nodes: vec![],
        }
    }
    /// Set an attribute on all nodes
    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        for node in &mut self.nodes {
            node.attributes.insert(name.to_string(), value.to_string());
        }
        self
    }
    /// Set a style on all nodes
    pub fn style(&mut self, name: &str, value: &str) -> &mut Self {
        for node in &mut self.nodes {
            node.styles.insert(name.to_string(), value.to_string());
        }
        self
    }
    /// Bind data to nodes, creating enter/exit selections
    pub fn data<T: ToString + Clone>(&mut self, data: &[T]) -> &mut Self {
        let mut new_nodes = vec![];
        let mut enter_nodes = vec![];
        let mut exit_nodes = vec![];
        let min_len = self.nodes.len().min(data.len());
        // Update existing nodes
        for (i, node) in self.nodes.iter_mut().enumerate().take(min_len) {
            node.data = Some(data[i].to_string());
            new_nodes.push(node.clone());
        }
        // Enter: new data
        for d in data.iter().skip(min_len) {
            let mut n = Node::new(&self.nodes.get(0).map(|n| n.tag.as_str()).unwrap_or("g"));
            n.data = Some(d.to_string());
            enter_nodes.push(n.clone());
            new_nodes.push(n);
        }
        // Exit: extra nodes
        for node in self.nodes.iter().skip(data.len()) {
            exit_nodes.push(node.clone());
        }
        self.nodes = new_nodes;
        self.enter_nodes = enter_nodes;
        self.exit_nodes = exit_nodes;
        self
    }
    /// Get the enter selection (nodes created by data join)
    pub fn enter(&self) -> Self {
        Selection {
            nodes: self.enter_nodes.clone(),
            enter_nodes: vec![],
            exit_nodes: vec![],
        }
    }
    /// Get the exit selection (nodes removed by data join)
    pub fn exit(&self) -> Self {
        Selection {
            nodes: self.exit_nodes.clone(),
            enter_nodes: vec![],
            exit_nodes: vec![],
        }
    }
    /// Append a child node to all nodes
    pub fn append(&mut self, element: &str) -> &mut Self {
        for node in &mut self.nodes {
            node.children.push(Node::new(element));
        }
        self
    }
    /// Remove all nodes from the selection
    pub fn remove(&mut self) -> &mut Self {
        self.nodes.clear();
        self
    }
    /// Attach an event handler to all nodes
    pub fn on<F>(&mut self, event: &str, handler: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static + Clone,
    {
        for node in &mut self.nodes {
            node.event_handlers.entry(event.to_string()).or_default().push(Box::new(handler.clone()));
        }
        self
    }
    /// Dispatch an event to all nodes
    pub fn dispatch(&mut self, event: &str) -> &mut Self {
        for node in &mut self.nodes {
            if let Some(handlers) = node.event_handlers.get(event) {
                for handler in handlers {
                    handler();
                }
            }
        }
        self
    }
    /// Get children of all nodes as a new selection
    pub fn children(&self) -> Self {
        let mut all_children = vec![];
        for node in &self.nodes {
            all_children.extend(node.children.clone());
        }
        Selection {
            nodes: all_children,
            enter_nodes: vec![],
            exit_nodes: vec![],
        }
    }
    /// Filter nodes by a predicate (like selection.filter in D3)
    pub fn filter<F>(&self, predicate: F) -> Self
    where
        F: Fn(&Node) -> bool,
    {
        let filtered = self.nodes.iter().cloned().filter(|n| predicate(n)).collect();
        Selection {
            nodes: filtered,
            enter_nodes: vec![],
            exit_nodes: vec![],
        }
    }
    /// Merge another selection into this one (like selection.merge in D3)
    pub fn merge(&self, other: &Selection) -> Self {
        let mut merged = self.nodes.clone();
        merged.extend(other.nodes.clone());
        Selection {
            nodes: merged,
            enter_nodes: vec![],
            exit_nodes: vec![],
        }
    }
    /// Each: apply a function to each node (like selection.each in D3)
    pub fn each<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut Node),
    {
        for node in &mut self.nodes {
            f(node);
        }
        self
    }
    /// Map: transform each node into a value (like selection.map in D3)
    pub fn map<F, T>(&self, mut f: F) -> Vec<T>
    where
        F: FnMut(&Node) -> T,
    {
        self.nodes.iter().map(|n| f(n)).collect()
    }
    /// Set or get a property on all nodes (simulated, like attr)
    pub fn property(&mut self, name: &str, value: &str) -> &mut Self {
        for node in &mut self.nodes {
            node.attributes.insert(format!("property:{}", name), value.to_string());
        }
        self
    }
    /// Add/remove/toggle a class (simulated, stores in attributes)
    pub fn classed(&mut self, class: &str, value: bool) -> &mut Self {
        for node in &mut self.nodes {
            let entry = node.attributes.entry("class".to_string()).or_default();
            let mut classes: Vec<&str> = entry.split_whitespace().collect();
            if value {
                if !classes.contains(&class) {
                    classes.push(class);
                }
            } else {
                classes.retain(|&c| c != class);
            }
            *entry = classes.join(" ");
        }
        self
    }
    /// Set or get text content (simulated, stores in attributes)
    pub fn text(&mut self, value: &str) -> &mut Self {
        for node in &mut self.nodes {
            node.attributes.insert("textContent".to_string(), value.to_string());
        }
        self
    }
    /// Set or get HTML content (simulated, stores in attributes)
    pub fn html(&mut self, value: &str) -> &mut Self {
        for node in &mut self.nodes {
            node.attributes.insert("innerHTML".to_string(), value.to_string());
        }
        self
    }
    /// Set or get datum directly (like D3's datum)
    pub fn datum(&mut self, value: &str) -> &mut Self {
        for node in &mut self.nodes {
            node.data = Some(value.to_string());
        }
        self
    }
    /// Insert a new child node before the first child (simulated)
    pub fn insert(&mut self, tag: &str) -> &mut Self {
        for node in &mut self.nodes {
            node.children.insert(0, Node::new(tag));
        }
        self
    }
    /// Call a function with this selection (like selection.call in D3)
    pub fn call<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        f(self);
        self
    }
    /// Returns true if the selection is empty
    pub fn empty(&self) -> bool {
        self.nodes.is_empty()
    }
    /// Returns the first node (if any)
    pub fn node(&self) -> Option<&Node> {
        self.nodes.get(0)
    }
    /// Returns the number of nodes in the selection
    pub fn size(&self) -> usize {
        self.nodes.len()
    }
    /// Returns all nodes as a Vec
    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
    /// Select the first child of each node
    pub fn select_child(&self) -> Self {
        let children = self.nodes.iter().filter_map(|n| n.children.get(0).cloned()).collect();
        Selection { nodes: children, enter_nodes: vec![], exit_nodes: vec![] }
    }
    /// Select all children of all nodes
    pub fn select_children(&self) -> Self {
        let mut all = vec![];
        for n in &self.nodes {
            all.extend(n.children.clone());
        }
        Selection { nodes: all, enter_nodes: vec![], exit_nodes: vec![] }
    }
    /// Select the parent of each node (not tracked, stub returns empty)
    pub fn select_parent(&self) -> Self {
        Selection { nodes: vec![], enter_nodes: vec![], exit_nodes: vec![] }
    }
    /// Select all parents of all nodes (not tracked, stub returns empty)
    pub fn select_parents(&self) -> Self {
        Selection { nodes: vec![], enter_nodes: vec![], exit_nodes: vec![] }
    }
}

impl Clone for Selection {
    fn clone(&self) -> Self {
        Selection {
            nodes: self.nodes.clone(),
            enter_nodes: self.enter_nodes.clone(),
            exit_nodes: self.exit_nodes.clone(),
        }
    }
}
