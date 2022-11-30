use crate::app_state::MutableAppState;
use crate::shared::common_errors::AppError;
use crate::slack_api::channels::invite::request::InviteToConvoRequest;
use crate::web_api_routes::slash_commands::SlashCommandForm;
use crate::web_api_state::MutableWebState;

pub async fn handle_invite_all(
    web_state: &MutableWebState,
    app_state: &MutableAppState,
    form: &SlashCommandForm,
) -> Result<String, AppError> {
    let existing = web_state
        .get_channel_members(form.channel_id.as_str())
        .await?;
    let users = {
        let app = app_state.app.lock().expect("Could not lock app state");
        app.get_slack_id_map()
    };
    let request = InviteToConvoRequest::new(form.channel_id.as_str(), users, existing);
    web_state.invite_users_to_channel(request).await?;
    Ok(String::from("Successfully invited all users"))
}
