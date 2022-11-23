use crate::db::save_user::DbUser;
use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use sqlx::PgPool;
use std::collections::HashMap;

/// get existing db users
pub async fn get_db_users(db_pool: &PgPool) -> Result<HashMap<String, F3User>, AppError> {
    let mut results = HashMap::<String, F3User>::new();
    println!("Fetch all users from db");
    let rows: Vec<DbUser> = sqlx::query_as!(
        DbUser,
        r#"
        SELECT slack_id, name, email, img_url
        FROM users;
    "#
    )
    .fetch_all(db_pool)
    .await?;
    println!("Finished fetching users");
    for item in rows {
        results.insert(item.slack_id.to_string(), F3User::from(item));
    }
    Ok(results)
}

/// get db user by name
pub async fn get_user_by_name(db_pool: &PgPool, name: &str) -> Result<Option<F3User>, AppError> {
    let name = name.to_lowercase();
    let result: Option<DbUser> = sqlx::query_as!(
        DbUser,
        r#"
        SELECT slack_id, name, email, img_url
        FROM users
        WHERE lower(name) = $1
        LIMIT 1;
        "#,
        name
    )
    .fetch_optional(db_pool)
    .await?;
    Ok(result.map(F3User::from))
}
