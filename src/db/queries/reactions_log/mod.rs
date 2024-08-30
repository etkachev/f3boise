use crate::db::save_reaction_log::ReactionLogDbItem;
use crate::shared::common_errors::AppError;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;
use uuid::Uuid;

/// download all reaction log items
pub async fn get_full_reaction_log(db: &PgPool) -> Result<Vec<ReactionLogDbItem>, AppError> {
    let results: Vec<ReactionLogDbItem> = sqlx::query_as!(
        ReactionLogDbItem,
        r#"
        SELECT id,
               entity_type,
               entity_id,
               reaction,
               slack_user,
               reaction_added,
               reaction_timestamp
        FROM reactions_log
        ORDER BY reaction_timestamp DESC;
    "#
    )
    .fetch_all(db)
    .await?;

    Ok(results)
}

pub struct PreBlastReactionLogItem {
    pub entity_id: Uuid,
    pub channel_id: String,
    pub slack_user: String,
    pub name: String,
    pub reaction: String,
    pub first_added_time: Option<NaiveDateTime>,
    pub final_reaction_status: bool,
    pub last_removal_time: Option<NaiveDateTime>,
}

/// get pre_blast reaction data based on date mentioned on pre-blast and list of reactions to look for.
pub async fn get_pre_blast_reaction_data(
    db: &PgPool,
    date: NaiveDate,
    reactions: Vec<String>,
) -> Result<Vec<PreBlastReactionLogItem>, AppError> {
    let results: Vec<PreBlastReactionLogItem> = sqlx::query_as!(PreBlastReactionLogItem, r#"
    WITH reactions_list AS (
    SELECT rl.reaction,
           rl.reaction_added,
           u.name,
           rl.slack_user,
           rl.entity_type,
           rl.entity_id,
           rl.reaction_timestamp
        FROM reactions_log rl
            INNER JOIN users u ON u.slack_id = rl.slack_user
        WHERE rl.entity_type = 'pre_blast'
),
reaction_statuses AS (
    SELECT entity_id,
           slack_user,
           reaction,
           MIN(reaction_timestamp) FILTER (WHERE reaction_added = TRUE) AS first_added_time,
           CASE WHEN SUM(CASE WHEN reaction_added THEN 1 ELSE -1 END) > 0 THEN TRUE ELSE FALSE END AS final_reaction_status,
           MAX(reaction_timestamp) FILTER (WHERE reaction_added = FALSE) as last_removal_time
    FROM reactions_list
    GROUP BY entity_id, slack_user, reaction
)

SELECT rs.entity_id,
       pb.channel_id,
       rs.slack_user,
       u.name,
       rs.reaction,
       rs.first_added_time AT TIME ZONE 'UTC' AT TIME ZONE 'America/Boise' AS first_added_time,
       rs.final_reaction_status as "final_reaction_status!",
       rs.last_removal_time AT TIME ZONE 'UTC' AT TIME ZONE 'America/Boise' AS last_removal_time
FROM reaction_statuses rs
         INNER JOIN users u ON u.slack_id = rs.slack_user
        INNER JOIN pre_blasts pb ON pb.id = rs.entity_id
WHERE pb.date = $1 AND rs.reaction = ANY($2)
ORDER BY rs.entity_id, rs.slack_user, rs.reaction;
    "#, date, &reactions).fetch_all(db).await?;

    Ok(results)
}
