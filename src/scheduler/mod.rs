mod internal_requests;

use crate::shared::time::local_boise_time;
use tokio_schedule::{every, Job};

pub async fn start_daily_scheduler(base_url: &str) {
    let local = local_boise_time().timezone();

    let daily = every(1)
        .day()
        .at(9, 0, 0)
        .in_timezone(&local)
        .perform(|| async {
            println!("starting daily task");
            match internal_requests::trigger_remind_missing_bb(base_url).await {
                Ok(_) => {
                    println!("after trigger daily");
                }
                Err(err) => println!("err with daily: {:?}", err),
            }
        });
    daily.await;
}

pub async fn start_leaderboard_scheduler(base_url: &str) {
    let local = local_boise_time().timezone();
    let daily = every(3)
        .hours()
        .at(0, 0)
        .in_timezone(&local)
        .perform(|| async {
            match internal_requests::trigger_leaderboard_stats(base_url).await {
                Ok(_) => println!("after leaderboard trigger"),
                Err(err) => println!("err with leaderboard: {:?}", err),
            }
        });
    daily.await;
}
