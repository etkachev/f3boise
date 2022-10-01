//! For initializing data needed to persist by default
//! 1. Sync AO list
//! 2. Fetch existing users from db into state
//! 3. Fetch public channels from slack and save to state (we don't persist in db for now)
//! 4. Fetch users from slack and save to state
//! 5. Upsert all users in app state to db

use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::ao_data::AoData;
use crate::db::save_user::{upsert_user, DbUser};
use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

/// Sync full ao list
pub async fn sync_ao_list(db_pool: &PgPool) -> Result<(), AppError> {
    match db_pool.try_begin().await {
        Ok(transaction) => {
            let mut transaction = transaction.expect("Failed to begin transaction");
            for item in AO_LIST {
                let ao = AoData::from(&item);
                let channel_id = item.channel_id();
                insert_ao_record(&mut transaction, &ao, channel_id).await?;
            }

            transaction
                .commit()
                .await
                .expect("Could not commit transaction");
            Ok(())
        }
        Err(err) => {
            println!("Err beginnging transaction: {:?}", err);
            Err(AppError::Sqlx(err))
        }
    }
}

async fn insert_ao_record(
    transaction: &mut Transaction<'_, Postgres>,
    ao: &AoData,
    channel_id: &str,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    let name = &ao.name;
    let days = &ao.days;
    let active = &ao.active;
    sqlx::query!(
        r#"
    INSERT INTO ao_list (id, name, days, channel_id, active)
    VALUES($1,$2,$3,$4,$5)
    ON CONFLICT (name)
    DO NOTHING;
    "#,
        id,
        name,
        days,
        channel_id,
        active
    )
    .execute(transaction)
    .await?;
    Ok(())
}

/// get existing db users
pub async fn get_db_users(db_pool: &PgPool) -> Result<HashMap<String, F3User>, AppError> {
    let mut results = HashMap::<String, F3User>::new();
    println!("Fetch all users from db");
    let rows: Vec<DbUser> = sqlx::query_as!(
        DbUser,
        r#"
        SELECT slack_id, name, email
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

pub async fn sync_users(db_pool: &PgPool, users: &HashMap<String, F3User>) -> Result<(), AppError> {
    let existing_users = get_db_users(db_pool).await?;
    println!(
        "existing: {} ----- all: {}",
        existing_users.len(),
        users.len()
    );
    println!("Start full sync users");
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    for (_, user) in users
        .iter()
        .filter(|(slack_id, _)| !existing_users.contains_key(&slack_id.to_string()))
    {
        println!("Inserting new user");
        upsert_user(&mut transaction, &DbUser::from(user)).await?;
    }
    println!("Finishing full sync users");
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    println!("Finished full sync users");
    Ok(())
}
