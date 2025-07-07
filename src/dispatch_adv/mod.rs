//! D3 Dispatch Advanced module
//! More event dispatching, etc.

use std::sync::{Arc, Mutex};

/// Dispatches an event only once.
pub fn dispatch_once<F: FnOnce()>(f: Arc<Mutex<Option<F>>>) {
    if let Some(func) = f.lock().unwrap().take() {
        func();
    }
}
