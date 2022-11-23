use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

/// user represented in db.
#[derive(Debug)]
pub struct DbUser {
    pub slack_id: String,
    pub name: String,
    pub email: String,
    pub img_url: Option<String>,
}

impl From<&F3User> for DbUser {
    fn from(user: &F3User) -> Self {
        let id = if let Some(id) = &user.id {
            id.to_string()
        } else {
            String::new()
        };
        DbUser {
            slack_id: id,
            name: user.name.to_string(),
            email: user.email.to_string(),
            img_url: user.img_url.clone(),
        }
    }
}

/// Upsert user to db if slack_id doesn't exist already
pub async fn upsert_user(
    transaction: &mut Transaction<'_, Postgres>,
    user: &DbUser,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
    INSERT INTO users (id, slack_id, name, email, img_url)
    VALUES($1,$2,$3,$4,$5)
    ON CONFLICT (slack_id)
        DO UPDATE
        SET name = EXCLUDED.name,
            img_url = EXCLUDED.img_url;
    "#,
        id,
        user.slack_id,
        user.name,
        user.email,
        user.img_url
    )
    .execute(transaction)
    .await?;

    Ok(())
}

/// Upsert user profile img
pub async fn update_user_profile_img(
    transaction: &mut Transaction<'_, Postgres>,
    user: &DbUser,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
    INSERT INTO users (id, slack_id, name, email, img_url)
    VALUES($1,$2,$3,$4,$5)
    ON CONFLICT (slack_id)
        DO UPDATE
        SET img_url = EXCLUDED.img_url;
    "#,
        id,
        user.slack_id,
        user.name,
        user.email,
        user.img_url
    )
    .execute(transaction)
    .await?;

    Ok(())
}
