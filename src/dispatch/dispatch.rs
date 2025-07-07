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

    
