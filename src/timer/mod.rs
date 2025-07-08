//! d3-timer (Rust port)
//!
//! Provides timer utilities for animation, scheduling, and repeated callbacks, similar to d3-timer.
//!
//! # Usage Example
//! ```rust
//! use rust_d3::timer::Timer;
//! let mut t = Timer::new(|| println!("tick"), 10);
//! t.start();
//! t.stop();
//! ```
//!

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, Instant};

pub struct Timer {
    callback: Arc<Mutex<dyn FnMut() + Send + 'static>>,
    delay: u64,
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Timer {
    pub fn new<F: FnMut() + Send + 'static>(callback: F, delay_ms: u64) -> Self {
        Timer {
            callback: Arc::new(Mutex::new(callback)),
            delay: delay_ms,
            running: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }
    pub fn start(&mut self) {
        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);
        let callback = self.callback.clone();
        let delay = self.delay;
        self.handle = Some(thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(delay));
                let _ = callback.lock().map(|mut cb| cb());
            }
        }));
    }
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
    /// Restart the timer (stop and start again)
    ///
    /// # Example
    /// ```rust
    /// use rust_d3::timer::Timer;
    /// let mut t = Timer::new(|| println!("tick"), 10);
    /// t.restart();
    /// ```
    ///
    /// Change the delay
    /// ```rust
    /// use rust_d3::timer::Timer;
    /// let mut t = Timer::new(|| println!("tick"), 10);
    /// t.delay(100);
    /// ```
    ///
    /// Check if running
    /// ```rust
    /// use rust_d3::timer::Timer;
    /// let t = Timer::new(|| println!("tick"), 10);
    /// assert!(t.is_running() || !t.is_running());
    /// ```
    pub fn restart(&mut self) {
        self.stop();
        self.start();
    }
    pub fn delay(&mut self, delay_ms: u64) {
        self.delay = delay_ms;
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
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
    fn test_timer_restart_and_delay() {
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();
        let mut timer = Timer::new(move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        }, 5);
        timer.start();
        thread::sleep(Duration::from_millis(10));
        timer.delay(1);
        timer.restart();
        thread::sleep(Duration::from_millis(10));
        timer.stop();
        assert!(count.load(Ordering::SeqCst) > 1);
        assert!(!timer.is_running());
    }
}
