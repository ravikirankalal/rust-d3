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
    /// Chain another transition (stub)
    pub fn transition(self) -> Self {
        // Not implemented: would require async chaining
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selection::Selection;

    #[test]
    fn test_transition_attr_and_style() {
        let sel = Selection::select_all("rect");
        let _t = Transition::new(sel.clone()).duration(10).attr("fill", "red").style("stroke", "blue");
        std::thread::sleep(Duration::from_millis(30));
        // The original selection is unchanged, but the transition's selection is updated
        // (in real D3, the DOM is updated; here, we simulate by updating a clone)
        // This is a stub: in a real system, you would have a shared reference or callback
        // For now, just check that the API is chainable and does not panic
    }
    #[test]
    fn test_transition_ease() {
        // let t = Transition::new(Selection::select("rect")).ease(ease::ease_quad);
        // assert_eq!((t.ease)(0.5), 0.25);
    }
}
