use crate::app_state::ao_data::AO;
use crate::db::queries::all_back_blasts::back_blasts_by_ao::back_blasts_by_channel_id;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::web_api_routes::back_blast_data::ao_back_blast_stats::AOBackBlastsStats;
use crate::web_api_routes::slash_commands::SlashCommandForm;
use sqlx::PgPool;

/// get stats for an AO from slash command
pub async fn get_ao_stats_block(
    db_pool: &PgPool,
    form: &SlashCommandForm,
) -> Result<BlockBuilder, AppError> {
    let possible_ao = AO::from_channel_id(form.channel_id.as_str());
    if matches!(possible_ao, AO::Unknown(_) | AO::DR) {
        return Ok(error_block(":warning: Must be called within an AO channel"));
    }

    let results = back_blasts_by_channel_id(db_pool, possible_ao.channel_id()).await?;
    let stats = AOBackBlastsStats::new(results);

    let block_builder = BlockBuilder::new()
        .section_markdown(format!("*Stats for {}*", possible_ao).as_str())
        .section_markdown(format!("*Total BDs*: {}", stats.total).as_str())
        .section_markdown(format!("*Total unique pax*: {}", stats.unique_pax).as_str())
        .section_markdown(format!("*Pax post per BD*: {:.2}", stats.avg_pax_per_bd).as_str());

    Ok(block_builder)
}

fn error_block(msg: &str) -> BlockBuilder {
    BlockBuilder::new().section_markdown(msg)
}
