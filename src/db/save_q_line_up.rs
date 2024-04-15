use crate::app_state::ao_data::AO;
use crate::shared::common_errors::AppError;
use crate::web_api_routes::interactive_events::interaction_types::QSheetActionComboData;
use chrono::NaiveDate;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

pub struct NewQLineUpDbEntry {
    pub id: Uuid,
    pub qs: String,
    pub ao: String,
    pub date: NaiveDate,
    pub closed: bool,
    pub channel_id: String,
}

impl NewQLineUpDbEntry {
    pub fn new(qs: Vec<String>, ao: &AO, date: &NaiveDate, channel_id: &str) -> Self {
        NewQLineUpDbEntry {
            id: Uuid::new_v4(),
            qs: qs.join(","),
            ao: ao.to_string(),
            date: *date,
            closed: false,
            channel_id: channel_id.to_string(),
        }
    }

    pub fn new_closed(ao: &AO, date: &NaiveDate, channel_id: &str) -> Self {
        NewQLineUpDbEntry {
            id: Uuid::new_v4(),
            qs: String::new(),
            ao: ao.to_string(),
            date: *date,
            closed: true,
            channel_id: channel_id.to_string(),
        }
    }
}

/// maps action trigger to new q line up db entry
pub fn map_from_action(
    action: &QSheetActionComboData,
    qs: Vec<String>,
    channel_id: String,
) -> Result<NewQLineUpDbEntry, AppError> {
    let qs = qs.join(",");

    Ok(NewQLineUpDbEntry {
        id: Uuid::new_v4(),
        qs,
        ao: action.ao.to_string(),
        date: action.date,
        closed: false,
        channel_id,
    })
}

/// save  list of q line up entries
pub async fn save_list(db_pool: &PgPool, list: &[NewQLineUpDbEntry]) -> Result<(), AppError> {
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    for item in list {
        save_entry(&mut transaction, item).await?;
    }

    transaction
        .commit()
        .await
        .expect("Could not commit transaction");

    Ok(())
}

async fn save_entry(
    transaction: &mut Transaction<'_, Postgres>,
    db_entry: &NewQLineUpDbEntry,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO q_line_up (id, qs, ao, date, closed, channel_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT ON CONSTRAINT q_line_up_channel_id_date_key
            DO NOTHING;
        "#,
        db_entry.id,
        db_entry.qs,
        db_entry.ao,
        db_entry.date,
        db_entry.closed,
        db_entry.channel_id
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

pub async fn close_q_line_up_entry(
    db_pool: &PgPool,
    ao: &AO,
    channel_id: &str,
    date: &NaiveDate,
) -> Result<(), AppError> {
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    close_single_q_line_up(&mut transaction, ao, channel_id, date).await?;
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}

async fn close_single_q_line_up(
    transaction: &mut Transaction<'_, Postgres>,
    ao: &AO,
    channel_id: &str,
    date: &NaiveDate,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    let ao = ao.to_string();
    sqlx::query!(
        r#"
    INSERT INTO q_line_up (id, qs, ao, date, closed, channel_id)
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT ON CONSTRAINT q_line_up_channel_id_date_key
    DO UPDATE
    SET qs = EXCLUDED.qs,
        closed = EXCLUDED.closed;
        "#,
        id,
        "closed",
        ao,
        date,
        true,
        channel_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

pub async fn delete_q_line_up_entry(
    db_pool: &PgPool,
    channel_id: &str,
    date: &NaiveDate,
) -> Result<(), AppError> {
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    delete_single_q_line_up(&mut transaction, channel_id, date).await?;
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}

/// delete single q line up by date and channel_id (ao).
async fn delete_single_q_line_up(
    transaction: &mut Transaction<'_, Postgres>,
    channel_id: &str,
    date: &NaiveDate,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        DELETE 
        FROM q_line_up
        WHERE channel_id = $1 AND date = $2;
        "#,
        channel_id,
        date
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
