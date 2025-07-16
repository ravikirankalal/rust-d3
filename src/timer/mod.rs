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

use std::collections::HashMap;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};

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
    pub paused: Arc<AtomicBool>,
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
            paused: Arc::new(AtomicBool::new(false)),
            handle: None,
            id,
        };
        GLOBAL_TIMERS
            .lock()
            .unwrap()
            .insert(id, Arc::new(timer.clone_for_registry()));
        timer
    }
    fn clone_for_registry(&self) -> Self {
        Timer {
            callback: self.callback.clone(),
            delay: self.delay,
            running: self.running.clone(),
            paused: self.paused.clone(),
            handle: None,
            id: self.id,
        }
    }
    pub fn start(&mut self) {
        let running = self.running.clone();
        let paused = self.paused.clone();
        running.store(true, Ordering::SeqCst);
        let callback = self.callback.clone();
        let delay = self.delay;
        let id = self.id;
        self.handle = Some(thread::spawn(move || {
            // D3 behavior: always wait the full delay before the first tick
            thread::sleep(Duration::from_millis(delay));
            while running.load(Ordering::SeqCst) {
                if paused.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }
                if running.load(Ordering::SeqCst) {
                    let _ = callback.lock().map(|mut cb| cb());
                }
                thread::sleep(Duration::from_millis(delay));
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
    /// Pause the timer (D3 parity)
    pub fn pause(&mut self) {
        self.paused.store(true, Ordering::SeqCst);
    }
    /// Resume the timer (D3 parity)
    pub fn resume(&mut self) {
        self.paused.store(false, Ordering::SeqCst);
    }
    /// Returns true if the timer is paused
    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
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
    /// Example: Pause and resume a timer
    /// ```rust
    /// use rust_d3::timer::Timer;
    /// let mut t = Timer::new(|| println!("tick"), 10);
    /// t.start();
    /// t.pause();
    /// // ...
    /// t.resume();
    /// t.stop();
    /// ```
    pub fn restart(&mut self) {
        self.stop();
        self.start();
    }
    pub fn delay(&mut self, delay_ms: u64) {
        self.delay = delay_ms;
        // If running, restart the timer to apply new delay immediately (D3 parity)
        if self.is_running() {
            self.stop();
            self.start();
        }
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
    /// Returns the next scheduled tick time (if running)
    pub fn next_tick(&self) -> Option<std::time::Instant> {
        if self.is_running() {
            Some(std::time::Instant::now() + std::time::Duration::from_millis(self.delay))
        } else {
            None
        }
    }
}

// #[cfg(feature = "async-timer")]
pub struct AsyncTimer {
    pub running: Arc<AtomicBool>,
    pub handle: Option<tokio::task::JoinHandle<()>>,
    pub id: usize,
}

// #[cfg(feature = "async-timer")]
impl AsyncTimer {
    pub fn new_async<F, Fut>(mut callback: F, delay_ms: u64) -> Self
    where
        F: FnMut() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let mut id_lock = TIMER_ID.lock().unwrap();
        let id = *id_lock;
        *id_lock += 1;
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        let handle = tokio::spawn(async move {
            while running_clone.load(Ordering::SeqCst) {
                tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                callback().await;
            }
            GLOBAL_TIMERS.lock().unwrap().remove(&id);
        });
        AsyncTimer {
            running,
            handle: Some(handle),
            id,
        }
    }
    pub async fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.await;
        }
        GLOBAL_TIMERS.lock().unwrap().remove(&self.id);
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
    pub async fn schedule_async<F, Fut>(callback: F, delay_ms: u64)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
            callback().await;
        });
    }
}
