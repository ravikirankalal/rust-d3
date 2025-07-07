//! Unit tests for d3 Dispatcher
use rust_d3::dispatch::Dispatcher;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn test_dispatcher_on_call() {
    let dispatcher = Dispatcher::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let c2 = counter.clone();
    dispatcher.on("event", move || {
        c2.fetch_add(1, Ordering::SeqCst);
    });
    dispatcher.call("event");
    dispatcher.call("event");
    assert_eq!(counter.load(Ordering::SeqCst), 2);
}

#[test]
fn test_dispatcher_copy_and_apply() {
    let dispatcher = Dispatcher::new();
    dispatcher.on("foo", || {});
    let copy = dispatcher.copy();
    copy.call("foo"); // Should not panic
    dispatcher.apply("foo", &[]); // Should not panic
}
