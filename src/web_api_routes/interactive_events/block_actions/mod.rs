use crate::app_state::MutableAppState;
use crate::shared::common_errors::AppError;
use crate::shared::constants;
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::web_api_routes::interactive_events::interaction_payload::{
    ActionChannel, ActionType, ActionUser, BlockAction, ButtonAction, InteractionMessageTypes,
    OverflowAction,
};
use crate::web_api_routes::interactive_events::interaction_types::{
    InteractionTypes, QSheetActionComboData,
};
use crate::web_api_routes::interactive_events::q_line_up::{
    clear_and_update_message, process_q_line_up_event, update_existing_q_line_up_message,
};
use crate::web_api_state::MutableWebState;
use sqlx::PgPool;

/// handle block action interactions
pub async fn handle_block_actions(
    payload: &str,
    db_pool: &PgPool,
    app_state: &MutableAppState,
    web_state: &MutableWebState,
) -> Result<(), AppError> {
    let result = serde_json::from_str::<BlockAction>(payload)?;

    match result {
        BlockAction {
            actions,
            user,
            channel: action_channel,
            message,
            ..
        } if !actions.is_empty() => {
            println!("With {} actions", actions.len());
            // handling one action for now, but looking into scenarios where 2 might exist
            let first_action = &actions[0];
            let action_combo = InteractionTypes::from(first_action.get_action_id().as_str());
            match &action_combo {
                InteractionTypes::QLineUp(_) => {
                    handle_q_lineup_interaction(
                        db_pool,
                        app_state,
                        web_state,
                        first_action,
                        &user,
                        &message,
                        &action_channel,
                    )
                    .await?
                }
                InteractionTypes::EditBackBlast => {}
                InteractionTypes::Unknown => {
                    println!("Unknown interaction");
                }
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_q_lineup_interaction(
    db_pool: &PgPool,
    app_state: &MutableAppState,
    web_state: &MutableWebState,
    action_type: &ActionType,
    user: &ActionUser,
    message: &Option<InteractionMessageTypes>,
    action_channel: &Option<ActionChannel>,
) -> Result<(), AppError> {
    println!("q line up action");
    let interaction_type = InteractionTypes::from(action_type.get_action_id().as_str());
    if let InteractionTypes::QLineUp(action_combo) = &interaction_type {
        match action_type {
            ActionType::Button(ButtonAction { .. }) => {
                let users = {
                    let app = app_state.app.lock().expect("Could not lock app");
                    app.get_slack_id_map()
                };

                let slack_id = user.id.as_str();
                // f3 name
                let user = users
                    .get(slack_id)
                    .map(|name| name.to_string())
                    .unwrap_or_else(String::new);

                if user.is_empty() {
                    return Err(AppError::General("Could not find user".to_string()));
                }

                let channel_id = get_channel_id_from_action(action_combo, app_state)?;
                process_q_line_up_event(db_pool, action_combo, vec![user.to_string()], channel_id)
                    .await?;
                println!("succeeded saving q line up!");
                update_existing_q_line_up_message(
                    web_state,
                    action_channel,
                    message,
                    action_combo,
                    &action_type.get_action_id(),
                    &action_type.get_block_id(),
                    slack_id,
                )
                .await?;
            }
            ActionType::Overflow(OverflowAction {
                action,
                selected_option,
            }) => {
                match selected_option.value.as_str() {
                    // when canceling existing q line up
                    constants::Q_LINE_UP_CANCEL_TEXT => {
                        let channel_id = get_channel_id_from_action(action_combo, app_state)?;
                        clear_and_update_message(
                            db_pool,
                            web_state,
                            action_combo,
                            channel_id.as_str(),
                            action_channel,
                            message,
                            action,
                        )
                        .await?;
                    }
                    _ => println!("Unknown overflow option"),
                }
            }
        }
    }

    Ok(())
}

fn get_channel_id_from_action(
    action_combo: &QSheetActionComboData,
    app_state: &MutableAppState,
) -> Result<String, AppError> {
    let channel = PublicChannels::from(&action_combo.ao);
    let channel_id = {
        let app = app_state.app.lock().expect("Could not lock app");
        let all_channels = &app.channels;
        all_channels
            .get(&channel)
            .map(|c| c.id.to_string())
            .unwrap_or_else(String::new)
    };

    if channel_id.is_empty() {
        Err(AppError::General("Could not find ao".to_string()))
    } else {
        Ok(channel_id)
    }
}