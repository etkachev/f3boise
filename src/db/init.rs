//! For initializing data needed to persist by default
//! 1. Sync AO list
//! 2. Fetch existing users from db into state
//! 3. Fetch public channels from slack and save to state (we don't persist in db for now)
//! 4. Fetch users from slack and save to state
//! 5. Upsert all users in app state to db

use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::ao_data::AoData;
use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

/// Sync full ao list
pub async fn sync_ao_list(db_pool: &PgPool) -> Result<(), AppError> {
    println!("Starting sync ao list");
    match db_pool.try_begin().await {
        Ok(transaction) => {
            let mut transaction = transaction.expect("Failed to begin transaction");
            println!("Start Sync ao list");
            for item in AO_LIST {
                let ao = AoData::from(&item);
                insert_ao_record(&mut transaction, &ao).await?;
            }
            println!("End Sync ao list");

            transaction
                .commit()
                .await
                .expect("Could not commit transaction");
            Ok(())
        }
        Err(err) => {
            println!("Err beginnging transaction: {:?}", err);
            AppError::Sqlx(err)
        }
    }
}

async fn insert_ao_record(
    transaction: &mut Transaction<'_, Postgres>,
    ao: &AoData,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    let name = &ao.name;
    let days = &ao.days;
    sqlx::query!(
        r#"
    INSERT INTO ao_list (id, name, days)
    VALUES($1,$2,$3)
    ON CONFLICT (name)
    DO NOTHING;
    "#,
        id,
        name,
        days
    )
    .execute(transaction)
    .await?;
    Ok(())
}

/// user represented in db.
pub struct DbUser {
    pub slack_id: String,
    pub name: String,
    pub email: String,
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
        }
    }
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
    println!("Start full sync users");
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    for (_, user) in users.iter() {
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

async fn upsert_user(
    transaction: &mut Transaction<'_, Postgres>,
    user: &DbUser,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
    INSERT INTO users (id, slack_id, name, email)
    VALUES($1,$2,$3,$4)
    ON CONFLICT (slack_id)
        DO NOTHING;
    "#,
        id,
        user.slack_id,
        user.name,
        user.email,
    )
    .execute(transaction)
    .await?;

    Ok(())
}
