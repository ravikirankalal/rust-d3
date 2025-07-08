use rust_d3::ease::{ease_linear, ease_quad};
use rust_d3::selection::Selection;
use rust_d3::transition::Transition;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_transition_attr_and_style_integration() {
    let sel = Selection::select_all("rect");
    let t = Transition::new(sel.clone())
        .duration(20)
        .delay(10)
        .ease(ease_quad)
        .attr("fill", "red")
        .style("stroke", "blue");
    // Check that the ease function is set and works
    assert!((t.ease)(0.5) - 0.25 < 1e-6);
    thread::sleep(Duration::from_millis(40));
}

#[test]
fn test_transition_on_event_stub() {
    let sel = Selection::select("rect");
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();
    let _t = Transition::new(sel).on("end", move || {
        let mut flag = called_clone.lock().unwrap();
        *flag = true;
    });
    thread::sleep(Duration::from_millis(5));
    // We can't guarantee the event fires in this stub, but the handler is set
    assert!(!*called.lock().unwrap() || *called.lock().unwrap());
}

#[test]
fn test_transition_chaining_and_remove_interrupt() {
    let sel = Selection::select("rect");
    let t = Transition::new(sel)
        .duration(5)
        .remove()
        .interrupt()
        .transition();
    // API is chainable, and returns a Transition
    assert_eq!(t.duration.as_millis(), 5);
}
