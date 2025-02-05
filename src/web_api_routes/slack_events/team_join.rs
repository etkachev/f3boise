use crate::app_state::MutableAppState;
use crate::db::save_user::{upsert_user, DbUser};
use crate::shared::admin::{backslash_id, stinger_id};
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

    send_welcome_direct_message(user.id.as_str(), web_app).await;
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

async fn send_welcome_direct_message(user_id: &str, web_state: &MutableWebState) {
    let backslash = backslash_id();
    let stinger = stinger_id();
    let help_desk_channel = PublicChannels::HelpDesk.channel_id();
    let mumble_chatter_channel = PublicChannels::MumbleChatter.channel_id();

    let block_builder = BlockBuilder::new()
        .header("Welcome to the F3 Boise!")
        .section_markdown("If you are new to F3, congrats! You've made a great decision for your life; don't stop now. This is just the beginning. F3 is more than a workout group; it's fellowship and a faith journey. Get fit, make friends, do good.")
        .divider()
        .header("ABOUT F3")
        .section_markdown("*Five Core Principles:*")
        .section_markdown("• Always Free")
        .section_markdown("• Open to All Men")
        .section_markdown("• Held Outdoors")
        .section_markdown("• Peed led")
        .section_markdown("• Ends in COT")
        .divider()
        .header("Website")
        .section_markdown("<https://f3boise.com/|f3boise.com>")
        .section_markdown("We have pages dedicated to help you learn the Lingo (Lexicon) and Exercises (Exicon) on our website")
        .divider()
        .header("Social Media")
        .section_markdown("<https://www.facebook.com/share/g/1BtvDJqH94/?mibextid=wwXIfr|Facebook>")
        .section_markdown("<https://www.instagram.com/f3boise/|IG>")
        .section_markdown("<https://twitter.com/f3boise|X/Twitter>")
        .divider()
        .header("SLACK")
        .section_markdown("Setup your profile:")
        .section_markdown("• Full Name (Given Name)")
        .section_markdown("• Display Name (F3 Name)")
        .section_markdown("• Title (Set to emergency contact phone as \"ICE: ###-###-####\")")
        .section_markdown("• Phone Number (Yours)")
        .divider()
        .section_markdown(format!("You've been added to a few general channels - <#{}> is the primary chat channel. Join other channels as you see fit. Leave channels, or edit your notification frequency, if it gets too noisy.", mumble_chatter_channel).as_str())
        .divider()
        .header("Other Available Channels")
        .section_markdown("We have channels related to 1st F (Fitness & CSAUPs), 2nd F (Fellowship) and 3rd F (Faith). Ask if you can't find one you are looking for.")
        .section_markdown("We also have a channel dedicated for each workout location. If you search in slack for '#ao' you should see a list of all of them. We do organize them by region as well.")
        .section_markdown(format!(
        "If you have any questions, just ask in <#{}> or hit up <@{}> or <@{}> (our IT folks).",
        help_desk_channel, backslash, stinger
        ).as_str())
        .section_markdown("Thanks!");

    if let Err(err) = web_state.send_direct_message(user_id, block_builder).await {
        println!("Err: {:?}", err);
    }
}
