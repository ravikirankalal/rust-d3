use rust_d3::timer::Timer;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_timer_tick() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
    }, 5);
    timer.start();
    thread::sleep(Duration::from_millis(20));
    timer.stop();
    assert!(count.load(Ordering::SeqCst) >= 3);
}

#[test]
fn test_timer_stop() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
    }, 5);
    timer.start();
    thread::sleep(Duration::from_millis(15));
    timer.stop();
    let after = count.load(Ordering::SeqCst);
    thread::sleep(Duration::from_millis(15));
    assert_eq!(after, count.load(Ordering::SeqCst));
}

#[test]
fn test_timer_restart() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
    }, 2);
    timer.start();
    thread::sleep(Duration::from_millis(6));
    timer.restart();
    thread::sleep(Duration::from_millis(6));
    timer.stop();
    assert!(count.load(Ordering::SeqCst) > 2);
}

#[test]
fn test_timer_delay() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
    }, 2);
    timer.start();
    thread::sleep(Duration::from_millis(5));
    let before = count.load(Ordering::SeqCst);
    timer.delay(10);
    thread::sleep(Duration::from_millis(15));
    timer.stop();
    let after = count.load(Ordering::SeqCst);
    assert!(after > before, "Timer should continue ticking after delay change");
}

#[test]
fn test_timer_is_running() {
    let mut timer = Timer::new(|| {}, 2);
    assert!(!timer.is_running());
    timer.start();
    assert!(timer.is_running());
    timer.stop();
    assert!(!timer.is_running());
}

#[test]
fn test_timer_double_stop_and_restart() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
    }, 2);
    timer.start();
    thread::sleep(Duration::from_millis(5));
    timer.stop();
    timer.stop(); // Should not panic
    timer.start();
    thread::sleep(Duration::from_millis(5));
    timer.stop();
    assert!(count.load(Ordering::SeqCst) > 1);
}

#[test]
fn test_timer_zero_delay() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
    }, 0);
    timer.start();
    thread::sleep(Duration::from_millis(5));
    timer.stop();
    assert!(count.load(Ordering::SeqCst) > 1);
}
