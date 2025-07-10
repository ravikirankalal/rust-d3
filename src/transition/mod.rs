//! d3-transition (Rust port)
//!
//! This module ports the D3.js d3-transition API to Rust.
//! It enables animated transitions for selections, attributes, and styles.
//! Timing is simulated (no real DOM or animation frames).

use std::time::{Duration};
use std::collections::HashMap;
use std::sync::Arc;
use crate::selection::Selection;

pub struct Transition {
    pub selection: Selection,
    pub duration: Duration,
    pub delay: Duration,
    pub ease: fn(f32) -> f32,
    pub event_handlers: Arc<HashMap<String, Vec<Arc<dyn Fn() + Send + Sync>>>>,
}

impl Clone for Transition {
    fn clone(&self) -> Self {
        Transition {
            selection: self.selection.clone(),
            duration: self.duration,
            delay: self.delay,
            ease: self.ease,
            event_handlers: self.event_handlers.clone(),
        }
    }
}

impl Transition {
    pub fn new(selection: Selection) -> Self {
        Transition {
            selection,
            duration: Duration::from_millis(250),
            delay: Duration::from_millis(0),
            ease: |t| t, // linear by default
            event_handlers: Arc::new(HashMap::new()),
        }
    }
    pub fn duration(mut self, ms: u64) -> Self {
        self.duration = Duration::from_millis(ms);
        self
    }
    pub fn delay(mut self, ms: u64) -> Self {
        self.delay = Duration::from_millis(ms);
        self
    }
    pub fn ease(mut self, f: fn(f32) -> f32) -> Self {
        self.ease = f;
        self
    }
    pub fn attr(self, name: &str, to: &str) -> Self {
        let mut sel = self.selection.clone();
        let name = name.to_string();
        let to = to.to_string();
        let delay = self.delay;
        let duration = self.duration;
        let handlers = self.event_handlers.clone();
        std::thread::spawn(move || {
            if let Some(hs) = handlers.get("start") {
                for h in hs { h(); }
            }
            std::thread::sleep(delay + duration);
            sel.attr(&name, &to);
            if let Some(hs) = handlers.get("end") {
                for h in hs { h(); }
            }
        });
        self
    }
    pub fn style(self, name: &str, to: &str) -> Self {
        let mut sel = self.selection.clone();
        let name = name.to_string();
        let to = to.to_string();
        let delay = self.delay;
        let duration = self.duration;
        let handlers = self.event_handlers.clone();
        std::thread::spawn(move || {
            if let Some(hs) = handlers.get("start") {
                for h in hs { h(); }
            }
            std::thread::sleep(delay + duration);
            sel.style(&name, &to);
            if let Some(hs) = handlers.get("end") {
                for h in hs { h(); }
            }
        });
        self
    }
    pub fn on<F: Fn() + Send + Sync + 'static>(mut self, event: &str, handler: F) -> Self {
        let mut handlers = (*self.event_handlers).clone();
        handlers.entry(event.to_string()).or_default().push(Arc::new(handler));
        self.event_handlers = Arc::new(handlers);
        self
    }
    /// Interrupt the transition (stub)
    pub fn interrupt(self) -> Self {
        // Not implemented: would require async control
        self
    }
    /// Remove nodes at the end of the transition (stub)
    pub fn remove(self) -> Self {
        let mut sel = self.selection.clone();
        let delay = self.delay;
        let duration = self.duration;
        std::thread::spawn(move || {
            std::thread::sleep(delay + duration);
            sel.remove();
        });
        self
    }
        /// Returns the active transition on a given node (stub).
    pub fn active(_node: &crate::selection::Node) -> Option<Self> {
        None // Not implemented
    }

    /// Transitions the value of the specified attribute using the specified tween function (stub).
    pub fn attr_tween<F>(self, _name: &str, _tween: F) -> Self
    where
        F: Fn() -> Box<dyn Fn(f64) -> String> + 'static,
    {
        self // Not implemented
    }

    /// Transitions the value of the specified style property using the specified tween function (stub).
    pub fn style_tween<F>(self, _name: &str, _tween: F) -> Self
    where
        F: Fn() -> Box<dyn Fn(f64) -> String> + 'static,
    {
        self // Not implemented
    }

    /// Transitions the text content using the specified tween function (stub).
    pub fn text_tween<F>(self, _tween: F) -> Self
    where
        F: Fn() -> Box<dyn Fn(f64) -> String> + 'static,
    {
        self // Not implemented
    }

    /// Specifies a different easing function for each element (stub).
    pub fn ease_varying<F>(self, _factory: F) -> Self
    where
        F: Fn(&crate::selection::Node) -> fn(f32) -> f32 + 'static,
    {
        self // Not implemented
    }

    /// Returns a promise that resolves when all transitions in the group have finished (stub).
    pub fn end(self) -> bool {
        false // Not implemented
    }

    /// Invokes the specified function for each selected element in the transition.
    pub fn each<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut crate::selection::Node) + 'static,
    {
        for node in &mut self.selection.nodes {
            f(node);
        }
        self
    }

    /// Invokes the specified function once, passing in the transition along with any optional arguments.
    pub fn call<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }

    /// For each selected element, selects the first child element that matches the specified selector string.
    /// If no selector is provided, selects the first child.
    pub fn select_child(&self) -> Self {
        Self {
            selection: self.selection.select_child(),
            ..self.clone()
        }
    }

    /// For each selected element, selects all children elements that match the specified selector string.
    /// If no selector is provided, selects all children.
    pub fn select_children(&self) -> Self {
        Self {
            selection: self.selection.select_children(),
            ..self.clone()
        }
    }

    /// Merges this transition with the specified other transition.
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            selection: self.selection.merge(&other.selection),
            ..self.clone()
        }
    }

    /// Returns true if the transition is empty.
    pub fn empty(&self) -> bool {
        self.selection.empty()
    }

    /// Returns the number of elements in the transition.
    pub fn size(&self) -> usize {
        self.selection.size()
    }

    /// Returns a vector of all nodes in the transition.
    pub fn nodes(&self) -> &Vec<crate::selection::Node> {
        self.selection.nodes()
    }

    /// Returns the first non-null node in the transition.
    pub fn node(&self) -> Option<&crate::selection::Node> {
        self.selection.node()
    }

    /// Returns the selection that the transition is operating on.
    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    /// For each selected element, selects all descendant elements that match the specified selector string.
    /// Returns a new transition on the selected descendants.
    pub fn select_all(&self, selector: Option<&str>) -> Self {
        let mut new_selection = self.selection.clone();
        new_selection.nodes = new_selection
            .nodes
            .iter()
            .flat_map(|node| {
                node.children
                    .iter()
                    .filter(|child| selector.map_or(true, |s| child.tag == s))
                    .cloned()
            })
            .collect();

        Self {
            selection: new_selection,
            duration: self.duration,
            delay: self.delay,
            ease: self.ease,
            event_handlers: self.event_handlers.clone(),
        }
    }

    /// For each selected element, selects the first descendant element that matches the specified selector string.
    /// Returns a new transition on the selected descendants.
    pub fn select(&self, selector: &str) -> Self {
        let mut new_selection = self.selection.clone();
        new_selection.nodes = new_selection
            .nodes
            .iter()
            .map(|node| {
                // In a real DOM, we would query. Here, we just take the first child if the tag matches.
                node.children
                    .iter()
                    .find(|child| child.tag == selector)
                    .cloned()
                    .unwrap_or_else(|| crate::selection::Node::new("empty")) // Return an empty node if not found
            })
            .collect();

        Self {
            selection: new_selection,
            duration: self.duration,
            delay: self.delay,
            ease: self.ease,
            event_handlers: self.event_handlers.clone(),
        }
    }

    /// Filters the transition, returning a new transition containing only the elements for which the specified filter is true.
    /// The filter is a function that receives the index and the element.
    pub fn filter<F>(&self, filter: F) -> Self
    where
        F: Fn(usize, &crate::selection::Node) -> bool,
    {
        let mut new_selection = self.selection.clone();
        new_selection.nodes = new_selection
            .nodes
            .into_iter()
            .enumerate()
            .filter(|(i, node)| filter(*i, node))
            .map(|(_, node)| node)
            .collect();

        Self {
            selection: new_selection,
            duration: self.duration,
            delay: self.delay,
            ease: self.ease,
            event_handlers: self.event_handlers.clone(),
        }
    }

    /// Creates a new transition that starts after the current transition ends.
    /// The new transition inherits the duration, ease, and event handlers of the current transition.
    pub fn transition(self) -> Self {
        Self {
            selection: self.selection.clone(),
            duration: self.duration,
            delay: self.delay + self.duration, // Key for chaining: new delay is old delay + duration
            ease: self.ease,
            event_handlers: self.event_handlers.clone(),
        }
    }
}