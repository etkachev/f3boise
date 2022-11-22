use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::recent_bd_for_pax::get_recent_bd_for_pax;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use sqlx::PgPool;

/// get message on where Freighter is at
pub async fn get_wheres_freighter_message(db_pool: &PgPool) -> Result<BlockBuilder, AppError> {
    let most_recent = get_recent_bd_for_pax(db_pool, "freighter").await?;
    if let Some(most_recent) = most_recent {
        let bd = BackBlastData::from(most_recent);
        let text = match bd.ao {
            AO::DR | AO::Unknown(_) => "Who knows :man-shrugging:".to_string(),
            AO::Ruckership => "Somewhere at Ruckership".to_string(),
            ao => format!("Was last seen here {}", ao.google_maps_link()),
        };
        let blocks = BlockBuilder::new()
            .section_markdown(text.as_str())
            .section_markdown(
                format!("on {} :face_with_monocle:", bd.date.format("%b %d, %Y")).as_str(),
            );
        Ok(blocks)
    } else {
        Ok(BlockBuilder::new().section_markdown("Who knows :man-shrugging:"))
    }
}
