use crate::app_state::pre_blast_data::PreBlastData;
use crate::db::queries::users::get_slack_id_map;
use crate::shared::common_errors::AppError;
use crate::web_api_routes::interactive_events::interaction_payload::{
    ActionUser, ViewSubmissionPayload, ViewSubmissionPayloadView, ViewSubmissionPayloadViewModal,
};
use crate::web_api_routes::slash_commands::back_blast::back_blast_post;
use crate::web_api_routes::slash_commands::black_diamond_rating::black_diamond_rating_post;
use crate::web_api_routes::slash_commands::modal_utils::view_ids::ViewIds;
use crate::web_api_routes::slash_commands::pre_blast::pre_blast_post;
use crate::web_api_state::MutableWebState;
use sqlx::PgPool;

/// handle a view submission from interactive event
pub async fn handle_view_submission(
    payload: &str,
    web_state: &MutableWebState,
    db_pool: &PgPool,
) -> Result<(), AppError> {
    let view_payload = serde_json::from_str::<ViewSubmissionPayload>(payload)?;

    let ViewSubmissionPayload { user, view } = &view_payload;
    match view {
        ViewSubmissionPayloadView::Modal(modal) => {
            if let Some(view_id) = modal.modal_view_id() {
                match view_id {
                    ViewIds::PreBlast => {
                        handle_pre_blast_submission(modal, db_pool, web_state, user).await
                    }
                    ViewIds::BackBlast => {
                        handle_back_blast_submission(modal, web_state, db_pool, user).await
                    }
                    ViewIds::BlackDiamondRating => {
                        handle_black_diamond_rating_submission(modal, web_state, db_pool, user)
                            .await
                    }
                    ViewIds::BackBlastEdit => {
                        handle_edit_back_blast_submission(modal, web_state, db_pool).await
                    }
                    ViewIds::PreBlastEdit => {
                        handle_edit_pre_blast_submission(modal, web_state, db_pool).await
                    }
                    ViewIds::Unknown => Ok(()),
                }
            } else {
                Ok(())
            }
        }
    }
}

async fn handle_black_diamond_rating_submission(
    modal: &ViewSubmissionPayloadViewModal,
    web_state: &MutableWebState,
    db_pool: &PgPool,
    user: &ActionUser,
) -> Result<(), AppError> {
    let form_values = modal.state.get_values();
    let post = black_diamond_rating_post::BlackDiamondRatingPost::from(form_values);
    let message =
        black_diamond_rating_post::convert_to_message(post, db_pool, user.id.as_str()).await;
    web_state.post_message(message).await?;
    Ok(())
}

async fn handle_edit_pre_blast_submission(
    modal: &ViewSubmissionPayloadViewModal,
    web_state: &MutableWebState,
    db_pool: &PgPool,
) -> Result<(), AppError> {
    use crate::db::queries::pre_blasts;
    use crate::db::save_pre_blast;

    let form_values = modal.state.get_values();
    let post = pre_blast_post::PreBlastPost::from(form_values);
    let users = get_slack_id_map(db_pool).await?;
    let db_data = PreBlastData::from(&post).with_qs(&post.qs, users);
    if let Some(id) = &modal.private_metadata {
        // save to backend
        save_pre_blast::update_pre_blast(db_pool, id, &db_data).await?;
        // fetch latest
        let updated_pb = pre_blasts::get_pre_blast_by_id(db_pool, id).await?;
        if let Some(ts) = updated_pb.map(|pb| pb.ts).unwrap_or_default() {
            let message = pre_blast_post::convert_to_update_message(
                post,
                modal.private_metadata.clone(),
                ts.as_str(),
            );
            // send message update to slack
            let ts = web_state.update_message(message).await?;
            if let Some(ts) = ts {
                // update ts on preblast
                save_pre_blast::update_pre_blast_ts(db_pool, id, ts).await?;
            }
        }
    }
    // todo
    Ok(())
}

async fn handle_edit_back_blast_submission(
    modal: &ViewSubmissionPayloadViewModal,
    web_state: &MutableWebState,
    db_pool: &PgPool,
) -> Result<(), AppError> {
    use crate::db::queries::all_back_blasts;
    use crate::db::save_back_blast;

    let form_values = modal.state.get_values();
    let post = back_blast_post::BackBlastPost::from(form_values);
    let users = get_slack_id_map(db_pool).await?;
    let db_data = back_blast_post::convert_to_bb_data(&post, users);
    let is_valid = db_data.is_valid_back_blast();
    if is_valid {
        if let Some(id) = &modal.private_metadata {
            // save to backend
            save_back_blast::update_back_blast(db_pool, id, &db_data).await?;
            // fetch latest update
            let updated_bb = all_back_blasts::get_back_blast_by_id(db_pool, id).await?;
            if let Some(ts) = updated_bb.map(|bb| bb.ts).unwrap_or_default() {
                let message = back_blast_post::convert_to_update_message(
                    post,
                    true,
                    modal.private_metadata.clone(),
                    ts.as_str(),
                );
                // send message update to slack
                let ts = web_state.update_message(message).await?;
                if let Some(ts) = ts {
                    // update ts on backblast
                    save_back_blast::update_back_blast_ts(db_pool, id, ts).await?;
                }
            }
        }
    }
    Ok(())
}

async fn handle_back_blast_submission(
    modal: &ViewSubmissionPayloadViewModal,
    web_state: &MutableWebState,
    db_pool: &PgPool,
    user: &ActionUser,
) -> Result<(), AppError> {
    use crate::db::save_back_blast;

    let form_values = modal.state.get_values();
    let post = back_blast_post::BackBlastPost::from(form_values);
    let users = get_slack_id_map(db_pool).await?;
    let db_data = back_blast_post::convert_to_bb_data(&post, users);
    let is_valid = db_data.is_valid_back_blast();
    let mut id: Option<String> = None;
    if is_valid {
        // save single back blast
        let saved_id = save_back_blast::save_single(db_pool, &db_data).await?;
        id = Some(saved_id);
    }
    let message =
        back_blast_post::convert_to_message(post, db_pool, is_valid, id.clone(), &user.id).await;

    // post message to slack
    let ts = web_state.post_message(message).await?;
    if let (Some(id), Some(ts)) = (id, ts) {
        // update backblast ts in db
        save_back_blast::update_back_blast_ts(db_pool, id.as_str(), ts).await?;
    }
    Ok(())
}

async fn handle_pre_blast_submission(
    modal: &ViewSubmissionPayloadViewModal,
    db_pool: &PgPool,
    web_state: &MutableWebState,
    user: &ActionUser,
) -> Result<(), AppError> {
    use crate::db::save_pre_blast;

    let form_values = modal.state.get_values();
    let post = pre_blast_post::PreBlastPost::from(form_values);
    let users = get_slack_id_map(db_pool).await?;
    let db_data = PreBlastData::from(&post).with_qs(&post.qs, users);
    let saved_id = save_pre_blast::save_single(db_pool, &db_data).await?;
    let message = pre_blast_post::convert_to_message(db_pool, post, &saved_id, &user.id).await;
    // post message to slack
    let ts = web_state.post_message(message).await?;
    if let Some(ts) = ts {
        save_pre_blast::update_pre_blast_ts(db_pool, &saved_id, ts).await?;
    }
    Ok(())
}
