use crate::db::queries::users::get_slack_id_map;
use crate::shared::common_errors::AppError;
use crate::slack_api::channels::invite::request::InviteToConvoRequest;
use crate::web_api_routes::slash_commands::SlashCommandForm;
use crate::web_api_state::MutableWebState;
use sqlx::PgPool;

pub async fn handle_invite_all(
    db_pool: &PgPool,
    web_state: &MutableWebState,
    form: &SlashCommandForm,
) -> Result<String, AppError> {
    let existing = web_state
        .get_channel_members(form.channel_id.as_str())
        .await?;
    let users = get_slack_id_map(db_pool).await?;
    let request = InviteToConvoRequest::new(form.channel_id.as_str(), users, existing);
    web_state.invite_users_to_channel(request).await?;
    Ok(String::from("Successfully invited all users"))
}
