use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use tokio::time::sleep;

// Note: This API is generic and not bound to DOM/SVG nodes. For DOM integration, use a wrapper type implementing Updatable.
// Real DOM event integration is not supported in Rust.

pub trait Updatable: Send + Sync + Clone {
    fn set_attr(&self, name: &str, value: &str);
    fn set_style(&self, name: &str, value: &str);
}
pub trait TextSet: Updatable {
    fn set_text(&self, value: &str);
}

pub struct Tween<T: Updatable> {
    pub name: String,
    pub func: Box<dyn Fn(f64, &T) + Send + Sync>,
}

pub struct Transition<T: Updatable> {
    pub targets: Vec<T>,
    pub duration: u64, // milliseconds
    pub delay: u64,
    pub ease: Option<Box<dyn Fn(f64) -> f64 + Send + Sync>>,
    pub events: HashMap<String, Vec<Box<dyn Fn() + Send + Sync>>>,
    pub styles: HashMap<String, String>,
    pub attrs: HashMap<String, String>,
    pub tweens: HashMap<String, Tween<T>>,
    pub removed: bool,
    pub interrupted: bool,
    pub state: Arc<Mutex<TransitionState>>,
    pub pause_flag: Arc<AtomicBool>,
    pub cancel_flag: Arc<AtomicBool>,
    pub finished_flag: Arc<AtomicBool>,
}

impl<T: Updatable> Clone for Tween<T> {
    fn clone(&self) -> Self {
        Tween {
            name: self.name.clone(),
            func: Box::new(|_, _| {}), // cannot clone closure, stub
        }
    }
}

impl<T: Updatable> Clone for Transition<T> {
    fn clone(&self) -> Self {
        Self {
            targets: self.targets.clone(),
            duration: self.duration,
            delay: self.delay,
            ease: None, // cannot clone closure
            events: HashMap::new(), // cannot clone event listeners
            styles: self.styles.clone(),
            attrs: self.attrs.clone(),
            tweens: self.tweens.clone(),
            removed: self.removed,
            interrupted: self.interrupted,
            state: Arc::new(Mutex::new(TransitionState::Idle)),
            pause_flag: self.pause_flag.clone(),
            cancel_flag: self.cancel_flag.clone(),
            finished_flag: self.finished_flag.clone(),
        }
    }
}

// Note: Due to Rust's type system, closures and event listeners cannot be cloned.
// Cloned transitions will not retain event listeners or tween logic.
// Only use .clone() for stateless transitions.

impl<T: Updatable> Transition<T> {
    pub fn new(targets: Vec<T>, duration: u64) -> Self {
        Self {
            targets,
            duration,
            delay: 0,
            ease: None,
            events: HashMap::new(),
            styles: HashMap::new(),
            attrs: HashMap::new(),
            tweens: HashMap::new(),
            removed: false,
            interrupted: false,
            state: Arc::new(Mutex::new(TransitionState::Idle)),
            pause_flag: Arc::new(AtomicBool::new(false)),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            finished_flag: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn delay(&mut self, ms: u64) -> &mut Self {
        self.delay = ms;
        self
    }
    pub fn ease<F>(&mut self, easing: F) -> &mut Self
    where
        F: Fn(f64) -> f64 + Send + Sync + 'static,
    {
        self.ease = Some(Box::new(easing));
        self
    }
    pub fn ease_builtin(&mut self, easing: fn(f64) -> f64) -> &mut Self {
        self.ease = Some(Box::new(easing));
        self
    }
    pub fn on<F>(&mut self, event: &str, listener: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.events.entry(event.to_string()).or_default().push(Box::new(listener));
        self
    }
    pub fn remove(&mut self) -> &mut Self {
        self.removed = true;
        self
    }
    pub fn style(&mut self, name: &str, value: &str) -> &mut Self {
        self.styles.insert(name.to_string(), value.to_string());
        self
    }
    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        self.attrs.insert(name.to_string(), value.to_string());
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self
    where
        T: TextSet,
    {
        for t in &self.targets {
            t.set_text(value);
        }
        self
    }
    pub fn attr_tween<F>(&mut self, name: &str, tween: F) -> &mut Self
    where
        F: Fn(f64, &T) + Send + Sync + 'static,
    {
        self.tweens.insert(name.to_string(), Tween { name: name.to_string(), func: Box::new(tween) });
        self
    }
    pub fn style_tween<F>(&mut self, name: &str, tween: F) -> &mut Self
    where
        F: Fn(f64, &T) + Send + Sync + 'static,
    {
        self.tweens.insert(name.to_string(), Tween { name: name.to_string(), func: Box::new(tween) });
        self
    }
    pub fn tween<F>(&mut self, name: &str, tween: F) -> &mut Self
    where
        F: Fn(f64, &T) + Send + Sync + 'static,
    {
        self.tweens.insert(name.to_string(), Tween { name: name.to_string(), func: Box::new(tween) });
        self
    }
    pub async fn run(&self) {
        {
            let mut state = self.state.lock().unwrap();
            *state = TransitionState::Running;
        }
        if let Some(listeners) = self.events.get("start") {
            for f in listeners { f(); }
        }
        if self.delay > 0 {
            sleep(Duration::from_millis(self.delay)).await;
        }
        let steps = (self.duration as usize / 16).max(1);
        for i in 0..=steps {
            if self.cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                let mut state = self.state.lock().unwrap();
                *state = TransitionState::Cancelled;
                break;
            }
            while self.pause_flag.load(std::sync::atomic::Ordering::SeqCst) {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            let t = i as f64 / steps as f64;
            let eased = if let Some(ref ease_fn) = self.ease {
                ease_fn(t)
            } else {
                t
            };
            for target in &self.targets {
                for tween in self.tweens.values() {
                    (tween.func)(eased, target);
                }
                for (name, value) in &self.attrs {
                    target.set_attr(name, value);
                }
                for (name, value) in &self.styles {
                    target.set_style(name, value);
                }
            }
            sleep(Duration::from_millis(16)).await;
        }
        {
            let mut state = self.state.lock().unwrap();
            if *state != TransitionState::Cancelled {
                *state = TransitionState::Finished;
                self.finished_flag.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
        if let Some(listeners) = self.events.get("end") {
            for f in listeners { f(); }
        }
    }
    pub fn pause(&self) {
        self.pause_flag.store(true, std::sync::atomic::Ordering::SeqCst);
        let mut state = self.state.lock().unwrap();
        *state = TransitionState::Paused;
    }
    pub fn resume(&self) {
        self.pause_flag.store(false, std::sync::atomic::Ordering::SeqCst);
        let mut state = self.state.lock().unwrap();
        *state = TransitionState::Running;
    }
    pub fn cancel(&self) {
        self.cancel_flag.store(true, std::sync::atomic::Ordering::SeqCst);
        let mut state = self.state.lock().unwrap();
        *state = TransitionState::Cancelled;
    }
    pub fn restart(&self) {
        self.cancel_flag.store(false, std::sync::atomic::Ordering::SeqCst);
        self.pause_flag.store(false, std::sync::atomic::Ordering::SeqCst);
        self.finished_flag.store(false, std::sync::atomic::Ordering::SeqCst);
        let mut state = self.state.lock().unwrap();
        *state = TransitionState::Running;
    }
    pub fn finished(&self) -> bool {
        self.finished_flag.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub async fn end(&self) {
        // Wait for transition to finish
        while !self.finished() && *self.state.lock().unwrap() != TransitionState::Cancelled {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
    pub fn interrupt(&mut self) {
        self.interrupted = true;
        if let Some(listeners) = self.events.get("interrupt") {
            for f in listeners { f(); }
        }
    }
    pub fn select<F>(&self, f: F) -> Transition<T>
    where
        F: Fn(&T) -> bool,
        T: Clone,
    {
        let targets = self.targets.iter().cloned().filter(|t| f(t)).collect();
        Transition { targets, ..self.clone() }
    }
    pub fn select_all<F>(&self, f: F) -> Transition<T>
    where
        F: Fn(&T) -> bool,
        T: Clone,
    {
        let targets = self.targets.iter().cloned().filter(|t| f(t)).collect();
        Transition { targets, ..self.clone() }
    }
    pub fn merge(&self, other: &Transition<T>) -> Transition<T>
    where
        T: Clone,
    {
        let mut targets = self.targets.clone();
        targets.extend(other.targets.iter().cloned());
        Transition { targets, ..self.clone() }
    }
    pub fn filter<F>(&self, f: F) -> Transition<T>
    where
        F: Fn(&T) -> bool,
        T: Clone,
    {
        let targets = self.targets.iter().cloned().filter(|t| f(t)).collect();
        Transition { targets, ..self.clone() }
    }
    pub fn each<F>(&self, mut f: F) -> &Self
    where
        F: FnMut(&T),
    {
        for t in &self.targets {
            f(t);
        }
        self
    }
    pub fn call<F>(&self, f: F) -> &Self
    where
        F: Fn(&Self),
    {
        f(self);
        self
    }
    pub fn nodes(&self) -> Vec<T> where T: Clone {
        self.targets.clone()
    }
    pub fn empty(&self) -> bool {
        self.targets.is_empty()
    }
    pub fn size(&self) -> usize {
        self.targets.len()
    }
}

pub mod easing {
    pub fn linear(t: f64) -> f64 { t }
    pub fn quad_in(t: f64) -> f64 { t * t }
    pub fn quad_out(t: f64) -> f64 { t * (2.0 - t) }
    pub fn quad_in_out(t: f64) -> f64 {
        if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t }
    }
    pub fn cubic_in(t: f64) -> f64 { t * t * t }
    pub fn cubic_out(t: f64) -> f64 { let t1 = t - 1.0; t1 * t1 * t1 + 1.0 }
    pub fn cubic_in_out(t: f64) -> f64 {
        if t < 0.5 { 4.0 * t * t * t } else { (t - 1.0) * (2.0 * t - 2.0).powi(2) + 1.0 }
    }
    // Add more as needed
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransitionState {
    Idle,
    Running,
    Paused,
    Finished,
    Cancelled,
}
