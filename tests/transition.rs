//! Unit tests for d3 Transition
use rust_d3::transition::Transition;
use rust_d3::Updatable;
use std::sync::{Arc, Mutex};

#[test]
fn test_transition_placeholder() {
    let target = MockTarget::new();
    let _ = Transition::new(vec![target], 0);
}

#[test]
fn test_transition_events_and_tween() {
    let target = MockTarget::new();
    let mut t = Transition::new(vec![target.clone()], 100);
    let started = Arc::new(Mutex::new(false));
    let ended = Arc::new(Mutex::new(false));
    let s1 = started.clone();
    let e1 = ended.clone();
    t.on("start", move || *s1.lock().unwrap() = true);
    t.on("end", move || *e1.lock().unwrap() = true);
    let tween_called = Arc::new(Mutex::new(false));
    let tc = tween_called.clone();
    t.tween("test", move |_t, _target| *tc.lock().unwrap() = true);
    // Simulate event triggers
    if let Some(listeners) = t.events.get("start") {
        for f in listeners { f(); }
    }
    if let Some(listeners) = t.events.get("end") {
        for f in listeners { f(); }
    }
    for tween in t.tweens.values() {
        (tween.func)(0.5, &target);
    }
    assert!(*started.lock().unwrap());
    assert!(*ended.lock().unwrap());
    assert!(*tween_called.lock().unwrap());
}

#[derive(Clone)]
struct MockTarget {
    pub attrs: Arc<Mutex<Vec<(String, String)>>>,
    pub styles: Arc<Mutex<Vec<(String, String)>>>,
}
impl MockTarget {
    fn new() -> Self {
        Self {
            attrs: Arc::new(Mutex::new(vec![])),
            styles: Arc::new(Mutex::new(vec![])),
        }
    }
}
impl Updatable for MockTarget {
    fn set_attr(&self, name: &str, value: &str) {
        self.attrs.lock().unwrap().push((name.to_string(), value.to_string()));
    }
    fn set_style(&self, name: &str, value: &str) {
        self.styles.lock().unwrap().push((name.to_string(), value.to_string()));
    }
}
impl rust_d3::transition::TextSet for MockTarget {
    fn set_text(&self, _value: &str) {
        // No-op for test
    }
}

#[tokio::test]
async fn test_async_transition_attr() {
    let target = MockTarget::new();
    let mut t = Transition::new(vec![target.clone()], 50);
    t.attr("x", "1");
    t.delay(10);
    let started = Arc::new(Mutex::new(false));
    let ended = Arc::new(Mutex::new(false));
    let s1 = started.clone();
    let e1 = ended.clone();
    t.on("start", move || *s1.lock().unwrap() = true);
    t.on("end", move || *e1.lock().unwrap() = true);
    t.run().await;
    assert!(*started.lock().unwrap());
    assert!(*ended.lock().unwrap());
    let attrs = target.attrs.lock().unwrap();
    // Accept either "1" or "1.0" as the value for key "x"
    assert!(attrs.iter().any(|(k, v)| k == "x" && (v == "1" || v == "1.0")));
}

#[tokio::test]
async fn test_transition_tween_and_interrupt() {
    let target = MockTarget::new();
    let mut t = Transition::new(vec![target.clone()], 30);
    let tween_called = Arc::new(Mutex::new(false));
    let tc = tween_called.clone();
    t.tween("custom", move |_t, _target| { *tc.lock().unwrap() = true; });
    t.interrupt();
    assert!(t.interrupted);
    // Simulate running tweens
    for tween in t.tweens.values() {
        (tween.func)(0.5, &target);
    }
    assert!(*tween_called.lock().unwrap());
}

#[test]
fn test_transition_selection_methods() {
    let target = MockTarget::new();
    let t = Transition::new(vec![target.clone()], 10);
    let t2 = t.select(|_| true);
    let _t3 = t.select_all(|_| true);
    let t4 = t.merge(&t2);
    let t5 = t.filter(|_| true);
    assert_eq!(t.size(), 1);
    assert!(!t.empty());
    assert_eq!(t.nodes().len(), 1);
    // Chaining
    t4.each(|_| {});
    t5.call(|_| {});
}

#[test]
fn test_transition_text_and_style_tween() {
    let target = MockTarget::new();
    let mut t = Transition::new(vec![target.clone()], 10);
    t.text("hello");
    t.style_tween("color", |_t, _target| {});
    t.attr_tween("x", |_t, _target| {});
    assert!(t.tweens.contains_key("color"));
    assert!(t.tweens.contains_key("x"));
}

#[tokio::test]
async fn test_transition_pause_resume_cancel_finished_end() {
    let target = MockTarget::new();
    let mut t = Transition::new(vec![target.clone()], 100);
    t.attr("x", "1");
    // Start transition in background
    let t_clone = t.clone();
    let handle = tokio::spawn(async move { t_clone.run().await; });
    // Pause after a short delay
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    t.pause();
    assert_eq!(*t.state.lock().unwrap(), rust_d3::transition::TransitionState::Paused);
    // Resume
    t.resume();
    assert_eq!(*t.state.lock().unwrap(), rust_d3::transition::TransitionState::Running);
    // Cancel
    t.cancel();
    assert_eq!(*t.state.lock().unwrap(), rust_d3::transition::TransitionState::Cancelled);
    // Wait for join
    let _ = handle.await;
    assert!(t.finished() || *t.state.lock().unwrap() == rust_d3::transition::TransitionState::Cancelled);
    // Restart
    t.restart();
    assert_eq!(*t.state.lock().unwrap(), rust_d3::transition::TransitionState::Running);
    // End (should complete quickly)
    let t2 = t.clone();
    let h2 = tokio::spawn(async move { t2.run().await; });
    t.end().await;
    let _ = h2.await;
    assert!(t.finished());
}

#[tokio::test]
async fn test_transition_builtin_easing() {
    let target = MockTarget::new();
    let mut t = Transition::new(vec![target.clone()], 10);
    t.ease_builtin(rust_d3::transition::easing::quad_in);
    t.attr("x", "1");
    t.run().await;
    assert!(t.finished());
}
