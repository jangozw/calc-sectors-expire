extern crate chrono;

use std::time::{Duration, UNIX_EPOCH};

use chrono::Local;
use chrono::prelude::DateTime;

pub fn now() -> i64 {
    Local::now().timestamp()
}

pub fn to_datetime(stamp: i64) -> String {
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(stamp as u64);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Local>::from(d);
    // Formats the combined date and time with the specified format string.
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}