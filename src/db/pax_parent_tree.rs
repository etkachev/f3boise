use crate::shared::common_errors::AppError;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

/// Upsert pax to parent relationship
pub async fn upsert_pax_parent_relationship(
    transaction: &mut Transaction<'_, Postgres>,
    relationship: &ParentPaxRelation,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    let parent_json = serde_json::to_value(relationship.parent.clone()).unwrap_or_default();
    sqlx::query!(
        r#"
    INSERT INTO parent_pax_relationships (id, pax_name, slack_id, parent)
    VALUES($1,$2,$3,$4)
    ON CONFLICT (pax_name)
        DO UPDATE
        SET slack_id = EXCLUDED.slack_id,
            parent = EXCLUDED.parent;
    "#,
        id,
        relationship.pax_name,
        relationship.slack_id,
        parent_json,
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

/// db representation of parent pax relationship row.
#[derive(Serialize, Deserialize)]
pub struct ParentPaxRelation {
    /// f3 name of pax
    pub pax_name: String,
    /// optional slack id of pax
    pub slack_id: Option<String>,
    /// how this pax got to F3
    pub parent: F3Parent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum F3Parent {
    Pax(ParentPax),
    AtBd,
    DrEh,
    Moved,
    Online,
}

impl F3Parent {
    pub fn new_pax(name: &str, slack_id: Option<String>) -> Self {
        F3Parent::Pax(ParentPax {
            name: name.to_string(),
            slack_id,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParentPax {
    name: String,
    slack_id: Option<String>,
}
