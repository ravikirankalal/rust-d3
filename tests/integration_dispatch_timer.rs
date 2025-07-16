//! Integration tests for dispatch and timer modules

use rust_d3::dispatch::Dispatch;
use rust_d3::timer::Timer;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::test]
async fn integration_dispatch_timer_event() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let dispatcher = Dispatch::new();
    dispatcher
        .on("tick", move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        })
        .await;
    let dispatcher = Arc::new(Mutex::new(dispatcher));
    let dispatcher_clone = dispatcher.clone();
    let handle = tokio::spawn(async move {
        let mut timer = Timer::new(
            move || {
                let dispatcher_clone = dispatcher_clone.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let d = dispatcher_clone.lock().await;
                    d.call("tick").await;
                    let _ = tx.send(());
                });
                let _ = rx.recv();
            },
            5,
        );
        timer.start();
        tokio::time::sleep(Duration::from_millis(25)).await;
        timer.stop();
    });
    handle.await.unwrap();
    assert!(count.load(Ordering::SeqCst) >= 3);
}

#[tokio::test]
async fn integration_timer_pause_resume_dispatch() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let dispatcher = Dispatch::new();
    dispatcher
        .on("tick", move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        })
        .await;
    let dispatcher = Arc::new(Mutex::new(dispatcher));
    let dispatcher_clone = dispatcher.clone();
    let handle = tokio::spawn(async move {
        let mut timer = Timer::new(
            move || {
                let dispatcher_clone = dispatcher_clone.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let d = dispatcher_clone.lock().await;
                    d.call("tick").await;
                    let _ = tx.send(());
                });
                let _ = rx.recv();
            },
            5,
        ); // Use 5ms interval for more robust timing
        timer.start();
        tokio::time::sleep(Duration::from_millis(50)).await; // Give more time for ticks
        timer.pause();
        tokio::time::sleep(Duration::from_millis(10)).await; // Allow in-flight ticks to complete
        let paused_count = count.load(Ordering::SeqCst);
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert_eq!(paused_count, count.load(Ordering::SeqCst));
        timer.resume();
        tokio::time::sleep(Duration::from_millis(50)).await;
        timer.stop();
        assert!(count.load(Ordering::SeqCst) > paused_count);
    });
    handle.await.unwrap();
}

#[tokio::test]
async fn integration_dispatch_remove_handler_mid_tick() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let dispatcher = Dispatch::new();
    let handle = dispatcher
        .on_with_handle("tick", move |_| {
            count_clone.fetch_add(1, Ordering::SeqCst);
        })
        .await;
    let dispatcher = Arc::new(Mutex::new(dispatcher));
    let dispatcher_clone = dispatcher.clone();
    let handle_tokio = tokio::spawn(async move {
        let handle_clone = handle.clone();
        let mut timer = Timer::new(
            move || {
                let dispatcher_clone = dispatcher_clone.clone();
                let handle_clone = handle_clone.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let d = dispatcher_clone.lock().await;
                    d.call("tick").await;
                    d.off_handle("tick", &handle_clone).await;
                    let _ = tx.send(());
                });
                let _ = rx.recv();
            },
            5,
        );
        timer.start();
        tokio::time::sleep(Duration::from_millis(20)).await;
        timer.stop();
    });
    handle_tokio.await.unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn integration_dispatch_event_bubbling_sim() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let log_parent = log.clone();
    let log_child = log.clone();
    let parent = Arc::new(Dispatch::new());
    let child = Dispatch::new();
    parent
        .on("custom", move || {
            let log_parent = log_parent.clone();
            let fut = async move {
                log_parent.lock().await.push("parent".to_string());
            };
            tokio::spawn(fut);
        })
        .await;
    // Handler that can optionally stop propagation
    let bubbling_handler = Arc::new({
        let log_child = log_child.clone();
        let parent = parent.clone();
        move |evt: Arc<rust_d3::dispatch::Event<'static>>| {
            let log_child = log_child.clone();
            let parent = parent.clone();
            async move {
                let fut = async move {
                    log_child.lock().await.push("child".to_string());
                };
                tokio::spawn(fut);
                if evt.event_type == "custom-stop" {
                    evt.stop_propagation();
                }
                if !evt.is_propagation_stopped() {
                    parent.call_event("custom", evt.clone()).await;
                }
            }
        }
    });
    child
        .on_with_handle("custom", {
            let bubbling_handler = bubbling_handler.clone();
            move |evt: &rust_d3::dispatch::Event| {
                let evt_static = rust_d3::dispatch::Event {
                    event_type: std::borrow::Cow::Owned(evt.event_type.to_string()),
                    data: evt.data.clone(),
                    timestamp: evt.timestamp,
                    source: evt
                        .source
                        .as_ref()
                        .map(|s| std::borrow::Cow::Owned(s.to_string())),
                    propagation_stopped: evt.propagation_stopped.clone(),
                    default_prevented: evt.default_prevented.clone(),
                };
                let evt = Arc::new(evt_static);
                tokio::spawn(bubbling_handler(evt));
            }
        })
        .await;
    child
        .on_with_handle("custom-stop", {
            let bubbling_handler = bubbling_handler.clone();
            move |evt: &rust_d3::dispatch::Event| {
                let evt_static = rust_d3::dispatch::Event {
                    event_type: std::borrow::Cow::Owned(evt.event_type.to_string()),
                    data: evt.data.clone(),
                    timestamp: evt.timestamp,
                    source: evt
                        .source
                        .as_ref()
                        .map(|s| std::borrow::Cow::Owned(s.to_string())),
                    propagation_stopped: evt.propagation_stopped.clone(),
                    default_prevented: evt.default_prevented.clone(),
                };
                let evt = Arc::new(evt_static);
                tokio::spawn(bubbling_handler(evt));
            }
        })
        .await;
    // Normal bubbling
    let evt = Arc::new(rust_d3::dispatch::Event {
        event_type: "custom".into(),
        data: None,
        timestamp: std::time::Instant::now(),
        source: None,
        propagation_stopped: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        default_prevented: Arc::new(std::sync::atomic::AtomicBool::new(false)),
    });
    child.call_event("custom", evt.clone()).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    let result = log.lock().await.clone();
    assert_eq!(result, vec!["child", "parent"]);
    // Bubbling stopped
    log.lock().await.clear();
    let evt2 = Arc::new(rust_d3::dispatch::Event {
        event_type: "custom-stop".into(),
        data: None,
        timestamp: std::time::Instant::now(),
        source: None,
        propagation_stopped: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        default_prevented: Arc::new(std::sync::atomic::AtomicBool::new(false)),
    });
    child.call_event("custom-stop", evt2.clone()).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    let result2 = log.lock().await.clone();
    assert_eq!(result2, vec!["child"]);
}

#[test]
fn integration_timer_next_tick_query() {
    let mut timer = Timer::new(|| {}, 50);
    assert!(timer.next_tick().is_none());
    timer.start();
    let next = timer.next_tick();
    assert!(next.is_some());
    let now = std::time::Instant::now();
    assert!(next.unwrap() > now);
    timer.stop();
}

#[tokio::test]
async fn integration_dispatch_async_event_bubbling() {
    use rust_d3::dispatch::Dispatch;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::time::Instant;
    use tokio::sync::Mutex as TokioMutex;
    let log = Arc::new(TokioMutex::new(Vec::new()));
    let log_parent = log.clone();
    let log_child = log.clone();
    let parent = Dispatch::new();
    let child = Dispatch::new();
    parent
        .on_async("custom", move |_evt| {
            let log_parent = log_parent.clone();
            Box::pin(async move {
                log_parent.lock().await.push("parent".to_string());
            })
        })
        .await;
    let parent_arc = Arc::new(parent);
    let bubbling_handler = {
        let log_child = log_child.clone();
        let parent_arc = parent_arc.clone();
        move |evt: Arc<rust_d3::dispatch::Event<'static>>| {
            let log_child = log_child.clone();
            let parent_arc = parent_arc.clone();
            async move {
                log_child.lock().await.push("child".to_string());
                if evt.event_type == "custom-stop" {
                    evt.stop_propagation();
                }
                parent_arc
                    .call_event_async_bubble(&parent_arc, "custom", evt.clone())
                    .await;
            }
        }
    };
    child.on_async("custom", bubbling_handler.clone()).await;
    child.on_async("custom-stop", bubbling_handler).await;
    let child_arc = Arc::new(child);
    // Normal bubbling
    let evt = Arc::new(rust_d3::dispatch::Event {
        event_type: "custom".into(),
        data: None,
        timestamp: Instant::now(),
        source: None,
        propagation_stopped: Arc::new(AtomicBool::new(false)),
        default_prevented: Arc::new(AtomicBool::new(false)),
    });
    child_arc.call_async_with("custom", evt.clone()).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    let result = log.lock().await.clone();
    assert_eq!(result, vec!["child", "parent"]);
    // Bubbling stopped
    log.lock().await.clear();
    let evt2 = Arc::new(rust_d3::dispatch::Event {
        event_type: "custom-stop".into(),
        data: None,
        timestamp: Instant::now(),
        source: None,
        propagation_stopped: Arc::new(AtomicBool::new(false)),
        default_prevented: Arc::new(AtomicBool::new(false)),
    });
    child_arc.call_async_with("custom-stop", evt2.clone()).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    let result2 = log.lock().await.clone();
    assert_eq!(result2, vec!["child"]);
}

#[tokio::test]
async fn integration_multiple_timers_shared_dispatch() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone1 = count.clone();
    let _count_clone2 = count.clone(); // Silence unused variable warning
    let dispatcher = Dispatch::new();
    dispatcher
        .on("tick", move || {
            count_clone1.fetch_add(1, Ordering::SeqCst);
        })
        .await;
    let dispatcher = Arc::new(Mutex::new(dispatcher));
    let dispatcher_clone1 = dispatcher.clone();
    let dispatcher_clone2 = dispatcher.clone();
    let handle1 = tokio::spawn(async move {
        let mut timer = Timer::new(
            move || {
                let dispatcher_clone = dispatcher_clone1.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let d = dispatcher_clone.lock().await;
                    d.call("tick").await;
                    let _ = tx.send(());
                });
                let _ = rx.recv();
            },
            5,
        );
        timer.start();
        tokio::time::sleep(Duration::from_millis(30)).await;
        timer.stop();
    });
    let handle2 = tokio::spawn(async move {
        let mut timer = Timer::new(
            move || {
                let dispatcher_clone = dispatcher_clone2.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let d = dispatcher_clone.lock().await;
                    d.call("tick").await;
                    let _ = tx.send(());
                });
                let _ = rx.recv();
            },
            7,
        );
        timer.start();
        tokio::time::sleep(Duration::from_millis(30)).await;
        timer.stop();
    });
    handle1.await.unwrap();
    handle2.await.unwrap();
    assert!(count.load(Ordering::SeqCst) >= 6); // Both timers should have fired at least 3 times each
}

#[tokio::test]
async fn integration_timer_dispatch_async_handler() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let dispatcher = Dispatch::new();
    dispatcher
        .on_async("tick", move |_evt| {
            let count_clone = count_clone.clone();
            Box::pin(async move {
                count_clone.fetch_add(1, Ordering::SeqCst);
            })
        })
        .await;
    let dispatcher = Arc::new(Mutex::new(dispatcher));
    let dispatcher_clone = dispatcher.clone();
    let handle = tokio::spawn(async move {
        let mut timer = Timer::new(
            move || {
                let dispatcher_clone = dispatcher_clone.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let d = dispatcher_clone.lock().await;
                    d.call_async("tick").await;
                    let _ = tx.send(());
                });
                let _ = rx.recv();
            },
            5,
        );
        timer.start();
        tokio::time::sleep(Duration::from_millis(60)).await;
        timer.stop();
    });
    handle.await.unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;
    let final_count = count.load(Ordering::SeqCst);
    assert!(final_count >= 1);
}
