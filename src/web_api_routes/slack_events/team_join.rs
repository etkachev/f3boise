use crate::app_state::AppState;
use crate::slack_api::users::users_list::response::SlackUserData;
use crate::users::f3_user::F3User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamJoinData {
    pub user: SlackUserData,
}

pub fn handle_new_user(user: &SlackUserData, app_state: &mut AppState) {
    let mapped_user = F3User::from(user);
    app_state.add_user(user.profile.display_name.as_str(), mapped_user);
}
