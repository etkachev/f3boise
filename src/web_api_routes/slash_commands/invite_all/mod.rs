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
    let request = {
        let app = app_state.app.lock().expect("Could not lock app state");
        InviteToConvoRequest::new(form.channel_id.as_str(), &app.users)
    };
    web_state.invite_users_to_channel(request).await?;
    Ok(String::from("Successfully invited all users"))
}
