//! D3 Fetch Advanced module
//! Advanced fetch utilities, e.g., streaming, retry, etc.

use std::{thread, time::Duration};

/// Attempts to fetch data using the provided closure, retrying with exponential backoff on failure.
pub fn fetch_with_retry<F, T, E>(mut fetch_fn: F, max_retries: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut retries = 0;
    let mut delay = 100;
    loop {
        match fetch_fn() {
            Ok(val) => return Ok(val),
            Err(_) if retries < max_retries => {
                thread::sleep(Duration::from_millis(delay));
                delay *= 2;
                retries += 1;
            }
            Err(e) => return Err(e),
        }
    }
}
