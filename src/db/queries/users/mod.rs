use crate::db::save_user::DbUser;
use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use sqlx::PgPool;
use std::collections::HashMap;

/// get existing db users. key is slack id
pub async fn get_db_users(db_pool: &PgPool) -> Result<HashMap<String, F3User>, AppError> {
    let mut results = HashMap::<String, F3User>::new();
    println!("Fetch all users from db");
    let rows: Vec<DbUser> = sqlx::query_as!(
        DbUser,
        r#"
        SELECT slack_id, name, email, img_url, parent, parent_type
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

/// query minimal slack id and f3 name of all users.
async fn query_slack_user_map(db_pool: &PgPool) -> Result<Vec<DbUserMap>, AppError> {
    let rows = sqlx::query_as!(
        DbUserMap,
        r#"
        SELECT slack_id, name
        FROM users;
        "#
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows)
}

/// get hashmap where key is slack id and value is f3 name
pub async fn get_slack_id_map(db_pool: &PgPool) -> Result<HashMap<String, String>, AppError> {
    let mut results = HashMap::<String, String>::new();
    let rows = query_slack_user_map(db_pool).await?;

    for item in rows {
        results.insert(item.slack_id.to_string(), item.name.to_string());
    }

    Ok(results)
}

/// get hashmap where key is f3 name and value is slack id
pub async fn get_user_name_map(db_pool: &PgPool) -> Result<HashMap<String, String>, AppError> {
    let mut results = HashMap::<String, String>::new();

    let rows = query_slack_user_map(db_pool).await?;

    for item in rows {
        results.insert(item.name.to_lowercase(), item.slack_id.to_string());
    }

    Ok(results)
}

/// Minimal map of user slack id and f3 name
pub struct DbUserMap {
    pub slack_id: String,
    pub name: String,
}

/// get db user by name
pub async fn get_user_by_name(db_pool: &PgPool, name: &str) -> Result<Option<F3User>, AppError> {
    let name = name.to_lowercase();
    let result: Option<DbUser> = sqlx::query_as!(
        DbUser,
        r#"
        SELECT slack_id, name, email, img_url, parent, parent_type
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

/// get db user by slack id
pub async fn get_user_by_slack_id(db_pool: &PgPool, id: &str) -> Result<Option<F3User>, AppError> {
    let result = sqlx::query_as!(
        DbUser,
        r#"
        SELECT slack_id, name, email, img_url, parent, parent_type
        FROM users
        WHERE slack_id = $1
        LIMIT 1;
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(result.map(F3User::from))
}
