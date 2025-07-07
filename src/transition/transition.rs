use std::collections::HashMap;

pub struct Transition {
    pub duration: u64, // milliseconds
    pub delay: u64,
    pub ease: Option<Box<dyn Fn(f64) -> f64>>,
    pub events: HashMap<String, Vec<Box<dyn Fn()>>>,
    pub styles: HashMap<String, String>,
    pub attrs: HashMap<String, String>,
    pub removed: bool,
}

impl Transition {
    pub fn new(duration: u64) -> Self {
        Self {
            duration,
            delay: 0,
            ease: None,
            events: HashMap::new(),
            styles: HashMap::new(),
            attrs: HashMap::new(),
            removed: false,
        }
    }

    pub fn interpolate<F>(&self, from: f64, to: f64, mut on_update: F)
    where
        F: FnMut(f64),
    {
        let steps = self.duration as usize / 16; // ~60fps
        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let value = if let Some(ref ease_fn) = self.ease {
                from + (to - from) * ease_fn(t)
            } else {
                from + (to - from) * t
            };
            on_update(value);
        }
    }

    pub fn delay(&mut self, ms: u64) -> &mut Self {
        self.delay = ms;
        self
    }
    pub fn ease<F>(&mut self, easing: F) -> &mut Self
    where
        F: Fn(f64) -> f64 + 'static,
    {
        self.ease = Some(Box::new(easing));
        self
    }
    pub fn on<F>(&mut self, event: &str, listener: F) -> &mut Self
    where
        F: Fn() + 'static,
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
}
