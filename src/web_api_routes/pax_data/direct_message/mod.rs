use crate::shared::admin::{backslash_id, stinger_id};
use crate::shared::responses::{failure, success};
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::web_api_state::MutableWebState;
use actix_web::{web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DMRequest {
    slack_id: String,
    msg: String,
}

/// send direct message to someone on slack
pub async fn send_direct_message_route(
    web_state: web::Data<MutableWebState>,
    req: web::Json<DMRequest>,
) -> impl Responder {
    let block_builder = BlockBuilder::new().section_markdown(&req.msg);
    match web_state
        .send_direct_message(&req.slack_id, block_builder)
        .await
    {
        Ok(_) => success(),
        Err(err) => failure(err),
    }
}

/// testing welcome direct message route
pub async fn test_welcome_direct_message(
    web_state: web::Data<MutableWebState>,
    req: web::Json<DMRequest>,
) -> impl Responder {
    let backslash = backslash_id();
    let stinger = stinger_id();
    let channel_id = PublicChannels::HelpDesk.channel_id();
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
        .section_markdown("You've been added to the #general channel - it is the primary chat channel. Join other channels as you see fit. Leave channels, or edit your notification frequency, if it gets too noisy. Within each channel are conversation threads, similar to email threads. To reply to a thread, TAP on the thread FIRST. (Disregard the checkmark for 'Also send to #channel')")
        .divider()
        .header("Other Available Channels")
        .section_markdown("We have channels related to 1st F (Fitness & CSAUPs), 2nd F (Fellowship) and 3rd F (Faith). Ask if you can't find one you are looking for.")
        .section_markdown("We also have a channel dedicated for each workout location. If you search in slack for '#ao' you should see a list of all of them. We do organize them by region as well.")
        .section_markdown(format!(
        "If you have any questions, just ask in <#{}> or hit up <@{}> or <@{}> (our IT folks).",
        channel_id, backslash, stinger
        ).as_str())
        .section_markdown("Thanks!");

    match web_state
        .send_direct_message(&req.slack_id, block_builder)
        .await
    {
        Ok(_) => success(),
        Err(err) => failure(err),
    }
}
