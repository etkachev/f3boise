use crate::app_state::backblast_data::BackBlastData;
use crate::app_state::MutableAppState;
use crate::db::queries::all_back_blasts::get_list_with_pax;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::web_api_routes::pax_data::PaxInfoResponse;
use crate::web_api_routes::slash_commands::SlashCommandForm;
use sqlx::PgPool;

/// parse user backblast data to pax info (TODO should be converted to sql query)
pub async fn get_user_stats_by_name(
    db_pool: &PgPool,
    user_name: &str,
) -> Result<PaxInfoResponse, AppError> {
    let list = get_list_with_pax(db_pool, user_name).await?;
    let response = list.into_iter().map(BackBlastData::from).fold(
        PaxInfoResponse::new(user_name),
        |mut acc, item| {
            acc.favorite_ao.for_ao(&item.ao);
            acc.post_count += 1;
            if item.qs.contains(user_name) {
                acc.q_count += 1;
            }

            if item.date < acc.start_date {
                acc.start_date = item.date;
            }
            acc
        },
    );
    Ok(response)
}

/// handle getting response for my stats.
pub async fn handle_my_stats(
    db_pool: &PgPool,
    app_state: &MutableAppState,
    form: &SlashCommandForm,
) -> Result<BlockBuilder, AppError> {
    let user_name = {
        let app = app_state.app.lock().expect("Could not lock app");
        app.users
            .get(&form.user_id)
            .map(|user| user.name.to_string())
    };

    if user_name.is_none() {
        return Err(AppError::General("User not found".to_string()));
    }

    let user_name = user_name.unwrap();

    let response = get_user_stats_by_name(db_pool, user_name.as_str()).await?;
    let block_builder = BlockBuilder::new()
        .add_section_markdown(format!("*Here are your stats {}:*", response.name).as_str())
        .add_section_markdown(format!("*Total Posts*: {}", response.post_count).as_str())
        .add_section_markdown(format!("*Q Posts*: {}", response.q_count).as_str())
        .add_section_markdown(
            format!("*Favorite AO*: {}", response.favorite_ao.favorite()).as_str(),
        )
        .add_section_markdown(format!("*First F3 Boise post*: {:?}", response.start_date).as_str());

    Ok(block_builder)
}
