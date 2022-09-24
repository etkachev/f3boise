use chrono::{DateTime, FixedOffset, TimeZone, Utc};

pub fn local_boise_time() -> DateTime<FixedOffset> {
    let now = Utc::now().naive_utc();
    let hour = 3600;
    FixedOffset::west(6 * hour).from_utc_datetime(&now)
}
