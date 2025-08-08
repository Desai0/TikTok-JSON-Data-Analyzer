// date_utils.rs
// date_utils.rs is in charge of :
// - providing functions that help calculate the time (in days) between two dates
// - these functions are used in lib.rs - for instance they're used when we need to calculate likes/day or videos/day...

use chrono::{NaiveDateTime, TimeZone};

pub fn date_to_unix_timestamp(date_str: &str) -> Option<i64> {
    let date_time = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").ok()?;
    let timestamp = chrono::Utc
        .timestamp_millis_opt(date_time.and_utc().timestamp_millis())
        .unwrap()
        .timestamp();
    Some(timestamp)
}

pub fn days_between(reference_timestamp: i64, date_str: &str) -> Option<usize> {
    let past_timestamp = match date_to_unix_timestamp(date_str) {
        Some(ts) => ts,
        None => return None,
    };

    if reference_timestamp < past_timestamp {
        return Some(0);
    }

    let duration_secs = (reference_timestamp - past_timestamp) as u64;
    let days = duration_secs / (24 * 60 * 60);
    Some(days as usize)
}
