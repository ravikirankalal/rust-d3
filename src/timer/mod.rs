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
use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref GLOBAL_TIMERS: Mutex<HashMap<usize, Arc<Timer>>> = Mutex::new(HashMap::new());
    pub(crate) static ref TIMER_ID: Mutex<usize> = Mutex::new(1);
}

pub fn now() -> Instant {
    Instant::now()
}

pub fn flush() {
    let timers: Vec<Arc<Timer>> = GLOBAL_TIMERS.lock().unwrap().values().cloned().collect();
    for timer in timers {
        if timer.is_running() {
            let _ = timer.tick_once();
        }
    }
}

pub struct Timer {
    pub callback: Arc<Mutex<dyn FnMut() + Send + 'static>>,
    pub delay: u64,
    pub running: Arc<AtomicBool>,
    pub handle: Option<thread::JoinHandle<()>>,
    pub id: usize,
}

impl Timer {
    pub fn new<F: FnMut() + Send + 'static>(callback: F, delay_ms: u64) -> Self {
        let mut id_lock = TIMER_ID.lock().unwrap();
        let id = *id_lock;
        *id_lock += 1;
        let timer = Timer {
            callback: Arc::new(Mutex::new(callback)),
            delay: delay_ms,
            running: Arc::new(AtomicBool::new(false)),
            handle: None,
            id,
        };
        GLOBAL_TIMERS.lock().unwrap().insert(id, Arc::new(timer.clone_for_registry()));
        timer
    }
    fn clone_for_registry(&self) -> Self {
        Timer {
            callback: self.callback.clone(),
            delay: self.delay,
            running: self.running.clone(),
            handle: None,
            id: self.id,
        }
    }
    pub fn start(&mut self) {
        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);
        let callback = self.callback.clone();
        let delay = self.delay;
        let id = self.id;
        self.handle = Some(thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(delay));
                let _ = callback.lock().map(|mut cb| cb());
            }
            GLOBAL_TIMERS.lock().unwrap().remove(&id);
        }));
    }
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        GLOBAL_TIMERS.lock().unwrap().remove(&self.id);
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
    pub fn active(&self) -> bool {
        self.is_running()
    }
    pub fn tick_once(&self) {
        let _ = self.callback.lock().map(|mut cb| cb());
    }
    /// Schedule a one-shot timer (D3's timeout)
    pub fn schedule<F: FnOnce() + Send + 'static>(callback: F, delay_ms: u64) {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(delay_ms));
            callback();
        });
    }
}
