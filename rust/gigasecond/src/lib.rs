use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    const ONE_BILLION_SECS: i64 = 1_000_000_000;
    start + Duration::seconds(ONE_BILLION_SECS)
}
