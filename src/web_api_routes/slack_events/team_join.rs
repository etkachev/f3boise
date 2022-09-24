use crate::app_state::MutableAppState;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::slack_api::users::users_list::response::SlackUserData;
use crate::users::f3_user::F3User;
use crate::web_api_state::MutableWebState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamJoinData {
    pub user: SlackUserData,
}

pub async fn handle_new_user(
    user: &SlackUserData,
    app_state: &MutableAppState,
    web_app: &MutableWebState,
) {
    let mapped_user = F3User::from(user);
    let channel_id = {
        let mut app = app_state.app.lock().unwrap();
        app.add_user(user.id.as_str(), mapped_user);
        app.get_channel_data(PublicChannels::Welcome)
            .map(|data| data.id.to_string())
    };

    if let Some(channel_id) = channel_id {
        post_message_to_welcome_channel(channel_id.as_str(), user.id.as_str(), web_app).await;
    }
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
