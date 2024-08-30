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

/// converts event_ts string to valid utc datetime
pub fn convert_event_ts(event_ts: &str) -> DateTime<Utc> {
    let (secs, nanos) = event_ts.split_once('.').unwrap();
    let secs = secs.parse::<i64>().expect("Invalid seconds");
    // Parse the fractional part as microseconds and convert to nanoseconds
    let micros = nanos.parse::<u32>().expect("Invalid microseconds");
    let nanos = micros * 1000;
    DateTime::from_timestamp(secs, nanos).unwrap()
}

/// converts date time utc to event_ts string
pub fn convert_date_time_to_event_ts(date_time: DateTime<Utc>) -> String {
    let secs = date_time.timestamp();
    let nanos = date_time.timestamp_subsec_nanos();
    // Convert nanoseconds to microseconds
    let micros = nanos / 1000;
    format!("{}.{:06}", secs, micros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_ts_to_datetime() {
        let time_stamp = "1725039792.016800";
        let time = convert_event_ts(time_stamp);
        assert_eq!(
            time,
            DateTime::from_timestamp(1725039792, 016800 * 1000).unwrap()
        );
    }

    #[test]
    fn datetime_to_event_ts() {
        let time = DateTime::from_timestamp(1725039792, 016800 * 1000).unwrap();
        let event_ts = convert_date_time_to_event_ts(time);
        assert_eq!(event_ts, "1725039792.016800".to_string());
    }
}
