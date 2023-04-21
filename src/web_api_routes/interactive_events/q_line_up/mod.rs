use crate::db::queries::q_line_up::get_single_q_line_up;
use crate::db::save_q_line_up::{
    close_q_line_up_entry, delete_q_line_up_entry, map_from_action, save_list,
};
use crate::shared::common_errors::AppError;
use crate::shared::constants;
use crate::shared::string_utils::{
    format_q_empty_row, format_q_line_up_date, map_q_line_up_existing, map_slack_id_to_link,
};
use crate::slack_api::block_kit::block_elements::BlockElementType;
use crate::slack_api::block_kit::BlockType;
use crate::web_api_routes::interactive_events::interaction_payload::{
    Action, ActionChannel, InteractionMessageTypes,
};
use crate::web_api_routes::interactive_events::interaction_types::QSheetActionComboData;
use crate::web_api_routes::interactive_events::message_utils::update_message_blocks_after_mut;
use crate::web_api_routes::interactive_events::q_line_up::utils::{
    get_ao_string_from_blocks, get_existing_q_overflow_options,
};
use crate::web_api_state::MutableWebState;
use sqlx::PgPool;

pub mod utils;

pub async fn clear_and_update_message(
    db_pool: &PgPool,
    web_state: &MutableWebState,
    action_combo: &QSheetActionComboData,
    channel_id: &str,
    action_channel: &Option<ActionChannel>,
    message: &Option<InteractionMessageTypes>,
    action: &Action,
) -> Result<(), AppError> {
    process_clearing_existing_q_line_up(db_pool, action_combo, channel_id).await?;
    clear_existing_q_line_up_message(web_state, action_channel, message, action_combo, action)
        .await?;
    Ok(())
}

pub async fn close_and_update_message(
    db_pool: &PgPool,
    web_state: &MutableWebState,
    action_combo: &QSheetActionComboData,
    channel_id: &str,
    action_channel: &Option<ActionChannel>,
    message: &Option<InteractionMessageTypes>,
    action_ids: (&str, &str),
) -> Result<(), AppError> {
    let (action_id, action_block_id) = action_ids;
    // update db
    process_closing_existing_q_line_up(db_pool, action_combo, channel_id).await?;
    // update message
    close_existing_q_line_up_message(
        web_state,
        action_channel,
        message,
        action_combo,
        action_id,
        action_block_id,
    )
    .await?;
    Ok(())
}

pub async fn process_q_line_up_event(
    db_pool: &PgPool,
    action: &QSheetActionComboData,
    users: Vec<String>,
    channel_id: String,
) -> Result<(), AppError> {
    let action = map_from_action(action, users, channel_id)?;
    let already_exists = get_single_q_line_up(db_pool, &action.date, &action.channel_id).await?;
    if already_exists.is_some() {
        return Err(AppError::General("Spot already taken".to_string()));
    }
    let list = vec![action];
    save_list(db_pool, &list).await?;
    Ok(())
}

/// update existing q line up message that was interacted with.
pub async fn update_existing_q_line_up_message(
    web_state: &MutableWebState,
    channel: &Option<ActionChannel>,
    message: &Option<InteractionMessageTypes>,
    action_combo: &QSheetActionComboData,
    action_id: &str,
    action_block_id: &str,
    slack_id: &str,
) -> Result<(), AppError> {
    if let Some(ActionChannel { id: channel_id, .. }) = channel {
        match message {
            Some(InteractionMessageTypes::Message(message)) => {
                let mut copy_blocks = message.blocks.clone();
                if let Some(blocks) = copy_blocks.as_mut() {
                    'blocks_list: for block in blocks {
                        if let BlockType::Section(section_block) = block {
                            match &section_block.block_id {
                                Some(block_id) if block_id.as_str() == action_block_id => {
                                    let friendly_date = format_q_line_up_date(&action_combo.date);
                                    let ao_combo_str = action_combo.ao.to_string();
                                    let ao_string = get_ao_string_from_blocks(
                                        &message.blocks,
                                        ao_combo_str.as_str(),
                                    );
                                    let updated_text = map_q_line_up_existing(
                                        &friendly_date,
                                        ao_string,
                                        vec![map_slack_id_to_link(slack_id)],
                                    );
                                    section_block.text.text = updated_text;
                                    let options = get_existing_q_overflow_options();
                                    section_block.accessory =
                                        Some(BlockElementType::new_overflow(action_id, options));
                                    break 'blocks_list;
                                }
                                _ => {}
                            }
                        }
                    }
                }

                update_message_blocks_after_mut(web_state, copy_blocks, channel_id, message)
                    .await?;
            }
            None => {}
        }
    }
    Ok(())
}

async fn process_closing_existing_q_line_up(
    db_pool: &PgPool,
    action: &QSheetActionComboData,
    channel_id: &str,
) -> Result<(), AppError> {
    close_q_line_up_entry(db_pool, &action.ao, channel_id, &action.date).await?;
    Ok(())
}

async fn process_clearing_existing_q_line_up(
    db_pool: &PgPool,
    action: &QSheetActionComboData,
    channel_id: &str,
) -> Result<(), AppError> {
    delete_q_line_up_entry(db_pool, channel_id, &action.date).await?;
    Ok(())
}

async fn close_existing_q_line_up_message(
    web_state: &MutableWebState,
    channel: &Option<ActionChannel>,
    message: &Option<InteractionMessageTypes>,
    action_combo: &QSheetActionComboData,
    action_id: &str,
    action_block_id: &str,
) -> Result<(), AppError> {
    if let Some(ActionChannel { id: channel_id, .. }) = channel {
        match message {
            Some(InteractionMessageTypes::Message(message)) => {
                let mut copy_blocks = message.blocks.clone();
                if let Some(blocks) = copy_blocks.as_mut() {
                    'blocks_list: for block in blocks {
                        if let BlockType::Section(section_block) = block {
                            match &section_block.block_id {
                                Some(block_id) if block_id.as_str() == action_block_id => {
                                    let friendly_date = format_q_line_up_date(&action_combo.date);
                                    let ao_combo_str = action_combo.ao.to_string();
                                    let ao_string = get_ao_string_from_blocks(
                                        &message.blocks,
                                        ao_combo_str.as_str(),
                                    );
                                    let updated_text = map_q_line_up_existing(
                                        &friendly_date,
                                        ao_string,
                                        vec!["closed".to_string()],
                                    );
                                    section_block.text.text = updated_text;
                                    let options = get_existing_q_overflow_options();
                                    section_block.accessory =
                                        Some(BlockElementType::new_overflow(action_id, options));
                                    break 'blocks_list;
                                }
                                _ => {}
                            }
                        }
                    }
                }

                update_message_blocks_after_mut(web_state, copy_blocks, channel_id, message)
                    .await?;
            }
            None => {}
        }
    }
    Ok(())
}

async fn clear_existing_q_line_up_message(
    web_state: &MutableWebState,
    channel: &Option<ActionChannel>,
    message: &Option<InteractionMessageTypes>,
    action_combo: &QSheetActionComboData,
    action: &Action,
) -> Result<(), AppError> {
    if let Some(ActionChannel { id: channel_id, .. }) = channel {
        match message {
            Some(InteractionMessageTypes::Message(message)) => {
                let mut copy_blocks = message.blocks.clone();
                if let Some(blocks) = copy_blocks.as_mut() {
                    let ao_combo_str = action_combo.ao.to_string();
                    let ao_string =
                        get_ao_string_from_blocks(&message.blocks, ao_combo_str.as_str());
                    'blocks_list: for block in blocks {
                        if let BlockType::Section(section_block) = block {
                            match &section_block.block_id {
                                Some(block_id) if block_id == &action.block_id => {
                                    let friendly_date = format_q_line_up_date(&action_combo.date);
                                    let text =
                                        format_q_empty_row(friendly_date.as_str(), ao_string);
                                    section_block.text.text = text;
                                    section_block.accessory = Some(BlockElementType::new_btn(
                                        constants::Q_LINE_UP_BTN_TEXT,
                                        action.action_id.as_str(),
                                    ));
                                    break 'blocks_list;
                                }
                                _ => {}
                            }
                        }
                    }
                }

                update_message_blocks_after_mut(web_state, copy_blocks, channel_id, message)
                    .await?;
            }
            None => {}
        }
    }
    Ok(())
}
