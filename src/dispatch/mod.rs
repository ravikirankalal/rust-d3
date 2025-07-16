//! d3-dispatch (Rust port)
//!
//! Provides event dispatching and listener registration, similar to d3-dispatch.
//!
//! # Usage Example
//! ```rust
//! use rust_d3::dispatch::Dispatch;
//! # #[tokio::main]
//! # async fn main() {
//! let mut d = Dispatch::new();
//! d.on("foo", || println!("foo event")).await;
//! d.call("foo").await;
//! # }
//! ```
//! Remove all listeners for an event
//!
//! # Example
//! ```rust
//! use rust_d3::dispatch::Dispatch;
//! # #[tokio::main]
//! # async fn main() {
//! let mut d = Dispatch::new();
//! d.off("foo").await;
//! # }
//! ```
//!
//! Call with argument (payload via closure capture)
//! ```rust
//! use rust_d3::dispatch::Dispatch;
//! # #[tokio::main]
//! # async fn main() {
//! let mut d = Dispatch::new();
//! let payload = 42;
//! d.on("bar", move || println!("payload: {}", payload)).await;
//! d.call_with("bar", payload).await;
//! # }
//! ```
//!
//! Integration Example: Using Dispatch with Selection/Transition
//!
//! ```rust
//! use rust_d3::dispatch::Dispatch;
//! use rust_d3::selection::{Arena, Selection};
//! use slotmap::SlotMap;
//! use std::rc::Rc;
//! use std::cell::RefCell;
//! let arena = Rc::new(RefCell::new(Arena { nodes: SlotMap::with_key() }));
//! let mut root = Selection::root(arena, "root");
//! let mut sel = root.select_all(None);
//! let mut dispatcher = Dispatch::new();
//! // Note: In actual async code, you'd need to await these calls
//! // dispatcher.on("custom", || println!("custom event!")).await;
//! // dispatcher.call("custom").await;
//! ```
//!

use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex as TokioMutex;

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
    listeners:
        Arc<TokioMutex<HashMap<String, Vec<(Arc<dyn Fn(&Event) + Send + Sync>, HandlerHandle)>>>>,
    async_listeners: Arc<
        TokioMutex<
            HashMap<
                String,
                Vec<(
                    Arc<
                        dyn Fn(
                                Arc<Event<'static>>,
                            )
                                -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
                            + Send
                            + Sync,
                    >,
                    HandlerHandle,
                )>,
            >,
        >,
    >,
    next_id: Arc<TokioMutex<u64>>,
}

impl Dispatch {
    pub fn new() -> Self {
        Dispatch {
            listeners: Arc::new(TokioMutex::new(HashMap::new())),
            async_listeners: Arc::new(TokioMutex::new(HashMap::new())),
            next_id: Arc::new(TokioMutex::new(1)),
        }
    }
    /// Register a handler and return a handle for ergonomic removal
    pub async fn on_with_handle<F: Fn(&Event) + Send + Sync + 'static>(
        &self,
        event: &str,
        handler: F,
    ) -> HandlerHandle {
        let mut map = self.listeners.lock().await;
        let mut id = self.next_id.lock().await;
        let handle = HandlerHandle(*id);
        *id += 1;
        map.entry(event.to_string())
            .or_default()
            .push((Arc::new(handler), handle.clone()));
        handle
    }
    /// Register a handler (no handle returned, for simple use)
    pub async fn on<F: Fn() + Send + Sync + 'static>(&self, event: &str, handler: F) {
        self.on_with_handle(event, move |_| handler()).await;
    }
    /// Remove all handlers for an event
    pub async fn off(&self, event: &str) {
        let mut map = self.listeners.lock().await;
        map.remove(event);
    }
    /// Remove a handler by handle
    pub async fn off_handle(&self, event: &str, handle: &HandlerHandle) {
        let mut map = self.listeners.lock().await;
        if let Some(vec) = map.get_mut(event) {
            vec.retain(|(_, h)| h != handle);
        }
    }
    /// Remove a handler by pointer (legacy, discouraged)
    pub async fn off_handler(&self, event: &str, handler_ptr: *const ()) {
        let mut map = self.listeners.lock().await;
        if let Some(vec) = map.get_mut(event) {
            vec.retain(|(h, _)| {
                let ptr = Arc::as_ptr(h) as *const dyn Fn(&Event) as *const ();
                ptr != handler_ptr
            });
        }
    }
    /// Remove a handler by pointer from all events (ergonomic global removal)
    pub async fn off_handler_global(&self, handler_ptr: *const ()) {
        let mut map = self.listeners.lock().await;
        for vec in map.values_mut() {
            vec.retain(|(h, _)| {
                let ptr = Arc::as_ptr(h) as *const dyn Fn(&Event) as *const ();
                ptr != handler_ptr
            });
        }
    }
    /// Remove all listeners for all events
    pub async fn clear(&self) {
        let mut map = self.listeners.lock().await;
        map.clear();
    }
    /// Return the number of listeners for an event
    pub async fn listeners(&self, event: &str) -> usize {
        self.listeners
            .lock()
            .await
            .get(event)
            .map(|v| v.len())
            .unwrap_or(0)
    }
    /// Remove all listeners for a namespace (e.g. ".bar")
    pub async fn off_namespace(&self, namespace: &str) {
        let mut map = self.listeners.lock().await;
        let ns = if namespace.starts_with('.') {
            &namespace[1..]
        } else {
            namespace
        };
        map.retain(|k, _| !k.split('.').skip(1).any(|n| n == ns));
    }
    /// Call all handlers for an event (no event object, with propagation)
    pub async fn call(&self, event: &str) {
        let evt = Arc::new(Event {
            event_type: Cow::Owned(event.to_string()),
            data: None,
            timestamp: std::time::Instant::now(),
            source: None,
            propagation_stopped: Arc::new(AtomicBool::new(false)),
            default_prevented: Arc::new(AtomicBool::new(false)),
        });
        self.call_event(event, evt).await;
    }
    /// Call all handlers for an event, passing a payload as event data (with propagation)
    pub async fn call_with<T: Send + Sync + 'static>(&self, event: &str, arg: T)
    where
        T: Clone,
    {
        let evt = Arc::new(Event {
            event_type: Cow::Owned(event.to_string()),
            data: Some(Arc::new(arg)),
            timestamp: std::time::Instant::now(),
            source: None,
            propagation_stopped: Arc::new(AtomicBool::new(false)),
            default_prevented: Arc::new(AtomicBool::new(false)),
        });
        self.call_event(event, evt).await;
    }
    /// Call all handlers for an event, passing a custom event object (with propagation)
    pub async fn call_event(&self, event: &str, evt: Arc<Event<'static>>) {
        if let Some(list) = self.listeners.lock().await.get(event) {
            for (handler, _) in list {
                handler(&*evt);
                if evt.is_propagation_stopped() {
                    break;
                }
            }
        }
    }
    /// Call all handlers for all events matching a wildcard (e.g. "foo.*" or "*")
    pub async fn call_wildcard(&self, pattern: &str) {
        let matcher = |k: &str| {
            if pattern == "*" {
                true
            } else if pattern.ends_with(".*") {
                k.starts_with(&pattern[..pattern.len() - 2])
            } else {
                k == pattern
            }
        };
        let events: Vec<String> = self
            .listeners
            .lock()
            .await
            .keys()
            .filter(|k| matcher(k))
            .cloned()
            .collect();
        for event in events {
            self.call(&event).await;
        }
    }
    /// List all registered events
    pub async fn events(&self) -> Vec<String> {
        self.listeners.lock().await.keys().cloned().collect()
    }
    /// List all handlers for an event
    pub async fn handlers(&self, event: &str) -> Vec<HandlerHandle> {
        self.listeners
            .lock()
            .await
            .get(event)
            .map(|v| v.iter().map(|(_, h)| h.clone()).collect())
            .unwrap_or_default()
    }
    /// Register an async handler and return a handle for ergonomic removal
    pub async fn on_async_with_handle<F, Fut>(&self, event: &str, handler: F) -> HandlerHandle
    where
        F: Fn(Arc<Event<'static>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let mut map = self.async_listeners.lock().await;
        let mut id = self.next_id.lock().await;
        let handle = HandlerHandle(*id);
        *id += 1;
        map.entry(event.to_string())
            .or_default()
            .push((Arc::new(move |evt| Box::pin(handler(evt))), handle.clone()));
        handle
    }
    /// Register an async handler (no handle returned, for simple use)
    pub async fn on_async<F, Fut>(&self, event: &str, handler: F)
    where
        F: Fn(Arc<Event<'static>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        self.on_async_with_handle(event, handler).await;
    }
    /// Call all async handlers for an event (returns a future)
    pub async fn call_async_with(&self, event: &str, evt: Arc<Event<'static>>) {
        if let Some(list) = self.async_listeners.lock().await.get(event) {
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
    /// Helper for async event bubbling: propagate to parent if not stopped
    pub async fn call_event_async_bubble(
        &self,
        parent: &Dispatch,
        event: &str,
        evt: Arc<Event<'static>>,
    ) {
        if !evt.is_propagation_stopped() {
            parent.call_async_with(event, evt.clone()).await;
        }
    }
}
