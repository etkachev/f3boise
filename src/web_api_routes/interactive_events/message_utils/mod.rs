use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockType;
use crate::slack_api::chat::update_message::request::UpdateMessageRequest;
use crate::web_api_routes::interactive_events::interaction_payload::InteractionMessage;
use crate::web_api_state::MutableWebState;

pub async fn update_message_blocks_after_mut(
    web_state: &MutableWebState,
    copy_blocks: Option<Vec<BlockType>>,
    channel_id: &str,
    message: &InteractionMessage,
) -> Result<(), AppError> {
    if let Some(blocks) = copy_blocks {
        let request = UpdateMessageRequest::new(channel_id, &message.ts, blocks);
        web_state.update_message(request).await?;
    }
    Ok(())
}
