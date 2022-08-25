use crate::app_state::backblast_data::BACK_BLAST_TAG;
use crate::app_state::parse_backblast::parse_back_blast;
use crate::app_state::AppState;
use crate::web_api_routes::slack_events::event_times::EventTimes;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ChannelMessageEvent {
    pub channel: String,
    pub user: String,
    pub text: String,
    pub ts: String,
    pub event_ts: String,
    // TODO update type
    pub channel_type: String,
}

pub async fn handle_channel_message(event: &ChannelMessageEvent, app_state: &AppState) {
    let (first_line, _) = event.text.split_once('\n').unwrap_or_else(|| ("", ""));
    match first_line.to_lowercase() {
        // is back-blast
        line if line.starts_with(BACK_BLAST_TAG) => {
            let mut data = parse_back_blast(event.text.as_str(), &app_state.users);
            data.set_event_times(EventTimes::new(
                event.ts.to_string(),
                event.event_ts.to_string(),
            ));
            println!("Backblast: {:?}", data);
            println!("All Pax: {:?}", data.get_pax());
            if data.is_valid_back_blast() {
                app_state.back_blast_verified(true, &data).await;
            } else {
                app_state.back_blast_verified(false, &data).await;
            }
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_format() {
        use serde::Deserialize;
        use serde_with::{formats::Flexible, TimestampSeconds};

        #[serde_with::serde_as]
        #[derive(Deserialize, Debug)]
        struct S {
            #[serde_as(as = "TimestampSeconds<String, Flexible>")]
            time: DateTime<Utc>,
        }

        let my_s = serde_json::from_str::<S>(r#"{ "time": 1355517523.000005 }"#).unwrap();
        println!("{:?}", my_s)
    }
}
