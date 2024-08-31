use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

/// user represented in db.
#[derive(Debug)]
pub struct DbUser {
    pub slack_id: String,
    pub name: String,
    pub email: String,
    pub img_url: Option<String>,
    pub parent: Option<serde_json::Value>,
    pub create_date: NaiveDateTime,
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
            parent: None,
            create_date: NaiveDateTime::default(),
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
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

/// update only username once in case they forget to set their display name when they first join
pub async fn update_user_name(db: &PgPool, slack_id: &str, name: &str) -> Result<(), AppError> {
    sqlx::query!(
        r#"
    UPDATE users
    SET name = CASE WHEN locked_name_update THEN name ELSE $2 END,
        locked_name_update = TRUE
    WHERE slack_id = $1 AND NOT locked_name_update;
    "#,
        slack_id,
        name,
    )
    .execute(db)
    .await?;
    Ok(())
}

/// Sync user to db from other db if slack_id doesn't exist already
pub async fn sync_user(
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
            img_url = EXCLUDED.img_url,
            create_date = $6;
    "#,
        id,
        user.slack_id,
        user.name,
        user.email,
        user.img_url,
        user.create_date
    )
    .execute(&mut **transaction)
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
    .execute(&mut **transaction)
    .await?;

    Ok(())
}
