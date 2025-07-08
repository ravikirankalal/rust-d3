use rust_d3::dispatch::Dispatch;
use std::sync::{Arc, Mutex};

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
