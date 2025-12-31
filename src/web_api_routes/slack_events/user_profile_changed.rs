use crate::db::save_user;
use crate::shared::f3_api_sync::sync_user;
use crate::slack_api::users::users_list::response::SlackUserData;
use crate::users::f3_user::F3User;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfileChangedData {
    pub user: SlackUserData,
}

pub async fn handle_user_profile_changed(db: &PgPool, user_data: &UserProfileChangedData) {
    if !user_data.user.profile.display_name.is_empty() {
        if let Err(err) = save_user::update_user_name(
            db,
            user_data.user.id.as_str(),
            user_data.user.profile.display_name.as_str(),
        )
        .await
        {
            println!("Error updating user: {:?}", err);
        }

        // sync to F3 API
        let mapped_user = F3User::from(&user_data.user);
        sync_user(&mapped_user).await;
    }
}
