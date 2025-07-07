use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Dispatcher {
    listeners: Arc<Mutex<HashMap<String, Vec<Box<dyn Fn() + Send + Sync>>>>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn on<F>(&self, event: &str, f: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.entry(event.to_string()).or_default().push(Box::new(f));
    }

    pub fn call(&self, event: &str) {
        if let Some(list) = self.listeners.lock().unwrap().get(event) {
            for f in list {
                f();
            }
        }
    }

    /// Placeholder for dispatch.copy()
    pub fn copy(&self) -> Self {
        // Deep copy listeners
        let listeners = self.listeners.lock().unwrap();
        let mut new_map: HashMap<String, Vec<Box<dyn Fn() + Send + Sync>>> = HashMap::new();
        for (k, v) in listeners.iter() {
            let mut new_vec = Vec::new();
            for _ in v {
                // Can't clone trait objects, so just push a no-op
                new_vec.push(Box::new(|| {}) as Box<dyn Fn() + Send + Sync>);
            }
            new_map.insert(k.clone(), new_vec);
        }
        Self {
            listeners: Arc::new(Mutex::new(new_map)),
        }
    }
    /// Placeholder for dispatch.apply(type, that, arguments)
    pub fn apply(&self, event: &str, _args: &[&dyn std::any::Any]) {
        // Just call the event for now
        self.call(event);
    }
}

// Placeholder for d3-dispatch API parity.
// See: https://github.com/d3/d3-dispatch#api-reference
// TODO: Implement full API parity with d3-dispatch (copy, apply, event type generics, etc.)
