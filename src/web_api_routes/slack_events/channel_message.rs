use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{formats::Flexible, TimestampSeconds};

#[serde_with::serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ChannelMessageEvent {
    pub channel: String,
    pub user: String,
    pub text: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub ts: DateTime<Utc>,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub event_ts: DateTime<Utc>,
    // TODO update type
    pub channel_type: String,
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
