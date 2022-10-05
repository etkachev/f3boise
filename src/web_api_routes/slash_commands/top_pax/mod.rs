use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::web_api_routes::back_blast_data::top_pax_per_ao::get_top_pax_per_ao;
use chrono::NaiveDate;
use sqlx::PgPool;
use std::str::FromStr;

pub async fn handle_top_pax(
    db_pool: &PgPool,
    slash_command_args: &str,
) -> Result<BlockBuilder, AppError> {
    let dates = slash_command_args
        .split_once("::")
        .map(
            |(start, end)| match (NaiveDate::from_str(start), NaiveDate::from_str(end)) {
                (Ok(start), Ok(end)) => Some((start, end)),
                _ => None,
            },
        )
        .unwrap_or(None);
    let builder = get_top_pax_per_ao(db_pool, dates).await?;
    Ok(builder)
}
