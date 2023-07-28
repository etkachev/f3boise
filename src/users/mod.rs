use crate::db::queries::users::get_db_users;
use crate::shared::common_errors::AppError;
use sqlx::PgPool;
use std::collections::HashMap;

pub mod f3_user;

// return hashmap of slack ids and username. key is slack id, value is name
pub async fn get_slack_id_map(db: &PgPool) -> Result<HashMap<String, String>, AppError> {
    let db_users = get_db_users(db).await?;
    let mapped = db_users.iter().fold(
        HashMap::<String, String>::new(),
        |mut acc, (slack_id, user)| {
            acc.insert(slack_id.to_string(), user.name.to_string());
            acc
        },
    );
    Ok(mapped)
}
