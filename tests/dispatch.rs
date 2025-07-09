use std::sync::{Arc, Mutex};
use rust_d3::dispatch::Dispatch;

#[tokio::test]
async fn test_dispatch_event() {
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();
    let d = Dispatch::new();
    d.on("foo", move || {
        let mut flag = called_clone.lock().unwrap();
        *flag = true;
    }).await;
    d.call("foo").await;
    assert!(*called.lock().unwrap());
}

#[tokio::test]
async fn test_dispatch_multiple_listeners() {
    let count = Arc::new(Mutex::new(0));
    let count1 = count.clone();
    let count2 = count.clone();
    let d = Dispatch::new();
    d.on("bar", move || {
        let mut c = count1.lock().unwrap();
        *c += 1;
    }).await;
    d.on("bar", move || {
        let mut c = count2.lock().unwrap();
        *c += 1;
    }).await;
    d.call("bar").await;
    assert_eq!(*count.lock().unwrap(), 2);
}

#[tokio::test]
async fn test_dispatch_on_with_handle_and_off_handle() {
    let called = Arc::new(Mutex::new(0));
    let called_clone = called.clone();
    let d = Dispatch::new();
    let handle = d.on_with_handle("foo", move |_| {
        let mut flag = called_clone.lock().unwrap();
        *flag += 1;
    }).await;
    d.call("foo").await;
    d.off_handle("foo", &handle).await;
    d.call("foo").await;
    assert_eq!(*called.lock().unwrap(), 1);
}

#[tokio::test]
async fn test_dispatch_event_object() {
    let called = Arc::new(Mutex::new(0));
    let called_clone = called.clone();
    let d = Dispatch::new();
    d.on_with_handle("foo", move |evt| {
        if let Some(data) = evt.data.as_ref() {
            if let Some(val) = data.downcast_ref::<i32>() {
                let mut flag = called_clone.lock().unwrap();
                *flag += *val;
            }
        }
    }).await;
    d.call_with("foo", 7).await;
    assert_eq!(*called.lock().unwrap(), 7);
}

#[tokio::test]
async fn test_dispatch_multiple_and_payload() {
    let count = Arc::new(Mutex::new(0));
    let count1 = count.clone();
    let count2 = count.clone();
    let d = Dispatch::new();
    let payload = 42;
    d.on("bar", move || {
        let mut c = count1.lock().unwrap();
        *c += payload;
    }).await;
    d.on("bar", move || {
        let mut c = count2.lock().unwrap();
        *c += 1;
    }).await;
    d.call_with("bar", payload).await;
    assert_eq!(*count.lock().unwrap(), 43);
}

#[tokio::test]
async fn test_dispatch_multiple_events() {
    let foo = Arc::new(Mutex::new(0));
    let bar = Arc::new(Mutex::new(0));
    let foo_clone = foo.clone();
    let bar_clone = bar.clone();
    let d = Dispatch::new();
    d.on("foo", move || {
        let mut f = foo_clone.lock().unwrap();
        *f += 1;
    }).await;
    d.on("bar", move || {
        let mut b = bar_clone.lock().unwrap();
        *b += 2;
    }).await;
    d.call("foo").await;
    d.call("bar").await;
    assert_eq!(*foo.lock().unwrap(), 1);
    assert_eq!(*bar.lock().unwrap(), 2);
}

#[tokio::test]
async fn test_dispatch_off_idempotent() {
    let d = Dispatch::new();
    d.on("foo", || {}).await;
    d.off("foo").await;
    d.off("foo").await; // Should not panic
    d.call("foo").await; // Should do nothing
}

#[tokio::test]
async fn test_dispatch_no_listeners() {
    let d = Dispatch::new();
    d.call("noevent").await; // Should not panic
    d.call_with("noevent", 123).await; // Should not panic
}

#[tokio::test]
async fn test_dispatch_clear_and_listeners() {
    let d = Dispatch::new();
    d.on("foo", || {}).await;
    d.on("foo", || {}).await;
    d.on("bar", || {}).await;
    assert_eq!(d.listeners("foo").await, 2);
    assert_eq!(d.listeners("bar").await, 1);
    d.clear().await;
    assert_eq!(d.listeners("foo").await, 0);
    assert_eq!(d.listeners("bar").await, 0);
}

#[tokio::test]
async fn test_dispatch_off_namespace() {
    let called = Arc::new(Mutex::new(0));
    let called_clone = called.clone();
    let d = Dispatch::new();
    d.on("foo.bar", move || {
        let mut flag = called_clone.lock().unwrap();
        *flag += 1;
    }).await;
    d.on("foo.baz", || {}).await;
    d.on("foo", || {}).await;
    d.off_namespace("bar").await;
    d.call("foo.bar").await;
    d.call("foo.baz").await;
    d.call("foo").await;
    assert_eq!(*called.lock().unwrap(), 0);
    assert_eq!(d.listeners("foo.baz").await, 1);
    assert_eq!(d.listeners("foo").await, 1);
}

#[tokio::test]
async fn test_dispatch_integration_selection() {
    // Simulate integration with Selection/Transition
    struct DummySel;
    impl DummySel {
        // fn on_event<F: Fn() + Send + Sync + 'static>(&self, _event: &str, _handler: F) {}
    }
    let _sel = DummySel;
    let d = Dispatch::new();
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();
    d.on("custom", move || {
        let mut flag = called_clone.lock().unwrap();
        *flag = true;
    }).await;
    // In a real integration, you might call dispatcher.call("custom") inside a transition or selection event
    d.call("custom").await;
    assert!(*called.lock().unwrap());
}
