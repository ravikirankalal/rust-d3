use crate::selection::arena::{Arena, NodeKey};
use crate::selection::data_join::DataJoin;
use crate::selection::node::Node;
use crate::selection::utils::{parse_selector, remove_node_recursively};
use slotmap::SlotMap;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Validates if a style value is valid CSS
/// This is a basic validation that checks for common invalid patterns
fn is_valid_style_value(value: &str) -> bool {
    // Empty values are considered valid (they remove the style)
    if value.is_empty() {
        return true;
    }

    // Check for basic CSS validity patterns
    let trimmed = value.trim();

    // Check for obviously invalid values
    if trimmed.is_empty() {
        return false;
    }

    // Check for common invalid patterns
    if trimmed.contains("<<") || trimmed.contains(">>") {
        return false;
    }

    // Check for unmatched quotes
    let single_quotes = trimmed.matches('\'').count();
    let double_quotes = trimmed.matches('"').count();
    if single_quotes % 2 != 0 || double_quotes % 2 != 0 {
        return false;
    }

    // Check for some common CSS units and values
    let has_valid_pattern = trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "-_.#%()[]{}:;, ".contains(c));

    has_valid_pattern
}

pub struct Selection {
    arena: Rc<RefCell<Arena>>,
    keys: Vec<NodeKey>,
    pending_data: Option<Vec<String>>,
    enter_keys: Option<Vec<NodeKey>>,
    update_keys: Option<Vec<NodeKey>>,
    exit_keys: Option<Vec<NodeKey>>,
}

// Full impl Selection moved from mod.rs
impl Selection {
    /// Create a new selection from arena and keys (usually root node)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rust_d3::selection::{Selection, Arena, NodeKey};
    /// # use slotmap::SlotMap;
    /// # use std::rc::Rc;
    /// # use std::cell::RefCell;
    /// let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
    /// let keys = vec![];
    /// let selection = Selection::new(arena, keys);
    /// assert!(selection.is_empty());
    /// ```
    pub fn new(arena: Rc<RefCell<Arena>>, keys: Vec<NodeKey>) -> Self {
        Selection {
            arena,
            keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn root(arena: Rc<RefCell<Arena>>, tag: &str) -> Self {
        let root = Node {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            properties: HashMap::new(),
            data: None,
            children: vec![],
            parent: None,
            text: None,
            event_handlers: HashMap::new(),
        };
        let root_key = arena.borrow_mut().nodes.insert(root);
        Selection {
            arena,
            keys: vec![root_key],
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = &NodeKey> {
        self.keys.iter()
    }
    
    /// Get an attribute value from the first node in the selection
    pub fn get_attr(&self, name: &str) -> Option<String> {
        if let Some(key) = self.keys.first() {
            let arena = self.arena.borrow();
            arena.nodes[*key].attributes.get(name).cloned()
        } else {
            None
        }
    }
    /// Append a new child element to each node in the selection
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rust_d3::selection::Selection;
    /// let mut svg = Selection::create("svg");
    /// let mut group = svg.append("g");
    /// group.attr("class", "chart-group");
    ///
    /// let svg_node = svg.node().unwrap();
    /// assert_eq!(svg_node.children.len(), 1);
    ///
    /// let group_node = group.node().unwrap();
    /// assert_eq!(group_node.tag, "g");
    /// assert_eq!(group_node.attributes["class"], "chart-group");
    /// ```
    pub fn append(&mut self, tag: &str) -> Selection {
        let mut new_keys = Vec::new();
        
        for &key in &self.keys {
            let (parent_key, data) = {
                let arena = self.arena.borrow();
                let node = &arena.nodes[key];
                // If the node has a parent and is a placeholder (empty tag), use the parent
                // Otherwise, use the node itself as the parent
                if node.tag.is_empty() && node.parent.is_some() {
                    (node.parent.unwrap(), node.data.clone())
                } else {
                    (key, node.data.clone())
                }
            };
            
            let child = Node {
                tag: tag.to_string(),
                attributes: HashMap::new(),
                properties: HashMap::new(),
                data: data.clone(),
                children: vec![],
                parent: Some(parent_key),
                text: None,
                event_handlers: HashMap::new(),
            };
            let child_key = self.arena.borrow_mut().nodes.insert(child);
            self.arena.borrow_mut().nodes[parent_key].children.push(child_key);
            new_keys.push(child_key);
        }
        
        Selection {
            arena: Rc::clone(&self.arena),
            keys: new_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                if value.is_empty() {
                    arena.nodes[key].attributes.remove(name);
                } else {
                    arena.nodes[key]
                        .attributes
                        .insert(name.to_string(), value.to_string());
                }
            }
        }
        self
    }
    pub fn attr_fn<F>(&mut self, name: &str, mut f: F) -> &mut Self
    where
        F: FnMut(&Node, usize, Option<String>) -> String,
    {
        {
            let mut arena = self.arena.borrow_mut();
            for (i, &key) in self.keys.iter().enumerate() {
                let previous_value = arena.nodes[key].attributes.get(name).cloned();
                let node = &arena.nodes[key];
                let value = f(node, i, previous_value);
                if value.is_empty() {
                    arena.nodes[key].attributes.remove(name);
                } else {
                    arena.nodes[key].attributes.insert(name.to_string(), value.clone());
                }
            }
        }
        self
    }
    pub fn select_all(&mut self, tag: Option<&str>) -> Selection {
        let mut found = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            self.traverse_children_ordered(&arena, key, &mut found, tag);
        }
        Selection {
            arena: Rc::clone(&self.arena),
            keys: found,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    
    fn traverse_children_ordered(
        &self,
        arena: &Arena,
        key: NodeKey,
        found: &mut Vec<NodeKey>,
        tag: Option<&str>,
    ) {
        if let Some(node) = arena.nodes.get(key) {
            // Process children in order
            for &child_key in &node.children {
                if let Some(child_node) = arena.nodes.get(child_key) {
                    if tag.map_or(true, |t| child_node.tag == t) {
                        found.push(child_key);
                    }
                }
                // Recursively traverse children
                self.traverse_children_ordered(arena, child_key, found, tag);
            }
        }
    }
    pub fn data<T: ToString>(&mut self, data: &[T]) -> DataJoin {
        let mut enter_keys = Vec::new();
        let mut update_keys = Vec::new();
        let mut exit_keys = Vec::new();
        let mut data_iter = data.iter().map(|d| d.to_string()).peekable();
        let mut node_iter = self.keys.iter().peekable();
        let mut arena = self.arena.borrow_mut();
        let mut _i = 0;
        
        // Find the parent node for enter selections
        let parent_key = if let Some(&first_key) = self.keys.first() {
            arena.nodes.get(first_key).and_then(|node| node.parent)
        } else {
            None
        };
        
        while let Some(d) = data_iter.next() {
            if let Some(&node_key) = node_iter.peek() {
                arena.nodes[*node_key].data = Some(d);
                update_keys.push(*node_key);
                node_iter.next();
            } else {
                let placeholder_node = Node {
                    tag: "".to_string(),
                    attributes: HashMap::new(),
                    properties: HashMap::new(),
                    data: Some(d),
                    children: vec![],
                    parent: parent_key,
                    text: None,
                    event_handlers: HashMap::new(),
                };
                let placeholder_key = arena.nodes.insert(placeholder_node);
                enter_keys.push(placeholder_key);
            }
            _i += 1;
        }
        while let Some(&node_key) = node_iter.next() {
            exit_keys.push(node_key);
        }
        
        self.enter_keys = Some(enter_keys.clone());
        self.update_keys = Some(update_keys.clone());
        self.exit_keys = Some(exit_keys.clone());
        let update_selection = Selection {
            arena: Rc::clone(&self.arena),
            keys: update_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        };
        let enter_selection = Selection {
            arena: Rc::clone(&self.arena),
            keys: enter_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        };
        let exit_selection = Selection {
            arena: Rc::clone(&self.arena),
            keys: exit_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        };
        DataJoin {
            update: update_selection,
            enter: enter_selection,
            exit: exit_selection,
        }
    }

    /// Data join with optional key function for matching data to nodes
    pub fn data_with_key<T: ToString, K: ToString, F>(&mut self, data: &[T], key_fn: F) -> DataJoin
    where
        F: Fn(&T, usize) -> K,
    {
        let mut enter_keys = Vec::new();
        let mut update_keys = Vec::new();
        let mut exit_keys = Vec::new();

        let mut arena = self.arena.borrow_mut();

        // Find the parent node for enter selections
        let parent_key = self.keys.first().copied();

        // Create data items with keys and indices
        let data_items: Vec<_> = data
            .iter()
            .enumerate()
            .map(|(i, d)| (key_fn(d, i).to_string(), d.to_string(), i))
            .collect();

        // Create existing nodes map with keys
        // Since the node data is already a string, we need to use a different approach
        // We'll create a map where the key is the node's current data, and we'll match it against
        // the data items using the key function
        let mut existing_nodes: HashMap<String, (NodeKey, usize)> = HashMap::new();
        for (i, &node_key) in self.keys.iter().enumerate() {
            if let Some(node) = arena.nodes.get(node_key) {
                if let Some(ref data) = node.data {
                    // We need to find which data item this node corresponds to
                    // We'll use the node's data as the key directly for now
                    // This assumes that the key function would produce the same key for the same data
                    existing_nodes.insert(data.clone(), (node_key, i));
                }
            }
        }

        // Process data items to create enter/update selections
        for (data_key, data_value, _data_index) in data_items.into_iter() {
            if let Some((node_key, _)) = existing_nodes.remove(&data_key) {
                // Update existing node
                arena.nodes[node_key].data = Some(data_value.clone());
                update_keys.push(node_key);
            } else {
                // Place new data in enter selection
                let enter_node = Node {
                    tag: "".to_string(),
                    attributes: HashMap::new(),
                    properties: HashMap::new(),
                    data: Some(data_value.clone()),
                    children: vec![],
                    parent: parent_key,
                    text: None,
                    event_handlers: HashMap::new(),
                };
                let enter_key = arena.nodes.insert(enter_node);
                enter_keys.push(enter_key);
            }
        }

        // Remaining nodes go to exit selection
        exit_keys.extend(existing_nodes.into_values().map(|(key, _)| key));

        self.enter_keys = Some(enter_keys.clone());
        self.update_keys = Some(update_keys.clone());
        self.exit_keys = Some(exit_keys.clone());

        let update_selection = Selection {
            arena: Rc::clone(&self.arena),
            keys: update_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        };
        let enter_selection = Selection {
            arena: Rc::clone(&self.arena),
            keys: enter_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        };
        let exit_selection = Selection {
            arena: Rc::clone(&self.arena),
            keys: exit_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        };

        DataJoin {
            update: update_selection,
            enter: enter_selection,
            exit: exit_selection,
        }
    }
    pub fn datum<T: ToString>(&mut self, value: T) -> &mut Self {
        let data_str = value.to_string();
        {
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

            // For enter selection, we need to find the parent where new elements should be added
            // Look for the first real node that could be a parent
            let mut parent_key = None;

            // Search for the most recently created group/container element
            let mut found_keys = Vec::new();
            for (key, node) in arena.nodes.iter() {
                if !node.tag.is_empty()
                    && (node.tag == "g" || node.tag == "svg" || node.tag == "div")
                {
                    found_keys.push(key);
                }
            }
            // Get the last (most recent) one
            if let Some(&key) = found_keys.last() {
                parent_key = Some(key);
            }

            if let Some(parent_key) = parent_key {
                let data_vec: Vec<Option<String>> = if let Some(ref pd) = self.pending_data {
                    pd.iter().map(|d| Some(d.clone())).collect()
                } else {
                    self.keys
                        .iter()
                        .map(|k| arena.nodes.get(*k).and_then(|n| n.data.clone()))
                        .collect()
                };

                let mut new_keys = Vec::new();
                for data in data_vec {
                    let node = Node {
                        tag: tag.to_string(),
                        attributes: HashMap::new(),
                        properties: HashMap::new(),
                        data,
                        children: vec![],
                        parent: Some(parent_key),
                        text: None,
                        event_handlers: HashMap::new(),
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
    /// Create a new selection with a single root node
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rust_d3::selection::Selection;
    /// let mut svg = Selection::create("svg");
    /// svg.attr("width", "400").attr("height", "300");
    /// let node = svg.node().unwrap();
    /// assert_eq!(node.tag, "svg");
    /// assert_eq!(node.attributes["width"], "400");
    /// ```
    pub fn create(tag: &str) -> Selection {
        let arena = Rc::new(RefCell::new(Arena {
            nodes: SlotMap::with_key(),
        }));
        let root = Node::new(tag);
        let root_key = arena.borrow_mut().nodes.insert(root);
        Selection {
            arena,
            keys: vec![root_key],
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn nodes(&self) -> Vec<Node> {
        let arena = self.arena.borrow();
        self.keys.iter().map(|k| arena.nodes[*k].clone()).collect()
    }
    pub fn nodes_ref(&self) -> Vec<Node> {
        let arena = self.arena.borrow();
        self.keys.iter().map(|k| arena.nodes[*k].clone()).collect()
    }
    pub fn remove(&mut self) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                // Always update parent children list, but only if parent is still valid
                if let Some(parent_key) = arena.nodes.get(key).and_then(|n| n.parent) {
                    if let Some(parent) = arena.nodes.get_mut(parent_key) {
                        parent.children.retain(|&c| c != key);
                        // Extra check: if key is still present, forcibly remove
                        if parent.children.contains(&key) {
                            parent.children.retain(|&c| c != key);
                        }
                    }
                }
                remove_node_recursively(&mut arena, key);
            }
            self.keys.clear();
        }
        self
    }
    pub fn style(&mut self, name: &str, value: &str) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                let node = &mut arena.nodes[key];
                let style_attr = node
                    .attributes
                    .entry("style".to_string())
                    .or_insert_with(String::new);
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
                    #[cfg(debug_assertions)]
                    {
                        if !is_valid_style_value(&value) {
                            println!(
                                "[WARN] Invalid style value for '{}': '{}'. Value dropped.",
                                name, value
                            );
                        } else {
                            styles.insert(name.to_string(), value.to_string());
                        }
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        styles.insert(name.to_string(), value.to_string());
                    }
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
    /// Set style using a function (D3 .style with function)
    pub fn style_fn<F>(&mut self, name: &str, mut f: F) -> &mut Self
    where
        F: FnMut(&Node, usize, Option<String>) -> String,
    {
        {
            let mut arena = self.arena.borrow_mut();
            for (i, &key) in self.keys.iter().enumerate() {
                // Clone the node before getting mutable reference to avoid borrow checker issues
                let node_clone = arena.nodes[key].clone();

                let node = &mut arena.nodes[key];
                let style_attr = node
                    .attributes
                    .entry("style".to_string())
                    .or_insert_with(String::new);
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
                let previous_value = styles.get(name).cloned();
                let value = f(&node_clone, i, previous_value);
                if value.is_empty() {
                    styles.remove(name);
                } else {
                    #[cfg(debug_assertions)]
                    {
                        if !is_valid_style_value(&value) {
                            println!(
                                "[WARN] Invalid style value for '{}': '{}'. Value dropped.",
                                name, value
                            );
                        } else {
                            styles.insert(name.to_string(), value);
                        }
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        styles.insert(name.to_string(), value);
                    }
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
                if value.is_empty() {
                    arena.nodes[key].properties.remove(name);
                } else {
                    arena.nodes[key]
                        .properties
                        .insert(name.to_string(), value.to_string());
                }
            }
        }
        self
    }
    pub fn classed(&mut self, name: &str, on: bool) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                let node = &mut arena.nodes[key];
                let mut classes: HashSet<String> =
                    node.attributes.get("class").map_or_else(HashSet::new, |c| {
                        c.split_whitespace().map(|s| s.to_string()).collect()
                    });
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
    /// Set inner HTML (for SVG/text, sets the text field)
    pub fn html(&mut self, value: &str) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                arena.nodes[key].text = Some(value.to_string());
            }
        }
        self
    }

    /// Insert a new child node before a reference node (by tag or index)
    pub fn insert(&mut self, tag: &str, before: Option<&str>) -> &mut Self {
        {
            let mut arena = self.arena.borrow_mut();
            for &parent_key in &self.keys {
                let new_node = Node {
                    tag: tag.to_string(),
                    attributes: HashMap::new(),
                    properties: HashMap::new(),
                    data: None,
                    children: vec![],
                    parent: Some(parent_key),
                    text: None,
                    event_handlers: HashMap::new(),
                };
                let new_key = arena.nodes.insert(new_node);
                let pos = if let Some(before_tag) = before {
                    // Find position before mutable borrow
                    let children = &arena.nodes[parent_key].children;
                    children
                        .iter()
                        .position(|&c| arena.nodes[c].tag == before_tag)
                } else {
                    None
                };
                let parent = &mut arena.nodes[parent_key];
                if let Some(pos) = pos {
                    parent.children.insert(pos, new_key);
                } else {
                    parent.children.push(new_key);
                }
            }
        }
        self
    }

    /// Attach an event handler (stores handler in node, for API completeness)
    pub fn on<F>(&mut self, event: &str, handler: F) -> &mut Self
    where
        F: FnMut(&mut Node) + Clone + 'static,
    {
        {
            let mut arena = self.arena.borrow_mut();
            for &key in &self.keys {
                let node = &mut arena.nodes[key];
                node.event_handlers
                    .entry(event.to_string())
                    .or_insert_with(Vec::new)
                    .push(Box::new(handler.clone()));
            }
        }
        self
    }
    pub fn call<F: FnOnce(&mut Self)>(&mut self, f: F) -> &mut Self {
        f(self);
        self
    }
    pub fn empty(&self) -> bool {
        self.keys.is_empty()
    }
    pub fn size(&self) -> usize {
        self.keys.len()
    }
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
        let filtered: Vec<NodeKey> = self
            .keys
            .iter()
            .cloned()
            .filter(|k| f(&arena.nodes[*k]))
            .collect();
        Selection {
            arena: Rc::clone(&self.arena),
            keys: filtered,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn merge(&mut self, other: &Selection) -> Selection {
        let mut merged = self.keys.clone();
        merged.extend(other.keys.iter().cloned());
        Selection {
            arena: Rc::clone(&self.arena),
            keys: merged,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn children(&mut self) -> Selection {
        let mut child_keys = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            child_keys.extend(arena.nodes[key].children.iter().cloned());
        }
        Selection {
            arena: Rc::clone(&self.arena),
            keys: child_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
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
        Selection {
            arena: Rc::clone(&self.arena),
            keys: child_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn parent(&mut self) -> Selection {
        let mut parent_keys = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            if let Some(parent) = arena.nodes[key].parent {
                parent_keys.push(parent);
            }
        }
        Selection {
            arena: Rc::clone(&self.arena),
            keys: parent_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
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
        Selection {
            arena: Rc::clone(&self.arena),
            keys: parent_keys,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn clone_selection(&mut self) -> Selection {
        Selection {
            arena: Rc::clone(&self.arena),
            keys: self.keys.clone(),
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
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
        Selection {
            arena: Rc::clone(&self.arena),
            keys: found,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn select(&mut self, tag: &str) -> Selection {
        let mut found = Vec::new();
        let arena = self.arena.borrow();
        for &key in &self.keys {
            if let Some(&child_key) = arena.nodes[key]
                .children
                .iter()
                .find(|&&c| arena.nodes[c].tag == tag)
            {
                found.push(child_key);
            }
        }
        Selection {
            arena: Rc::clone(&self.arena),
            keys: found,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn select_by(&mut self, selector: &str) -> Selection {
        let mut found = Vec::new();
        let parsed_selector = parse_selector(selector);
        let arena = self.arena.borrow();
        for &root_key in &self.keys {
            let mut stack = vec![root_key];
            while let Some(key) = stack.pop() {
                if let Some(node) = arena.nodes.get(key) {
                    let node_id = node.attributes.get("id").map(|s| s.as_str());
                    let node_classes: Vec<String> =
                        node.attributes.get("class").map_or_else(Vec::new, |cls| {
                            cls.split_whitespace().map(|s| s.to_string()).collect()
                        });

                    if parsed_selector.matches(&node.tag, node_id, &node_classes) {
                        found.push(key);
                    }
                    stack.extend(&node.children);
                }
            }
        }
        Selection {
            arena: Rc::clone(&self.arena),
            keys: found,
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    pub fn sort_by<F>(&mut self, mut cmp: F) -> &mut Self
    where
        F: FnMut(&Node, &Node) -> std::cmp::Ordering,
    {
        {
            let arena = self.arena.borrow();
            self.keys
                .sort_by(|a, b| cmp(&arena.nodes[*a], &arena.nodes[*b]));
        }
        self
    }
    pub fn order(&mut self) -> &mut Self {
        // In D3, order() reorders the DOM elements to match the selection order
        // We need to update parent.children to reflect the current selection order
        {
            let mut arena = self.arena.borrow_mut();
            // Group keys by their parent
            let mut parent_groups: HashMap<Option<NodeKey>, Vec<NodeKey>> = HashMap::new();

            for &key in &self.keys {
                if let Some(node) = arena.nodes.get(key) {
                    let parent_key = node.parent;
                    parent_groups
                        .entry(parent_key)
                        .or_insert_with(Vec::new)
                        .push(key);
                }
            }

            // For each parent, reorder its children to match the selection order
            for (parent_key, selected_keys) in parent_groups {
                if let Some(parent_key) = parent_key {
                    if let Some(parent) = arena.nodes.get_mut(parent_key) {
                        // Create a new children vector with selected keys in selection order
                        let mut new_children = Vec::new();
                        let mut remaining_children = parent.children.clone();

                        // First, add the selected keys in their selection order
                        for &selected_key in &selected_keys {
                            if let Some(pos) =
                                remaining_children.iter().position(|&c| c == selected_key)
                            {
                                remaining_children.remove(pos);
                                new_children.push(selected_key);
                            }
                        }

                        // Then, add any remaining children that weren't in the selection
                        new_children.extend(remaining_children);

                        // Update the parent's children
                        parent.children = new_children;
                    }
                }
            }
        }
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
    /// Returns the update selection from the last data join
    pub fn update(&self) -> Selection {
        Selection {
            arena: Rc::clone(&self.arena),
            keys: self.update_keys.clone().unwrap_or_default(),
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    /// Returns the enter selection from the last data join
    pub fn enter(&self) -> Selection {
        Selection {
            arena: Rc::clone(&self.arena),
            keys: self.enter_keys.clone().unwrap_or_default(),
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    /// Returns the exit selection from the last data join
    pub fn exit(&self) -> Selection {
        Selection {
            arena: Rc::clone(&self.arena),
            keys: self.exit_keys.clone().unwrap_or_default(),
            pending_data: None,
            enter_keys: None,
            update_keys: None,
            exit_keys: None,
        }
    }
    /// Clone selection and nodes (D3.js parity)
    /// Creates a clone of this selection that inserts cloned nodes after the originals
    pub fn clone(&self) -> Selection {
        self.clone_with_deep(true)
    }

    /// Clone selection only (shallow clone - just keys, not nodes)
    pub fn clone_shallow(&self) -> Selection {
        Selection {
            arena: Rc::clone(&self.arena),
            keys: self.keys.clone(),
            pending_data: self.pending_data.clone(),
            enter_keys: self.enter_keys.clone(),
            update_keys: self.update_keys.clone(),
            exit_keys: self.exit_keys.clone(),
        }
    }

    /// Internal method for cloning with optional deep cloning
    fn clone_with_deep(&self, deep: bool) -> Selection {
        if !deep {
            return self.clone_shallow();
        }

        let mut arena = self.arena.borrow_mut();
        let mut new_keys = Vec::new();

        // First pass: clone all nodes and collect their info
        let mut clone_info = Vec::new();
        for &key in &self.keys {
            if let Some(node) = arena.nodes.get(key) {
                let cloned_node = node.clone();
                let parent_key = node.parent;
                clone_info.push((key, cloned_node, parent_key));
            }
        }

        // Second pass: insert cloned nodes and update parent references
        for (_original_key, cloned_node, parent_key) in clone_info {
            let cloned_key = arena.nodes.insert(cloned_node);
            new_keys.push(cloned_key);

            // Update the cloned node's parent reference
            if let Some(parent_key) = parent_key {
                if let Some(cloned_node_mut) = arena.nodes.get_mut(cloned_key) {
                    cloned_node_mut.parent = Some(parent_key);
                }
            }
        }

        // Third pass: insert cloned nodes into their parent's children lists
        for (i, &original_key) in self.keys.iter().enumerate() {
            if let Some(node) = arena.nodes.get(original_key) {
                if let Some(parent_key) = node.parent {
                    if let Some(parent) = arena.nodes.get_mut(parent_key) {
                        // Find the position of the original node
                        if let Some(pos) = parent.children.iter().position(|&c| c == original_key) {
                            // Insert the cloned node right after the original
                            parent.children.insert(pos + 1, new_keys[i]);
                        } else {
                            // Fallback: append to the end
                            parent.children.push(new_keys[i]);
                        }
                    }
                }
            }
        }

        drop(arena);

        Selection {
            arena: Rc::clone(&self.arena),
            keys: new_keys,
            pending_data: self.pending_data.clone(),
            enter_keys: self.enter_keys.clone(),
            update_keys: self.update_keys.clone(),
            exit_keys: self.exit_keys.clone(),
        }
    }
    /// Deep clone: clones all nodes and structure
    pub fn deep_clone(&self) -> Selection {
        let arena = self.arena.borrow();
        let new_arena = Rc::new(RefCell::new(Arena {
            nodes: SlotMap::with_key(),
        }));
        let mut key_map = HashMap::new();
        for &key in &self.keys {
            Self::clone_node_recursive(
                &arena,
                &mut new_arena.borrow_mut(),
                key,
                None,
                &mut key_map,
            );
        }
        let new_keys = self.keys.iter().map(|k| key_map[k]).collect();
        Selection {
            arena: new_arena,
            keys: new_keys,
            pending_data: self.pending_data.clone(),
            enter_keys: self.enter_keys.clone(),
            update_keys: self.update_keys.clone(),
            exit_keys: self.exit_keys.clone(),
        }
    }
    fn clone_node_recursive(
        arena: &Arena,
        new_arena: &mut Arena,
        key: NodeKey,
        parent: Option<NodeKey>,
        key_map: &mut HashMap<NodeKey, NodeKey>,
    ) {
        let node = &arena.nodes[key];
        let mut new_node = node.clone();
        new_node.parent = parent;
        new_node.children = vec![];
        let new_key = new_arena.nodes.insert(new_node);
        key_map.insert(key, new_key);
        for &child in &node.children {
            Self::clone_node_recursive(arena, new_arena, child, Some(new_key), key_map);
            new_arena.nodes[new_key].children.push(key_map[&child]);
        }
    }

    /// Find first matching descendant by tag or class (D3 .find)
    pub fn find(&self, selector: &str) -> Option<Node> {
        let parsed_selector = parse_selector(selector);
        let arena = self.arena.borrow();
        for &key in &self.keys {
            let mut stack = vec![key];
            while let Some(k) = stack.pop() {
                let node = &arena.nodes[k];
                let node_id = node.attributes.get("id").map(|s| s.as_str());
                let node_classes: Vec<String> =
                    node.attributes.get("class").map_or_else(Vec::new, |cls| {
                        cls.split_whitespace().map(|s| s.to_string()).collect()
                    });

                if parsed_selector.matches(&node.tag, node_id, &node_classes) {
                    return Some(node.clone());
                }
                stack.extend(&node.children);
            }
        }
        None
    }

    /// Documentation for all selection methods (D3 parity)
    // select, select_all, filter, data, datum, append, insert, remove, attr, style, property, classed, text, html, on, each, call, merge, order, raise, lower, node, nodes, size, empty, parent, children, clone, deep_clone, find, find_all, select_by, sort_by, map, select_child, select_parent, transition, interrupt, dispatch, enter, update, exit, join
    pub fn render_node(arena: &Rc<RefCell<Arena>>, key: NodeKey) -> String {
        let arena_borrow = arena.borrow();
        if !arena_borrow.nodes.contains_key(key) {
            return String::new(); // Node was removed, skip rendering
        }
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
        if let Some(ref text) = node.text {
            s.push_str(text);
        }
        for &child in &node.children {
            s.push_str(&Self::render_node(&Rc::clone(arena), child));
        }
        s.push_str(&format!("</{}>", node.tag));
        s
    }

    /// Debug helper: print tag and class of each child node in the selection
    pub fn debug_print_children<F>(&self, label: &str, filter_fn: F)
    where
        F: Fn(&Node) -> bool,
    {
        let arena_ref = self.arena.borrow();
        println!("[DEBUG] {} children count: {}", label, self.keys.len());
        for (i, key) in self.keys.iter().enumerate() {
            if let Some(node) = arena_ref.nodes.get(*key) {
                if filter_fn(node) {
                    let class = node.attributes.get("class");
                    println!("[DEBUG] Child {}: tag={}, class={:?}", i, node.tag, class);
                }
            }
        }
    }

    /// Debug helper: print all children (convenience method)
    pub fn debug_print_all_children(&self, label: &str) {
        self.debug_print_children(label, |_| true);
    }
    
    /// Render the first node in the selection as an HTML string
    pub fn render(&self) -> String {
        if let Some(&key) = self.keys.first() {
            Self::render_node(&self.arena, key)
        } else {
            String::new()
        }
    }
}
