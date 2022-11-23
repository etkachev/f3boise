use crate::db::save_user::{update_user_profile_img, DbUser};
use crate::shared::common_errors::AppError;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

/// route for syncing data to state
pub async fn sync_user_imgs_route(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
) -> impl Responder {
    match update_user_imgs(&db_pool, &web_state).await {
        Ok(_) => HttpResponse::Ok().body("Synced profile imgs!"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

pub async fn update_user_imgs(
    db_pool: &PgPool,
    web_state: &MutableWebState,
) -> Result<(), AppError> {
    let slack_users = web_state.get_users().await?;
    let users = slack_users.users;
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    for (_, user) in users.iter() {
        update_user_profile_img(&mut transaction, &DbUser::from(user)).await?;
    }
    println!("Finishing upserting user images");
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}
