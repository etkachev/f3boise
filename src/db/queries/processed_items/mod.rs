use crate::shared::common_errors::AppError;
use crate::shared::processed_type::ResolvingProcessedItems;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessedItem {
    pub id: Uuid,
    pub item_type: String,
    pub item_id: String,
    pub initial_date_processed: NaiveDateTime,
    pub date_updated: NaiveDateTime,
    pub amt_processed: i32,
}

impl ProcessedItem {
    pub fn get_unique_id(&self) -> String {
        format!("{}.{}", self.item_type, self.item_id)
    }
}

pub async fn process_items(
    db: &PgPool,
    resolver: &impl ResolvingProcessedItems,
) -> Result<(), AppError> {
    let items = resolver.get_processed_items();
    let mut transaction = db.begin().await.expect("Failed to begin transaction");

    for item in items.iter() {
        for id in item.item_ids.iter() {
            upsert_processed_item(&mut transaction, &item.item_type, id).await?;
        }
    }

    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}

async fn upsert_processed_item(
    transaction: &mut Transaction<'_, Postgres>,
    item_type: &str,
    item_id: &str,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
    INSERT INTO processed_items (id, item_type, item_id, date_updated, amt_processed)
    VALUES($1,$2,$3,now(),1)
    ON CONFLICT ON CONSTRAINT processed_items_item_type_item_id_key
        DO UPDATE
        SET date_updated = now(),
            amt_processed = processed_items.amt_processed + 1;
    "#,
        id,
        item_type,
        item_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

pub async fn get_processed_items(
    db: &PgPool,
    items: &[String],
) -> Result<Vec<ProcessedItem>, AppError> {
    let results: Vec<ProcessedItem> = sqlx::query_as!(
        ProcessedItem,
        r#"
    SELECT id, item_type, item_id, initial_date_processed, date_updated, amt_processed
    FROM processed_items
    WHERE CONCAT(item_type, '.', item_id) = ANY ($1);
    "#,
        items
    )
    .fetch_all(db)
    .await?;
    Ok(results)
}
