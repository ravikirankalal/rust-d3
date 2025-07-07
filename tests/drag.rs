//! Unit test for d3 drag
use rust_d3::drag::{DragEvent, DragState};

#[test]
fn test_drag_lifecycle() {
    let mut drag = DragState::new(0.0, 0.0);
    assert!(!drag.active);
    drag.start();
    assert!(drag.active);
    drag.drag_by(3.0, 4.0);
    assert_eq!(drag.x, 3.0);
    assert_eq!(drag.y, 4.0);
    assert_eq!(drag.dx, 3.0);
    assert_eq!(drag.dy, 4.0);
    drag.drag_by(-1.0, -2.0);
    assert_eq!(drag.x, 2.0);
    assert_eq!(drag.y, 2.0);
    assert_eq!(drag.dx, -1.0);
    assert_eq!(drag.dy, -2.0);
    drag.end();
    assert!(!drag.active);
    assert_eq!(drag.dx, 0.0);
    assert_eq!(drag.dy, 0.0);
}

#[test]
fn test_drag_event_struct() {
    let event = DragEvent {
        event_type: "start",
        x: 1.0,
        y: 2.0,
        dx: 0.0,
        dy: 0.0,
    };
    assert_eq!(event.event_type, "start");
    assert_eq!(event.x, 1.0);
    assert_eq!(event.y, 2.0);
}

#[test]
fn test_drag_on_subject_filter_container_touchable() {
    let drag = DragState::new(0.0, 0.0);
    drag.on("drag", |_e| {});
    drag.subject((1, 2));
    drag.filter(|| true);
    drag.container(vec![1, 2, 3]);
    drag.touchable(|| false);
    // No panic = pass
}
