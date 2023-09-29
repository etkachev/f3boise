use crate::app_state::MutableAppState;
use crate::db::save_user::{upsert_user, DbUser};
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::slack_api::users::users_list::response::SlackUserData;
use crate::users::f3_user::F3User;
use crate::web_api_state::MutableWebState;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamJoinData {
    pub user: SlackUserData,
}

pub async fn handle_new_user(
    db_pool: &PgPool,
    user: &SlackUserData,
    app_state: &MutableAppState,
    web_app: &MutableWebState,
) {
    let mapped_user = F3User::from(user);
    if let Err(err) = add_user_to_db(&mapped_user, db_pool).await {
        println!("Error handling new user: {:?}", err);
    }
    let channel_id = {
        let app = app_state.app.lock().unwrap();
        app.get_channel_data(PublicChannels::Welcome)
            .map(|data| data.id.to_string())
    };

    if let Some(channel_id) = channel_id {
        post_message_to_welcome_channel(channel_id.as_str(), user.id.as_str(), web_app).await;
    }
}

async fn add_user_to_db(user: &F3User, db_pool: &PgPool) -> Result<(), AppError> {
    let user = DbUser::from(user);
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    upsert_user(&mut transaction, &user).await?;
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}

async fn post_message_to_welcome_channel(
    channel_id: &str,
    user_id: &str,
    web_app: &MutableWebState,
) {
    let block_builder = BlockBuilder::new().section_markdown(
        format!(
            "Please welcome new member to F3 Boise! :wave: <@{}>",
            user_id
        )
        .as_str(),
    );

    let request = PostMessageRequest::new(channel_id, block_builder.blocks);
    if let Err(err) = web_app.post_message(request).await {
        println!("Err: {:?}", err.to_string());
    }
}
