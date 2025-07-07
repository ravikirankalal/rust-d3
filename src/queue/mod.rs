//! Minimal d3-queue-like API for legacy compatibility
//! Replaced by async/futures in idiomatic Rust, but provided for D3.js API parity.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// A simple queue for running async tasks in order, similar to d3-queue.
pub struct Queue {
    tasks: Arc<Mutex<Vec<Pin<Box<dyn Future<Output = ()> + Send>>>>>,
    waker: Arc<Mutex<Option<Waker>>>,
    started: Arc<Mutex<bool>>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            tasks: Arc::new(Mutex::new(Vec::new())),
            waker: Arc::new(Mutex::new(None)),
            started: Arc::new(Mutex::new(false)),
        }
    }

    /// Add a task to the queue.
    pub fn defer<Fut>(&self, fut: Fut)
    where
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.tasks.lock().unwrap().push(Box::pin(fut));
        if let Some(waker) = &*self.waker.lock().unwrap() {
            waker.wake_by_ref();
        }
    }
}

impl Future for Queue {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut started = self.started.lock().unwrap();
        let mut tasks = self.tasks.lock().unwrap();
        if !*started && tasks.is_empty() {
            // If never started and no tasks, resolve immediately
            return Poll::Ready(());
        }
        *started = true;
        if tasks.is_empty() {
            *self.waker.lock().unwrap() = Some(cx.waker().clone());
            Poll::Pending
        } else {
            let mut all_done = true;
            let mut i = 0;
            while i < tasks.len() {
                let fut = &mut tasks[i];
                match fut.as_mut().poll(cx) {
                    Poll::Ready(()) => {
                        let _ = tasks.remove(i);
                    }
                    Poll::Pending => {
                        all_done = false;
                        i += 1;
                    }
                }
            }
            if all_done {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }
}

// (All tests have been moved to tests/queue.rs)
