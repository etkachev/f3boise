use crate::db::queries::all_back_blasts::BackBlastJsonData;
use crate::shared::common_errors::AppError;
use sqlx::PgPool;

/// get most recent BD for particular pax (name passed in) is included.
pub async fn get_recent_bd_for_pax(
    db_pool: &PgPool,
    name: &str,
) -> Result<Option<BackBlastJsonData>, AppError> {
    let name = name.to_lowercase();
    let result: Option<BackBlastJsonData> = sqlx::query_as!(
        BackBlastJsonData,
        r#"
    WITH list_view AS (
        SELECT
            bb.id,
            al.name as ao,
            string_to_array(lower(q), ',') as q,
            string_to_array(lower(pax), ',') as pax,
            date,
            bb_type,
            bb.channel_id
        FROM back_blasts bb
        INNER JOIN ao_list al on bb.channel_id = al.channel_id
        WHERE bb.bb_type = 'backblast' AND bb.active = true
    )
    
    SELECT id, ao, channel_id, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    WHERE pax @> array[$1]
    ORDER BY date DESC
    LIMIT 1;
    "#,
        name
    )
    .fetch_optional(db_pool)
    .await?;
    Ok(result)
}
