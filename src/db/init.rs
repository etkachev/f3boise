//! For initializing data needed to persist by default
//! 1. Sync AO list
//! 2. Fetch existing users from db into state
//! 3. Fetch public channels from slack and save to state (we don't persist in db for now)
//! 4. Fetch users from slack and save to state
//! 5. Upsert all users in app state to db

use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::ao_data::AoData;
use crate::db::insert_ao::insert_ao_record;
use crate::db::queries::users::get_db_users;
use crate::db::save_user::{upsert_user, DbUser};
use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use sqlx::PgPool;
use std::collections::HashMap;

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
        println!("Upsert user");
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
