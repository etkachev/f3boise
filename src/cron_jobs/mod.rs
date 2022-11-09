use crate::shared::time::local_boise_time;
use chrono::{DateTime, FixedOffset, Timelike};
use tokio::time;

/// 8am local time
const TIME_OF_DAY: u32 = 8;
/// every 24 hours
const FREQUENCY_IN_SECONDS: u64 = 60 * 60 * 24;
const IS_LOCAL: bool = true;

pub async fn start_cron() {
    let local_now = local_boise_time();
    // if already passed schedule time, then run process right away?
    if local_now.time().hour() > TIME_OF_DAY {
        // let start = time::Instant::now() + time::Duration::from_secs(1);
    }

    let tomorrow = if IS_LOCAL {
        get_dev_time()
    } else {
        get_tomorrow_at_scheduled_time()
    };
    println!("tomorrow: {:?}", tomorrow);
    let duration_between = tomorrow.signed_duration_since(local_now);

    let time_duration = time::Duration::from_secs(duration_between.num_seconds() as u64);
    let start = time::Instant::now() + time_duration;
    let frequency = if IS_LOCAL { 2 } else { FREQUENCY_IN_SECONDS };

    let mut interval = time::interval_at(start, time::Duration::from_secs(frequency));

    loop {
        interval.tick().await;
        println!("Do stuff here!");
    }
}

fn get_tomorrow_at_scheduled_time() -> DateTime<FixedOffset> {
    let tomorrow = local_boise_time()
        .checked_add_signed(chrono::Duration::days(1))
        .unwrap();
    let tomorrow = tomorrow.with_hour(TIME_OF_DAY).unwrap();
    let tomorrow = tomorrow.with_minute(0).unwrap();
    tomorrow.with_second(0).unwrap()
}

fn get_dev_time() -> DateTime<FixedOffset> {
    local_boise_time()
        .checked_add_signed(chrono::Duration::seconds(5))
        .unwrap()
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn first_time_test() {
        assert_eq!(1, 1);
    }
}
