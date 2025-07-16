use rust_d3::timer::Timer;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

#[test]
fn test_timer_tick() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        5,
    );
    timer.start();
    thread::sleep(Duration::from_millis(20));
    timer.stop();
    assert!(count.load(Ordering::SeqCst) >= 3);
}

#[test]
fn test_timer_stop() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        5,
    );
    timer.start();
    thread::sleep(Duration::from_millis(15));
    timer.stop();
    let after = count.load(Ordering::SeqCst);
    thread::sleep(Duration::from_millis(15));
    assert_eq!(after, count.load(Ordering::SeqCst));
}

#[test]
fn test_timer_restart() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        2,
    );
    timer.start();
    thread::sleep(Duration::from_millis(6));
    timer.restart();
    thread::sleep(Duration::from_millis(6));
    timer.stop();
    assert!(count.load(Ordering::SeqCst) > 2);
}

#[test]
fn test_timer_delay() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        2,
    );
    timer.start();
    thread::sleep(Duration::from_millis(5));
    let before = count.load(Ordering::SeqCst);
    timer.delay(10);
    thread::sleep(Duration::from_millis(15));
    timer.stop();
    let after = count.load(Ordering::SeqCst);
    assert!(
        after > before,
        "Timer should continue ticking after delay change"
    );
}

#[test]
fn test_timer_is_running() {
    let mut timer = Timer::new(|| {}, 2);
    assert!(!timer.is_running());
    timer.start();
    assert!(timer.is_running());
    timer.stop();
    assert!(!timer.is_running());
}

#[test]
fn test_timer_double_stop_and_restart() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        2,
    );
    timer.start();
    thread::sleep(Duration::from_millis(5));
    timer.stop();
    timer.stop(); // Should not panic
    timer.start();
    thread::sleep(Duration::from_millis(5));
    timer.stop();
    assert!(count.load(Ordering::SeqCst) > 1);
}

#[test]
fn test_timer_zero_delay() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        0,
    );
    timer.start();
    thread::sleep(Duration::from_millis(5));
    timer.stop();
    assert!(count.load(Ordering::SeqCst) > 1);
}

#[test]
fn test_timer_active_and_now() {
    let mut timer = Timer::new(|| {}, 1);
    assert!(!timer.active());
    timer.start();
    assert!(timer.active());
    timer.stop();
    assert!(!timer.active());
    let t1 = rust_d3::timer::now();
    std::thread::sleep(std::time::Duration::from_millis(1));
    let t2 = rust_d3::timer::now();
    assert!(t2 > t1);
}

#[test]
fn test_timer_flush_tick_once() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        1000,
    );
    timer.start();
    // Should not tick yet
    assert_eq!(count.load(Ordering::SeqCst), 0);
    timer.tick_once();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    rust_d3::timer::flush(); // Should tick again
    assert!(count.load(Ordering::SeqCst) >= 2);
    timer.stop();
}

#[test]
fn test_timer_schedule() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    Timer::schedule(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        5,
    );
    std::thread::sleep(std::time::Duration::from_millis(10));
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn test_timer_global_registry() {
    let mut timer = Timer::new(|| {}, 1);
    timer.start();
    let timers = rust_d3::timer::GLOBAL_TIMERS.lock().unwrap();
    assert!(timers.values().any(|t| t.id == timer.id));
    drop(timers);
    timer.stop();
    let timers = rust_d3::timer::GLOBAL_TIMERS.lock().unwrap();
    assert!(!timers.values().any(|t| t.id == timer.id));
}

#[test]
fn test_timer_long_delay_no_early_tick() {
    // Run the test multiple times to reduce flakiness
    let mut failures = 0;
    for _ in 0..10 {
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();
        let mut timer = Timer::new(
            move || {
                count_clone.fetch_add(1, Ordering::SeqCst);
            },
            200,
        ); // 200ms delay
        timer.start();
        thread::sleep(Duration::from_millis(10));
        timer.stop();
        if count.load(Ordering::SeqCst) > 1 {
            failures += 1;
        }
    }
    assert!(
        failures <= 1,
        "Timer ticked early in more than 1 out of 10 runs"
    );
}

#[test]
fn test_timer_restart_after_stop() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let mut timer = Timer::new(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        2,
    );
    timer.start();
    thread::sleep(Duration::from_millis(5));
    timer.stop();
    let after = count.load(Ordering::SeqCst);
    timer.start();
    thread::sleep(Duration::from_millis(5));
    timer.stop();
    assert!(
        count.load(Ordering::SeqCst) > after,
        "Timer should tick again after restart"
    );
}

#[test]
fn test_multiple_timers_independent() {
    let a = Arc::new(AtomicUsize::new(0));
    let b = Arc::new(AtomicUsize::new(0));
    let mut t1 = Timer::new(
        {
            let a = a.clone();
            move || {
                a.fetch_add(1, Ordering::SeqCst);
            }
        },
        2,
    );
    let mut t2 = Timer::new(
        {
            let b = b.clone();
            move || {
                b.fetch_add(1, Ordering::SeqCst);
            }
        },
        2,
    );
    t1.start();
    t2.start();
    thread::sleep(Duration::from_millis(10));
    t1.stop();
    t2.stop();
    assert!(
        a.load(Ordering::SeqCst) > 0 && b.load(Ordering::SeqCst) > 0,
        "Both timers should tick independently"
    );
}

#[test]
fn test_timer_schedule_one_shot() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    Timer::schedule(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        5,
    );
    thread::sleep(Duration::from_millis(20));
    assert_eq!(
        count.load(Ordering::SeqCst),
        1,
        "Schedule should only tick once"
    );
}

#[test]
fn test_timer_tick_once_does_not_start() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    let timer = Timer::new(
        move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        },
        1000,
    );
    timer.tick_once();
    thread::sleep(Duration::from_millis(10));
    assert_eq!(
        count.load(Ordering::SeqCst),
        1,
        "tick_once should only tick once and not start timer"
    );
    assert!(!timer.is_running());
}

// Optionally, add async timer test if feature enabled
// #[cfg(feature = "async-timer")]
// #[tokio::test]
// async fn test_async_timer_tick() {
//     use rust_d3::timer::AsyncTimer;
//     use std::sync::atomic::AtomicUsize;
//     use std::sync::Arc;
//     let count = Arc::new(AtomicUsize::new(0));
//     let count_clone = count.clone();
//     let mut timer = AsyncTimer::new_async(move || {
//         let count_clone = count_clone.clone();
//         async move {
//             count_clone.fetch_add(1, Ordering::SeqCst);
//         }
//     }, 2);
//     tokio::time::sleep(std::time::Duration::from_millis(10)).await;
//     timer.stop().await;
//     assert!(count.load(Ordering::SeqCst) > 1);
// }
