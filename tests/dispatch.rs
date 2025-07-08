use rust_d3::dispatch::{Dispatch, HandlerHandle, Event};
use std::sync::{Arc, Mutex};
use std::any::Any;

#[test]
fn test_dispatch_event() {
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();
    let mut d = Dispatch::new();
    d.on("foo", move || {
        let mut flag = called_clone.lock().unwrap();
        *flag = true;
    });
    d.call("foo");
    assert!(*called.lock().unwrap());
}

#[test]
fn test_dispatch_multiple_listeners() {
    let count = Arc::new(Mutex::new(0));
    let count1 = count.clone();
    let count2 = count.clone();
    let mut d = Dispatch::new();
    d.on("bar", move || {
        let mut c = count1.lock().unwrap();
        *c += 1;
    });
    d.on("bar", move || {
        let mut c = count2.lock().unwrap();
        *c += 1;
    });
    d.call("bar");
    assert_eq!(*count.lock().unwrap(), 2);
}

#[test]
fn test_dispatch_on_with_handle_and_off_handle() {
    let called = Arc::new(Mutex::new(0));
    let called_clone = called.clone();
    let mut d = Dispatch::new();
    let handle = d.on_with_handle("foo", move |_| {
        let mut flag = called_clone.lock().unwrap();
        *flag += 1;
    });
    d.call("foo");
    d.off_handle("foo", &handle);
    d.call("foo");
    assert_eq!(*called.lock().unwrap(), 1);
}

#[test]
fn test_dispatch_event_object() {
    let called = Arc::new(Mutex::new(0));
    let called_clone = called.clone();
    let mut d = Dispatch::new();
    d.on_with_handle("foo", move |evt| {
        if let Some(data) = evt.data {
            if let Some(val) = data.downcast_ref::<i32>() {
                let mut flag = called_clone.lock().unwrap();
                *flag += *val;
            }
        }
    });
    d.call_with("foo", 7);
    assert_eq!(*called.lock().unwrap(), 7);
}

#[test]
fn test_dispatch_multiple_and_payload() {
    let count = Arc::new(Mutex::new(0));
    let count1 = count.clone();
    let count2 = count.clone();
    let mut d = Dispatch::new();
    let payload = 42;
    d.on("bar", move || {
        let mut c = count1.lock().unwrap();
        *c += payload;
    });
    d.on("bar", move || {
        let mut c = count2.lock().unwrap();
        *c += 1;
    });
    d.call_with("bar", payload);
    assert_eq!(*count.lock().unwrap(), 43);
}

#[test]
fn test_dispatch_multiple_events() {
    let foo = Arc::new(Mutex::new(0));
    let bar = Arc::new(Mutex::new(0));
    let foo_clone = foo.clone();
    let bar_clone = bar.clone();
    let mut d = Dispatch::new();
    d.on("foo", move || {
        let mut f = foo_clone.lock().unwrap();
        *f += 1;
    });
    d.on("bar", move || {
        let mut b = bar_clone.lock().unwrap();
        *b += 2;
    });
    d.call("foo");
    d.call("bar");
    assert_eq!(*foo.lock().unwrap(), 1);
    assert_eq!(*bar.lock().unwrap(), 2);
}

#[test]
fn test_dispatch_off_idempotent() {
    let mut d = Dispatch::new();
    d.on("foo", || {});
    d.off("foo");
    d.off("foo"); // Should not panic
    d.call("foo"); // Should do nothing
}

#[test]
fn test_dispatch_no_listeners() {
    let d = Dispatch::new();
    d.call("noevent"); // Should not panic
    d.call_with("noevent", 123); // Should not panic
}

#[test]
fn test_dispatch_clear_and_listeners() {
    let mut d = Dispatch::new();
    d.on("foo", || {});
    d.on("foo", || {});
    d.on("bar", || {});
    assert_eq!(d.listeners("foo"), 2);
    assert_eq!(d.listeners("bar"), 1);
    d.clear();
    assert_eq!(d.listeners("foo"), 0);
    assert_eq!(d.listeners("bar"), 0);
}

#[test]
fn test_dispatch_off_namespace() {
    let called = Arc::new(Mutex::new(0));
    let called_clone = called.clone();
    let mut d = Dispatch::new();
    d.on("foo.bar", move || {
        let mut flag = called_clone.lock().unwrap();
        *flag += 1;
    });
    d.on("foo.baz", || {});
    d.on("foo", || {});
    d.off_namespace("bar");
    d.call("foo.bar");
    d.call("foo.baz");
    d.call("foo");
    assert_eq!(*called.lock().unwrap(), 0);
    assert_eq!(d.listeners("foo.baz"), 1);
    assert_eq!(d.listeners("foo"), 1);
}

#[test]
fn test_dispatch_integration_selection() {
    // Simulate integration with Selection/Transition
    struct DummySel;
    impl DummySel {
        fn on_event<F: Fn() + Send + Sync + 'static>(&self, _event: &str, _handler: F) {}
    }
    let sel = DummySel;
    let mut dispatcher = Dispatch::new();
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();
    dispatcher.on("custom", move || {
        let mut flag = called_clone.lock().unwrap();
        *flag = true;
    });
    // In a real integration, you might call dispatcher.call("custom") inside a transition or selection event
    dispatcher.call("custom");
    assert!(*called.lock().unwrap());
}
