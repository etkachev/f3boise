use crate::app_state::MutableAppState;
use crate::slack_api::users::users_list::response::SlackUserData;
use crate::users::f3_user::F3User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamJoinData {
    pub user: SlackUserData,
}

pub fn handle_new_user(user: &SlackUserData, app_state: &MutableAppState) {
    let mapped_user = F3User::from(user);
    {
        let mut app = app_state.app.lock().unwrap();
        app.add_user(user.id.as_str(), mapped_user);
    }
}
