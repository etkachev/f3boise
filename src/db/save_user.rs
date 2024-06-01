use crate::shared::common_errors::AppError;
use crate::users::f3_user::{F3Parent, F3User};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

/// user represented in db.
#[derive(Debug)]
pub struct DbUser {
    pub slack_id: String,
    pub name: String,
    pub email: String,
    pub img_url: Option<String>,
    pub parent: Option<String>,
    pub parent_type: Option<String>,
}

impl From<&F3User> for DbUser {
    fn from(user: &F3User) -> Self {
        let id = if let Some(id) = &user.id {
            id.to_string()
        } else {
            String::new()
        };
        let (parent_type, parent) = user
            .invited_by
            .as_ref()
            .map(|invited_by| match invited_by {
                F3Parent::Pax(pax) => (Some("pax"), Some(pax.to_string())),
                F3Parent::Online => (Some("online"), None),
            })
            .unwrap_or((None, None));
        DbUser {
            slack_id: id,
            name: user.name.to_string(),
            email: user.email.to_string(),
            img_url: user.img_url.clone(),
            parent,
            parent_type: parent_type.map(|pt| pt.to_string()),
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

/// update parent info for a pax
pub async fn update_user_parent_data(
    transaction: &mut Transaction<'_, Postgres>,
    user: &DbUser,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE users SET parent = $2, parent_type = $3
        WHERE slack_id = $1 AND parent IS NULL
        "#,
        user.slack_id,
        user.parent,
        user.parent_type
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
