use crate::app_state::backblast_data::BackBlastData;
use crate::app_state::MutableAppState;
use crate::db::queries::users::get_slack_id_map;
use crate::shared::common_errors::AppError;
use crate::shared::constants;
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::slack_api::views::payload::ViewPayload;
use crate::slack_api::views::request::ViewsOpenRequest;
use crate::web_api_routes::interactive_events::edit_backblast::{
    create_edit_modal, get_back_blast, get_user_data, BackBlastUsersEdit,
};
use crate::web_api_routes::interactive_events::interaction_payload::{
    ActionChannel, ActionType, ActionUser, BlockAction, ButtonAction, InteractionMessageTypes,
    OverflowAction,
};
use crate::web_api_routes::interactive_events::interaction_types::{
    InteractionTypes, QSheetActionComboData,
};
use crate::web_api_routes::interactive_events::q_line_up::{
    clear_and_update_message, close_and_update_message, process_q_line_up_event,
    update_existing_q_line_up_message,
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
            trigger_id,
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
                InteractionTypes::EditBackBlast(id) => {
                    handle_edit_back_blast(
                        db_pool,
                        web_state,
                        id,
                        &action_channel,
                        trigger_id.as_str(),
                        &user,
                    )
                    .await?
                }
                InteractionTypes::Unknown => {
                    println!("Unknown interaction");
                }
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_edit_back_blast(
    db_pool: &PgPool,
    web_state: &MutableWebState,
    id: &str,
    action_channel: &Option<ActionChannel>,
    trigger_id: &str,
    user: &ActionUser,
) -> Result<(), AppError> {
    let bb = get_back_blast(db_pool, id).await?;
    let channel = action_channel.as_ref().map(|c| c.id.to_string());
    if channel.is_none() {
        return Err(AppError::from("Missing channel id"));
    }

    let channel = channel.unwrap();
    let users = get_user_data(db_pool, &bb).await?;
    // only Q's can edit backblast
    if !user_allowed_to_edit(user, &bb, &users) {
        return Ok(());
    }
    let modal = create_edit_modal(channel.as_str(), &bb, users, id);
    let view = ViewsOpenRequest::new(trigger_id, ViewPayload::Modal(modal));
    web_state.open_view(view).await?;
    Ok(())
}

/// only qs can edit back blast
fn user_allowed_to_edit(
    user: &ActionUser,
    bb: &BackBlastData,
    user_edit: &BackBlastUsersEdit,
) -> bool {
    let slack_ids = user_edit.convert_to_slack_ids(&bb.qs);
    slack_ids.iter().any(|id| &user.id == id)
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
                let users = get_slack_id_map(db_pool).await.unwrap_or_default();

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
                    // when closing existing q line up
                    constants::Q_LINE_UP_CLOSED_TEXT => {
                        let channel_id = get_channel_id_from_action(action_combo, app_state)?;
                        close_and_update_message(
                            db_pool,
                            web_state,
                            action_combo,
                            channel_id.as_str(),
                            action_channel,
                            message,
                            (&action_type.get_action_id(), &action_type.get_block_id()),
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
            .unwrap_or_default()
    };

    if channel_id.is_empty() {
        Err(AppError::General("Could not find ao".to_string()))
    } else {
        Ok(channel_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_state::ao_data::AO;
    use crate::users::f3_user::F3User;
    use chrono::NaiveDate;
    use std::collections::HashSet;

    fn mock_user(id: &str, name: &str) -> F3User {
        F3User {
            id: Some(id.to_string()),
            name: name.to_string(),
            email: "".to_string(),
            img_url: None,
            invited_by: None,
        }
    }

    #[test]
    fn only_allow_qs() {
        let action_user = ActionUser {
            id: "123".to_string(),
            name: "Stinger".to_string(),
            username: "stinger".to_string(),
        };
        let bb = BackBlastData::new(
            AO::Tower,
            HashSet::from(["backslash".to_string()]),
            HashSet::from(["stinger".to_string()]),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        );
        let edit_data = BackBlastUsersEdit::new(
            vec![mock_user("123", "stinger"), mock_user("22", "backslash")],
            vec![],
        );
        let allowed = user_allowed_to_edit(&action_user, &bb, &edit_data);
        assert_eq!(allowed, false);
        let action_user = ActionUser {
            id: "22".to_string(),
            name: "backslash".to_string(),
            username: "backslash".to_string(),
        };
        let allowed = user_allowed_to_edit(&action_user, &bb, &edit_data);
        assert_eq!(allowed, true);
    }
}
