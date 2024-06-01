use crate::db::queries::users::get_user_by_slack_id;
use crate::db::save_user::{update_user_parent_data, DbUser};
use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3Parent;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct PaxParentRequest {
    slack_id: String,
    invited_by: F3Parent,
}

/// route for setting parent data for a pax user
pub async fn set_pax_parent_route(
    db_pool: web::Data<PgPool>,
    req: web::Json<PaxParentRequest>,
) -> impl Responder {
    match set_pax_parent(&db_pool, &req).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn set_pax_parent(db: &PgPool, req: &PaxParentRequest) -> Result<(), AppError> {
    let existing_user = get_user_by_slack_id(db, &req.slack_id).await?;

    if let Some(mut user) = existing_user {
        let mut transaction = db.begin().await.expect("Failed to begin transaction");

        user.invited_by = Some(req.invited_by.clone());
        let db_user = DbUser::from(&user);
        update_user_parent_data(&mut transaction, &db_user).await?;

        transaction
            .commit()
            .await
            .expect("Could not commit transaction");
        Ok(())
    } else {
        Err(AppError::General("Couldn't find user".to_string()))
    }
}
