use serde::{Deserialize, Serialize};

/// General event times struct for knowing when events happen in slack.
#[derive(Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct EventTimes {
    pub ts: String,
    pub event_ts: String,
}

impl EventTimes {
    pub fn new(ts: String, event_ts: String) -> Self {
        EventTimes { ts, event_ts }
    }
}
