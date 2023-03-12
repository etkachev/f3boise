use crate::app_state::MutableAppState;
use crate::shared::common_errors::AppError;
use crate::web_api_routes::interactive_events::interaction_payload::{
    ActionUser, ViewSubmissionPayload, ViewSubmissionPayloadView, ViewSubmissionPayloadViewModal,
};
use crate::web_api_routes::slash_commands::back_blast::back_blast_post;
use crate::web_api_routes::slash_commands::modal_utils::view_ids::ViewIds;
use crate::web_api_routes::slash_commands::pre_blast::pre_blast_post;
use crate::web_api_state::MutableWebState;
use sqlx::PgPool;

/// handle a view submission from interactive event
pub async fn handle_view_submission(
    view_payload: &ViewSubmissionPayload,
    web_state: &MutableWebState,
    app_state: &MutableAppState,
    db_pool: &PgPool,
) -> Result<(), AppError> {
    let ViewSubmissionPayload { user, view } = view_payload;
    match view {
        ViewSubmissionPayloadView::Modal(modal) => {
            if let Some(view_id) = modal.modal_view_id() {
                match view_id {
                    ViewIds::PreBlast => {
                        handle_pre_blast_submission(modal, web_state, app_state, user).await
                    }
                    ViewIds::BackBlast => {
                        handle_back_blast_submission(modal, web_state, app_state, db_pool).await
                    }
                    ViewIds::Unknown => Ok(()),
                }
            } else {
                Ok(())
            }
        }
    }
}

async fn handle_back_blast_submission(
    modal: &ViewSubmissionPayloadViewModal,
    web_state: &MutableWebState,
    app_state: &MutableAppState,
    db_pool: &PgPool,
) -> Result<(), AppError> {
    use crate::db::save_back_blast;

    let form_values = modal.state.get_values();
    let post = back_blast_post::BackBlastPost::from(form_values);
    let db_data = back_blast_post::convert_to_bb_data(&post, app_state);
    save_back_blast::save(db_pool, &[db_data]).await?;
    let message = back_blast_post::convert_to_message(post, app_state);
    web_state.post_message(message).await
}

async fn handle_pre_blast_submission(
    modal: &ViewSubmissionPayloadViewModal,
    web_state: &MutableWebState,
    app_state: &MutableAppState,
    user: &ActionUser,
) -> Result<(), AppError> {
    let form_values = modal.state.get_values();
    let post = pre_blast_post::PreBlastPost::from(form_values);
    println!("from user {:?}", user.username);
    let message = pre_blast_post::convert_to_message(post, app_state);
    web_state.post_message(message).await
}
