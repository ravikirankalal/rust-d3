// Selection implementation

pub struct Selection<T> {
    data: Vec<T>,
}

impl<T> Selection<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn map<U, F>(&self, mut f: F) -> Selection<U>
    where
        F: FnMut(&T) -> U,
    {
        Selection {
            data: self.data.iter().map(|d| f(d)).collect(),
        }
    }

    pub fn filter<F>(&self, mut f: F) -> Selection<T>
    where
        F: FnMut(&T) -> bool,
        T: Clone,
    {
        Selection {
            data: self.data.iter().cloned().filter(|d| f(d)).collect(),
        }
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        for d in &self.data {
            f(d);
        }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
    /// D3.js data join API: replaces the current data with new data, returns a new selection.
    pub fn set_data<U: Clone>(&self, new_data: Vec<U>) -> Selection<U> {
        // Implements a simple data join: returns new selection with new data
        Selection::new(new_data)
    }

    /// D3.js enter selection: returns items in new_data not in current data
    pub fn enter<U: Clone + Eq + PartialEq<T>>(&self, new_data: Vec<U>) -> Selection<U> where T: PartialEq<U> {
        let enter_data = new_data.into_iter().filter(|d| !self.data.iter().any(|x| x == d)).collect();
        Selection::new(enter_data)
    }

    /// D3.js exit selection: returns items in current data not in new_data
    pub fn exit<U: Clone + Eq + PartialEq<T>>(&self, new_data: Vec<U>) -> Selection<T> where T: Clone + PartialEq<U> {
        let exit_data = self.data.iter().cloned().filter(|d| !new_data.iter().any(|x| x == d)).collect();
        Selection::new(exit_data)
    }

    /// D3.js join: returns a tuple of (enter, update, exit) selections
    pub fn join<U: Clone + Eq + PartialEq<T>>(&self, new_data: Vec<U>) -> (Selection<U>, Selection<U>, Selection<T>) where T: Clone + PartialEq<U> {
        let mut enter = Vec::new();
        let mut update = Vec::new();
        let mut exit = Vec::new();
        let old_len = self.data.len();
        let new_len = new_data.len();
        let min_len = old_len.min(new_len);
        // Update: items with both old and new data (by index)
        for i in 0..min_len {
            update.push(new_data[i].clone());
        }
        // Enter: new data with no corresponding old data
        for i in min_len..new_len {
            enter.push(new_data[i].clone());
        }
        // Exit: old data with no corresponding new data
        for i in min_len..old_len {
            exit.push(self.data[i].clone());
        }
        (Selection::new(enter), Selection::new(update), Selection::new(exit))
    }

    /// D3.js keyed join: returns (enter, update, exit) using key functions
    pub fn join_keyed<U, K, FOld, FNew>(
        &self,
        new_data: Vec<U>,
        old_key: FOld,
        new_key: FNew,
    ) -> (Selection<U>, Selection<U>, Selection<T>)
    where
        T: Clone,
        U: Clone,
        K: Eq + std::hash::Hash,
        FOld: Fn(&T) -> K,
        FNew: Fn(&U) -> K,
    {
        use std::collections::{HashMap, HashSet};
        let mut old_map: HashMap<K, &T> = HashMap::new();
        for d in &self.data {
            old_map.insert(old_key(d), d);
        }
        let mut enter = Vec::new();
        let mut update = Vec::new();
        let mut seen_keys = HashSet::new();
        for d in &new_data {
            let k = new_key(d);
            if old_map.get(&k).is_some() {
                update.push(d.clone());
            } else {
                enter.push(d.clone());
            }
            seen_keys.insert(k);
        }
        let mut exit = Vec::new();
        for d in &self.data {
            let k = old_key(d);
            if !seen_keys.contains(&k) {
                exit.push(d.clone());
            }
        }
        (Selection::new(enter), Selection::new(update), Selection::new(exit))
    }

    /// D3.js transition API stub: returns self for chaining
    pub fn transition(&mut self) -> &mut Self {
        // No-op for API parity
        self
    }
    /// D3.js interrupt API stub: returns self for chaining
    pub fn interrupt(&mut self) -> &mut Self {
        // No-op for API parity
        self
    }

    pub fn select<F>(&self, mut f: F) -> Selection<T>
    where
        F: FnMut(&T) -> bool,
        T: Clone,
    {
        if let Some(item) = self.data.iter().find(|d| f(d)) {
            Selection { data: vec![item.clone()] }
        } else {
            Selection { data: Vec::new() }
        }
    }

    pub fn select_all<F>(&self, mut f: F) -> Selection<T>
    where
        F: FnMut(&T) -> bool,
        T: Clone,
    {
        Selection {
            data: self.data.iter().cloned().filter(|d| f(d)).collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn nodes(&self) -> Vec<&T> {
        self.data.iter().collect()
    }

    pub fn node(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn call<F>(&self, f: F)
    where
        F: FnOnce(&Selection<T>),
    {
        f(self);
    }

    pub fn each<F>(&self, mut f: F)
    where
        F: FnMut(&T, usize),
    {
        for (i, d) in self.data.iter().enumerate() {
            f(d, i);
        }
    }

    pub fn merge(&self, other: &Selection<T>) -> Selection<T>
    where
        T: Clone,
    {
        let mut data = self.data.clone();
        data.extend(other.data.iter().cloned());
        Selection { data }
    }

    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self
    where
        T: AttrSet,
    {
        for d in &mut self.data {
            d.set_attr(name, value);
        }
        self
    }
    pub fn style(&mut self, name: &str, value: &str) -> &mut Self
    where
        T: StyleSet,
    {
        for d in &mut self.data {
            d.set_style(name, value);
        }
        self
    }
    pub fn property(&mut self, name: &str, value: &str) -> &mut Self
    where
        T: PropertySet,
    {
        for d in &mut self.data {
            d.set_property(name, value);
        }
        self
    }
    pub fn classed(&mut self, name: &str, value: bool) -> &mut Self
    where
        T: ClassedSet,
    {
        for d in &mut self.data {
            d.set_classed(name, value);
        }
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self
    where
        T: TextSet,
    {
        for d in &mut self.data {
            d.set_text(value);
        }
        self
    }
    pub fn html(&mut self, value: &str) -> &mut Self
    where
        T: HtmlSet,
    {
        for d in &mut self.data {
            d.set_html(value);
        }
        self
    }
}

impl<T: Clone + PartialEq> Selection<T> {
    /// Move the first item matching the predicate to the end (raise)
    pub fn raise<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        if let Some(pos) = self.data.iter().position(|d| f(d)) {
            let item = self.data.remove(pos);
            self.data.push(item);
        }
    }
    /// Move the first item matching the predicate to the start (lower)
    pub fn lower<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        if let Some(pos) = self.data.iter().position(|d| f(d)) {
            let item = self.data.remove(pos);
            self.data.insert(0, item);
        }
    }
    /// Sort the selection by a comparator
    pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.data.sort_by(|a, b| cmp(a, b));
    }
    /// Order: no-op for flat vectors, included for API parity
    pub fn order(&mut self) {}
}

// Event handling support (minimal parity)
use std::collections::HashMap;

pub struct EventHandler<T> {
    handlers: HashMap<String, Vec<Box<dyn Fn(&T)>>>,
}

impl<T> EventHandler<T> {
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }
    pub fn on<F>(&mut self, event: &str, handler: F)
    where
        F: Fn(&T) + 'static,
    {
        self.handlers.entry(event.to_string()).or_default().push(Box::new(handler));
    }
    pub fn dispatch(&self, event: &str, data: &T) {
        if let Some(handlers) = self.handlers.get(event) {
            for h in handlers {
                h(data);
            }
        }
    }
}

impl<T> Selection<T> {
    // Attach an event handler (for API parity)
    pub fn on<F>(&mut self, event: &str, handler: F) -> &mut Self
    where
        F: Fn(&T) + 'static,
    {
        // No-op: for API parity only, as Rust does not have DOM events
        // Could be extended for custom event systems
        self
    }
    // Dispatch an event (for API parity)
    pub fn dispatch(&self, event: &str) -> &Self {
        // No-op: for API parity only
        self
    }
}

// Tree traversal for NodeLike
impl<T: NodeLike> Selection<T> {
    pub fn children(&self) -> Selection<T> {
        let mut all_children = Vec::new();
        for node in &self.data {
            all_children.extend(node.children());
        }
        Selection::new(all_children)
    }
    pub fn descendants(&self) -> Selection<T> {
        fn collect_descendants<T: NodeLike>(node: &T, out: &mut Vec<T>) {
            for child in node.children() {
                out.push(child.clone_node());
                collect_descendants(&child, out);
            }
        }
        let mut all = Vec::new();
        for node in &self.data {
            collect_descendants(node, &mut all);
        }
        Selection::new(all)
    }
    pub fn parent(&self) -> Selection<T> {
        let mut parents = Vec::new();
        for node in &self.data {
            if let Some(parent) = node.parent() {
                parents.push(parent);
            }
        }
        Selection::new(parents)
    }
    pub fn ancestors(&self) -> Selection<T> {
        let mut all_ancestors = Vec::new();
        for node in &self.data {
            let mut current = node.parent();
            while let Some(parent) = current {
                all_ancestors.push(parent.clone_node());
                current = parent.parent();
            }
        }
        Selection::new(all_ancestors)
    }
}

pub trait AttrSet { fn set_attr(&mut self, name: &str, value: &str); }
pub trait StyleSet { fn set_style(&mut self, name: &str, value: &str); }
pub trait PropertySet { fn set_property(&mut self, name: &str, value: &str); }
pub trait ClassedSet { fn set_classed(&mut self, name: &str, value: bool); }
pub trait TextSet { fn set_text(&mut self, value: &str); }
pub trait HtmlSet { fn set_html(&mut self, value: &str); }


pub trait NodeLike: Clone {
    fn append(&mut self, child: Self);
    fn insert(&mut self, index: usize, child: Self);
    fn remove(&mut self, index: usize);
    fn clone_node(&self) -> Self;
    fn children(&self) -> Vec<Self>;
    fn parent(&self) -> Option<Self>;
}

impl<T: NodeLike> Selection<T> {
    pub fn append(&mut self, child: T) {
        for node in &mut self.data {
            node.append(child.clone_node());
        }
    }
    pub fn insert(&mut self, index: usize, child: T) {
        for node in &mut self.data {
            node.insert(index, child.clone_node());
        }
    }
    pub fn remove(&mut self, index: usize) {
        for node in &mut self.data {
            node.remove(index);
        }
    }
    pub fn clone_selection(&self) -> Selection<T> {
        Selection {
            data: self.data.iter().map(|n| n.clone_node()).collect(),
        }
    }
}

// Re-export traits for downstream use
// (Do not re-export if already in scope)
