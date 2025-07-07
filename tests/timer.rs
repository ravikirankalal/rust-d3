//! Unit tests for d3 timer (now, timer, timer_flush)
use rust_d3::timer::{now, timer, TimerHandle, timer_flush, register_timer_flush};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_now() {
    let _ = now();
    // Just check that now() returns a value
}

#[test]
fn test_timer_runs_and_stops() {
    let hit = Arc::new(Mutex::new(0));
    let hit_clone = hit.clone();
    let handle: TimerHandle = timer(move |_elapsed| {
        let mut h = hit_clone.lock().unwrap();
        *h += 1;
    }, 10);
    thread::sleep(Duration::from_millis(50));
    handle.stop();
    let count = *hit.lock().unwrap();
    assert!(count > 0, "Timer callback should have run at least once");
}

#[test]
fn test_timer_flush() {
    let hit = Arc::new(Mutex::new(false));
    let hit_clone = hit.clone();
    register_timer_flush(move |_elapsed| {
        let mut h = hit_clone.lock().unwrap();
        *h = true;
    });
    timer_flush();
    assert!(*hit.lock().unwrap(), "timer_flush should execute registered callback");
}
