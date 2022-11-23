use crate::app_state::ao_data::AoData;
use crate::shared::common_errors::AppError;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

/// insert ao record if doesn't exist
pub async fn insert_ao_record(
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
