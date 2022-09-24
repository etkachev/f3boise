use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::ao_data::AO;
use crate::db::queries::q_line_up::{
    get_q_line_up_between_dates, get_q_line_up_between_dates_for_ao, QLineUpDbData,
};
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::{BlockBuilder, SectionBlock};
use crate::web_api_routes::interactive_events::interaction_types::ActionComboData;
use chrono::{Datelike, Duration, NaiveDate};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Default)]
pub struct QLineUpCommand {
    pub ao: Option<AO>,
    pub month: Option<u16>,
}

impl From<&str> for QLineUpCommand {
    fn from(text: &str) -> Self {
        let (ao, _month) = text.split_once("::").unwrap_or((text, ""));
        let possible_ao = AO::from(ao.to_string());
        let ao = if let AO::Unknown(_) = possible_ao {
            None
        } else {
            Some(possible_ao)
        };

        QLineUpCommand {
            ao,
            // TODO month
            month: None,
        }
    }
}

/// for all ao's get q line up for next few days
pub async fn get_q_line_up_message_all(
    db_pool: &PgPool,
    month_to_check: &NaiveDate,
    users: &HashMap<String, String>,
) -> Result<BlockBuilder, AppError> {
    let end_date = (*month_to_check)
        .checked_add_signed(Duration::days(5))
        .unwrap_or_else(|| (*month_to_check).succ());
    let result = get_q_line_up_for_range(db_pool, month_to_check, end_date, users).await?;
    Ok(result)
}

pub async fn get_q_line_up_for_ao(
    db_pool: &PgPool,
    ao: AO,
    start_date: &NaiveDate,
    users: &HashMap<String, String>,
) -> Result<BlockBuilder, AppError> {
    let end_date = (*start_date)
        .checked_add_signed(Duration::days(20))
        .unwrap_or_else(|| (*start_date).succ());
    let result = get_q_line_up_for_range_for_ao(db_pool, ao, start_date, end_date, users).await?;
    Ok(result)
}

async fn get_q_line_up_for_range_for_ao(
    db_pool: &PgPool,
    ao: AO,
    start_date: &NaiveDate,
    end_date: NaiveDate,
    users: &HashMap<String, String>,
) -> Result<BlockBuilder, AppError> {
    let month_display = start_date.format("%B %Y").to_string();
    let mut block_builder = BlockBuilder::new()
        .header(format!(":calendar: Q Line-up for {} :calendar:", ao.to_string()).as_str())
        .context(format!("*{}*  |  Fill em up!", month_display).as_str())
        .divider();

    let existing_line_up =
        get_q_line_up_between_dates_for_ao(db_pool, &ao, start_date, &end_date).await?;
    let mut date_to_check = (*start_date).succ();

    while date_to_check < end_date && !block_builder.reached_max() {
        if ao.week_days().contains(&date_to_check.weekday()) {
            let friendly_date = date_to_check.format("%m/%d (%a)").to_string();
            if let Some(existing) =
                someone_signed_up_for_ao(&existing_line_up, &date_to_check, users)
            {
                // someone signed up
                block_builder.add_section_markdown(
                    format!("`{}` - {}", friendly_date, existing.join(",")).as_str(),
                );
            } else {
                // no one signed up yet.
                let text = format!("`{}` - _EMPTY_", friendly_date);
                let action_combo = ActionComboData::new_q_line_up(&date_to_check, &ao);
                let action_id = action_combo.to_string();
                block_builder.add_section(SectionBlock::new_markdown_with_btn(
                    text.as_str(),
                    "Sign up",
                    action_id.as_str(),
                ));
            }
        }
        date_to_check = date_to_check.succ();
    }

    Ok(block_builder)
}

async fn get_q_line_up_for_range(
    db_pool: &PgPool,
    start_date: &NaiveDate,
    end_date: NaiveDate,
    users: &HashMap<String, String>,
) -> Result<BlockBuilder, AppError> {
    let month_display = start_date.format("%B %Y").to_string();
    let mut block_builder = BlockBuilder::new()
        .header(":calendar: Q Line-up :calendar:")
        .context(format!("*{}*  |  Fill em up!", month_display).as_str())
        .divider();

    let existing_line_up = get_line_up_map(db_pool, start_date, &end_date).await?;

    let mut date_to_check = (*start_date).succ();

    while date_to_check < end_date && !block_builder.reached_max() {
        for ao in AO_LIST {
            if block_builder.reached_max() {
                break;
            }
            let ao_string = ao.to_string();
            if ao.week_days().contains(&date_to_check.weekday()) {
                let friendly_date = date_to_check.format("%m/%d (%a)").to_string();
                if let Some(existing) =
                    someone_signed_up(&existing_line_up, &date_to_check, ao_string.as_str(), users)
                {
                    // someone signed up
                    block_builder.add_section_markdown(
                        format!(
                            "`{}` - *{}* - {}",
                            friendly_date,
                            ao_string,
                            existing.join(",")
                        )
                        .as_str(),
                    );
                } else {
                    // no one signed up yet.
                    let text = format!("`{}` - *{}*", friendly_date, ao_string);
                    let action_combo = ActionComboData::new_q_line_up(&date_to_check, &ao);
                    let action_id = action_combo.to_string();
                    block_builder.add_section(SectionBlock::new_markdown_with_btn(
                        text.as_str(),
                        "Sign up",
                        action_id.as_str(),
                    ));
                }
            }
        }
        date_to_check = date_to_check.succ();
    }

    block_builder.add_divider();
    Ok(block_builder)
}

fn someone_signed_up(
    full_list: &HashMap<String, Vec<QLineUpDbData>>,
    date: &NaiveDate,
    ao: &str,
    users: &HashMap<String, String>,
) -> Option<Vec<String>> {
    match full_list.get(ao) {
        Some(list) => someone_signed_up_for_ao(list, date, users),
        None => None,
    }
}

fn someone_signed_up_for_ao(
    ao_line_up: &[QLineUpDbData],
    date: &NaiveDate,
    users: &HashMap<String, String>,
) -> Option<Vec<String>> {
    let taken = ao_line_up.iter().find_map(|item| {
        if &item.date == date {
            let qs: Vec<String> = item
                .qs
                .iter()
                .map(|q| {
                    if let Some(slack_id) = users.get(q.trim()) {
                        format!("<@{}>", slack_id)
                    } else {
                        q.trim().to_string()
                    }
                })
                .collect();
            Some(qs)
        } else {
            None
        }
    });
    taken
}

async fn get_line_up_map(
    db_pool: &PgPool,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<HashMap<String, Vec<QLineUpDbData>>, AppError> {
    let all = get_q_line_up_between_dates(db_pool, start_date, end_date).await?;
    let results = all.into_iter().fold(
        HashMap::<String, Vec<QLineUpDbData>>::new(),
        |mut acc, item| {
            if let Some(existing) = acc.get_mut(item.ao.as_str()) {
                existing.push(item);
            } else {
                acc.insert(item.ao.to_string(), vec![item]);
            }
            acc
        },
    );
    Ok(results)
}
