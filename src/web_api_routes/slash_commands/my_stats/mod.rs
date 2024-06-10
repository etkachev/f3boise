use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::{
    get_dd_list_with_pax, get_list_with_pax, BackBlastJsonData,
};
use crate::db::queries::users::get_user_by_slack_id;
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
    let dd = get_dd_list_with_pax(db_pool, user_name).await?;
    let response = get_pax_info_from_bb_data(&list, &dd, user_name);
    Ok(response)
}

pub fn get_pax_info_from_bb_data(
    list: &[BackBlastJsonData],
    dd: &[BackBlastJsonData],
    user_name: &str,
) -> PaxInfoResponse {
    let mut result = list.iter().map(BackBlastData::from).fold(
        PaxInfoResponse::new(user_name),
        |mut acc, item| {
            acc.favorite_ao.for_ao(&item.ao);
            acc.post_count += 1;
            if item.qs.contains(&user_name.to_lowercase()) {
                acc.q_count += 1;
            }

            if item.date < acc.start_date {
                acc.start_date = item.date;
            }
            acc
        },
    );

    result.with_dd(dd);

    result
}

/// handle getting response for my stats.
pub async fn handle_my_stats(
    db_pool: &PgPool,
    form: &SlashCommandForm,
) -> Result<BlockBuilder, AppError> {
    let user_name = get_user_by_slack_id(db_pool, &form.user_id)
        .await
        .unwrap_or_default()
        .map(|user| user.name);

    if user_name.is_none() {
        return Err(AppError::General("User not found".to_string()));
    }

    let user_name = user_name.unwrap();

    let response = get_user_stats_by_name(db_pool, user_name.as_str()).await?;
    let block_builder = BlockBuilder::new()
        .section_markdown(format!("*Here are your stats {}:*", response.name).as_str())
        .section_markdown(format!("*Total Posts*: {}", response.post_count).as_str())
        .section_markdown(format!("*Q Posts*: {}", response.q_count).as_str())
        .section_markdown(format!("*Favorite AO*: {}", response.favorite_ao.favorite()).as_str())
        .section_markdown(format!("*First F3 Boise post*: {:?}", response.start_date).as_str())
        .section_markdown(
            format!(
                "*{} DD Posts*: {}",
                response.current_dd_program, response.dd_count
            )
            .as_str(),
        );

    Ok(block_builder)
}
