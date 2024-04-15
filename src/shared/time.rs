use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone, Utc};
use serde::Deserialize;

/// Basic date range struct
#[derive(Deserialize)]
pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// Get local Boise time
pub fn local_boise_time() -> DateTime<FixedOffset> {
    let now = Utc::now().naive_utc();
    let hour = 3600;
    FixedOffset::west_opt(6 * hour)
        .unwrap()
        .from_utc_datetime(&now)
}
