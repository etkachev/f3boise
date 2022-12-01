use crate::web_api_routes::interactive_events::interaction_payload::ActionUser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewSubmission {
    /// The user who interacted to trigger this request
    pub user: ActionUser,
}
