// D3 timer module for Rust D3
// Provides a simple timer utility similar to d3-timer.
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::collections::VecDeque;

/// Handle to stop a running timer
pub struct TimerHandle {
    stop: Arc<AtomicBool>,
}

impl TimerHandle {
    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

// Global registry for timer flush
lazy_static::lazy_static! {
    static ref TIMER_QUEUE: Arc<Mutex<VecDeque<Box<dyn FnMut(f64) + Send>>>> = Arc::new(Mutex::new(VecDeque::new()));
}

/// Schedules a callback to be called repeatedly with elapsed time (ms). Returns a handle to stop the timer.
pub fn timer<F>(callback: F, delay: u64) -> TimerHandle
where
    F: FnMut(f64) + Send + 'static,
{
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();
    let mut cb_boxed = Box::new(callback) as Box<dyn FnMut(f64) + Send>;
    thread::spawn(move || {
        let start = Instant::now();
        thread::sleep(Duration::from_millis(delay));
        while !stop_clone.load(Ordering::SeqCst) {
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            cb_boxed(elapsed);
            thread::sleep(Duration::from_millis(16)); // ~60fps
        }
    });
    TimerHandle { stop }
}

/// Register a callback for immediate execution on timer_flush (for testing/flush semantics)
pub fn register_timer_flush<F>(callback: F)
where
    F: FnMut(f64) + Send + 'static,
{
    let mut queue = TIMER_QUEUE.lock().unwrap();
    queue.push_back(Box::new(callback));
}

/// Immediately executes all pending timer callbacks (for testing/flush semantics)
pub fn timer_flush() {
    let mut queue = TIMER_QUEUE.lock().unwrap();
    let now = Instant::now();
    for cb in queue.iter_mut() {
        cb(now.elapsed().as_secs_f64() * 1000.0);
    }
    queue.clear();
}

pub fn timeout<F: FnOnce() + Send + 'static>(f: F, ms: u64) {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(ms));
        f();
    });
}

pub fn interval<F: Fn() + Send + 'static>(f: F, ms: u64, count: usize) {
    thread::spawn(move || {
        for _ in 0..count {
            thread::sleep(Duration::from_millis(ms));
            f();
        }
    });
}

pub fn now() -> u128 {
    Instant::now().elapsed().as_millis()
}
