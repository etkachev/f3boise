use crate::db::pax_parent_tree::{F3Parent, ParentPaxRelation};
use crate::shared::common_errors::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct ParentPaxRelationDbItem {
    /// f3 name of pax
    pub pax_name: String,
    /// optional slack id of pax
    pub slack_id: Option<String>,
    /// how this pax got to F3
    pub parent: serde_json::Value,
}

impl TryFrom<ParentPaxRelationDbItem> for ParentPaxRelation {
    type Error = AppError;

    fn try_from(value: ParentPaxRelationDbItem) -> Result<Self, Self::Error> {
        let parent = serde_json::from_value::<F3Parent>(value.parent)?;
        Ok(ParentPaxRelation {
            pax_name: value.pax_name.to_string(),
            slack_id: value.slack_id.clone(),
            parent,
        })
    }
}

/// raw entries from db for pax parent relationships
pub async fn get_pax_parent_relationship_entries(
    db_pool: &PgPool,
) -> Result<Vec<ParentPaxRelationDbItem>, AppError> {
    let rows: Vec<ParentPaxRelationDbItem> = sqlx::query_as!(
        ParentPaxRelationDbItem,
        r#"
        SELECT slack_id, pax_name, parent
        FROM parent_pax_relationships;
    "#
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

/// get mapped list of pax parent relationships
pub async fn get_pax_parent_relationships(
    db_pool: &PgPool,
) -> Result<Vec<ParentPaxRelation>, AppError> {
    let rows = get_pax_parent_relationship_entries(db_pool).await?;

    let mapped: Vec<ParentPaxRelation> = rows
        .into_iter()
        .filter_map(|item| ParentPaxRelation::try_from(item).ok())
        .collect();

    Ok(mapped)
}
