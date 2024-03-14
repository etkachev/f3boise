use crate::db::queries::users;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::web_api_routes::slash_commands::SlashCommandForm;
use sqlx::PgPool;

/// api on whether pax name is taken or not. Helpful when naming FNGs
pub async fn pax_name_taken(
    db_pool: &PgPool,
    form: &SlashCommandForm,
) -> Result<BlockBuilder, AppError> {
    let name = form.text.as_str();
    let all_pax = users::get_user_name_map(db_pool).await?;
    let already_exists = all_pax.contains_key(name.to_lowercase().as_str());
    let message = if already_exists {
        format!("Sorry, {} is taken", name)
    } else {
        format!("{} is available!", name)
    };
    let block_builder = BlockBuilder::new().section_markdown(&message);
    Ok(block_builder)
}
