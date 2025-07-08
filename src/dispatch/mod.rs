//! d3-dispatch (Rust port)
//!
//! Provides event dispatching and listener registration, similar to d3-dispatch.
//!
//! # Usage Example
//! ```rust
//! use rust_d3::dispatch::Dispatch;
//! let mut d = Dispatch::new();
//! d.on("foo", || println!("foo event"));
//! d.call("foo");
//! ```
//! Remove all listeners for an event
//!
//! # Example
//! ```rust
//! use rust_d3::dispatch::Dispatch;
//! let mut d = Dispatch::new();
//! d.off("foo");
//! ```
//!
//! Call with argument (payload via closure capture)
//! ```rust
//! use rust_d3::dispatch::Dispatch;
//! let mut d = Dispatch::new();
//! let payload = 42;
//! d.on("bar", move || println!("payload: {}", payload));
//! d.call_with("bar", payload);
//! ```
//!
//! Integration Example: Using Dispatch with Selection/Transition
//!
//! ```rust
//! use rust_d3::dispatch::Dispatch;
//! use rust_d3::selection::Selection;
//! let mut sel = Selection::select_all("rect");
//! let mut dispatcher = Dispatch::new();
//! dispatcher.on("custom", || println!("custom event!"));
//! // In a real integration, you might call dispatcher.call("custom") inside a transition or selection event
//! dispatcher.call("custom");
//! ```
//!

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::any::Any;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::borrow::Cow;

/// A handle for a registered event handler, for ergonomic removal.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HandlerHandle(u64);

impl fmt::Debug for HandlerHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandlerHandle({})", self.0)
    }
}

/// Event object passed to handlers (optional, for advanced usage)
pub struct Event<'a> {
    pub event_type: Cow<'a, str>,
    pub data: Option<Arc<dyn Any + Send + Sync>>,
    pub timestamp: std::time::Instant,
    pub source: Option<Cow<'a, str>>,
    pub propagation_stopped: Arc<AtomicBool>,
    pub default_prevented: Arc<AtomicBool>,
}

impl<'a> Event<'a> {
    pub fn stop_propagation(&self) {
        self.propagation_stopped.store(true, Ordering::SeqCst);
    }
    pub fn is_propagation_stopped(&self) -> bool {
        self.propagation_stopped.load(Ordering::SeqCst)
    }
    pub fn prevent_default(&self) {
        self.default_prevented.store(true, Ordering::SeqCst);
    }
    pub fn is_default_prevented(&self) -> bool {
        self.default_prevented.load(Ordering::SeqCst)
    }
}

pub struct Dispatch {
    listeners: Arc<Mutex<HashMap<String, Vec<(Arc<dyn Fn(&Event) + Send + Sync>, HandlerHandle)>>>>,
    async_listeners: Arc<Mutex<HashMap<String, Vec<(Arc<dyn Fn(Arc<Event<'static>>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>, HandlerHandle)>>>>,
    next_id: Arc<Mutex<u64>>,
}

impl Dispatch {
    pub fn new() -> Self {
        Dispatch {
            listeners: Arc::new(Mutex::new(HashMap::new())),
            async_listeners: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
    /// Register a handler and return a handle for ergonomic removal
    pub fn on_with_handle<F: Fn(&Event) + Send + Sync + 'static>(&mut self, event: &str, handler: F) -> HandlerHandle {
        let mut map = self.listeners.lock().unwrap();
        let mut id = self.next_id.lock().unwrap();
        let handle = HandlerHandle(*id);
        *id += 1;
        map.entry(event.to_string()).or_default().push((Arc::new(handler), handle.clone()));
        handle
    }
    /// Register a handler (no handle returned, for simple use)
    pub fn on<F: Fn() + Send + Sync + 'static>(&mut self, event: &str, handler: F) {
        self.on_with_handle(event, move |_| handler());
    }
    /// Remove all handlers for an event
    pub fn off(&mut self, event: &str) {
        let mut map = self.listeners.lock().unwrap();
        map.remove(event);
    }
    /// Remove a handler by handle
    pub fn off_handle(&mut self, event: &str, handle: &HandlerHandle) {
        let mut map = self.listeners.lock().unwrap();
        if let Some(vec) = map.get_mut(event) {
            vec.retain(|(_, h)| h != handle);
        }
    }
    /// Remove a handler by pointer (legacy, discouraged)
    pub fn off_handler(&mut self, event: &str, handler_ptr: *const ()) {
        let mut map = self.listeners.lock().unwrap();
        if let Some(vec) = map.get_mut(event) {
            vec.retain(|(h, _)| {
                let ptr = Arc::as_ptr(h) as *const dyn Fn(&Event) as *const ();
                ptr != handler_ptr
            });
        }
    }
    /// Remove all listeners for all events
    pub fn clear(&mut self) {
        let mut map = self.listeners.lock().unwrap();
        map.clear();
    }
    /// Return the number of listeners for an event
    pub fn listeners(&self, event: &str) -> usize {
        self.listeners.lock().unwrap().get(event).map(|v| v.len()).unwrap_or(0)
    }
    /// Remove all listeners for a namespace (e.g. ".bar")
    pub fn off_namespace(&mut self, namespace: &str) {
        let mut map = self.listeners.lock().unwrap();
        let ns = if namespace.starts_with('.') { &namespace[1..] } else { namespace };
        map.retain(|k, _| !k.split('.').skip(1).any(|n| n == ns));
    }
    /// Call all handlers for an event (no event object, with propagation)
    pub fn call(&self, event: &str) {
        let evt = Event {
            event_type: Cow::Borrowed(event),
            data: None,
            timestamp: std::time::Instant::now(),
            source: None,
            propagation_stopped: Arc::new(AtomicBool::new(false)),
            default_prevented: Arc::new(AtomicBool::new(false)),
        };
        self.call_event(event, &evt);
    }
    /// Call all handlers for an event, passing a payload as event data (with propagation)
    pub fn call_with<T: Send + Sync + 'static>(&self, event: &str, arg: T)
    where
        T: Clone,
    {
        let evt = Event {
            event_type: Cow::Borrowed(event),
            data: Some(Arc::new(arg)),
            timestamp: std::time::Instant::now(),
            source: None,
            propagation_stopped: Arc::new(AtomicBool::new(false)),
            default_prevented: Arc::new(AtomicBool::new(false)),
        };
        self.call_event(event, &evt);
    }
    /// Call all handlers for an event, passing a custom event object (with propagation)
    pub fn call_event(&self, event: &str, evt: &Event) {
        if let Some(list) = self.listeners.lock().unwrap().get(event) {
            for (handler, _) in list {
                handler(evt);
                if evt.is_propagation_stopped() {
                    break;
                }
            }
        }
    }
    /// Call all handlers for all events matching a wildcard (e.g. "foo.*" or "*")
    pub fn call_wildcard(&self, pattern: &str) {
        let matcher = |k: &str| {
            if pattern == "*" { true }
            else if pattern.ends_with(".*") {
                k.starts_with(&pattern[..pattern.len()-2])
            } else {
                k == pattern
            }
        };
        let events: Vec<String> = self.listeners.lock().unwrap()
            .keys()
            .filter(|k| matcher(k))
            .cloned()
            .collect();
        for event in events {
            self.call(&event);
        }
    }
    /// List all registered events
    pub fn events(&self) -> Vec<String> {
        self.listeners.lock().unwrap().keys().cloned().collect()
    }
    /// List all handlers for an event
    pub fn handlers(&self, event: &str) -> Vec<HandlerHandle> {
        self.listeners.lock().unwrap().get(event).map(|v| v.iter().map(|(_, h)| h.clone()).collect()).unwrap_or_default()
    }
    /// Register an async handler and return a handle for ergonomic removal
    pub fn on_async_with_handle<F, Fut>(&mut self, event: &str, handler: F) -> HandlerHandle
    where
        F: Fn(Arc<Event<'static>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let mut map = self.async_listeners.lock().unwrap();
        let mut id = self.next_id.lock().unwrap();
        let handle = HandlerHandle(*id);
        *id += 1;
        map.entry(event.to_string())
            .or_default()
            .push((Arc::new(move |evt| Box::pin(handler(evt))), handle.clone()));
        handle
    }
    /// Register an async handler (no handle returned, for simple use)
    pub fn on_async<F, Fut>(&mut self, event: &str, handler: F)
    where
        F: Fn(Arc<Event<'static>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        self.on_async_with_handle(event, handler);
    }
    /// Call all async handlers for an event (returns a future)
    pub async fn call_async_with(&self, event: &str, evt: Arc<Event<'static>>) {
        if let Some(list) = self.async_listeners.lock().unwrap().get(event) {
            for (handler, _) in list {
                handler(evt.clone()).await;
                if evt.is_propagation_stopped() {
                    break;
                }
            }
        }
    }
    /// Call all async handlers for an event (no event object)
    pub async fn call_async(&self, event: &str) {
        let evt = Arc::new(Event {
            event_type: Cow::Owned(event.to_string()),
            data: None,
            timestamp: std::time::Instant::now(),
            source: None,
            propagation_stopped: Arc::new(AtomicBool::new(false)),
            default_prevented: Arc::new(AtomicBool::new(false)),
        });
        self.call_async_with(event, evt).await;
    }
}
