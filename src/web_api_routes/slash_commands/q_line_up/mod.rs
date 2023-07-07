use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::ao_data::AO;
use crate::db::queries::q_line_up::{get_q_line_up_between_dates_for_ao, QLineUpDbData};
use crate::shared::common_errors::AppError;
use crate::shared::constants::Q_LINE_UP_BTN_TEXT;
use crate::shared::string_utils::{
    format_q_empty_row, format_q_line_up_date, map_month_str_to_future_date,
    map_q_line_up_existing, map_slack_id_to_link,
};
use crate::shared::time::local_boise_time;
use crate::slack_api::block_kit::{BlockBuilder, SectionBlock};
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::web_api_routes::interactive_events::interaction_types::InteractionTypes;
use crate::web_api_routes::interactive_events::q_line_up::utils::get_existing_q_overflow_options;
use crate::web_api_routes::q_line_up::get_line_up_map;
use crate::web_api_state::MutableWebState;
use chrono::{Datelike, Duration, NaiveDate};
use sqlx::PgPool;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Default)]
pub struct QLineUpCommand {
    pub ao: Option<AO>,
    pub month: Option<NaiveDate>,
}

impl From<&str> for QLineUpCommand {
    fn from(text: &str) -> Self {
        let (ao, month) = text.split_once(' ').unwrap_or((text, ""));
        let now = local_boise_time().date_naive();
        let month = map_month_str_to_future_date(month, &now);
        let possible_ao = AO::from(ao.to_string());
        let ao = if let AO::Unknown(_) = possible_ao {
            None
        } else {
            Some(possible_ao)
        };

        QLineUpCommand { ao, month }
    }
}

/// sends message to requestor for q line up for next few days.
pub async fn send_all_q_line_up_message(
    db_pool: &PgPool,
    month_to_check: &NaiveDate,
    users: &HashMap<String, String>,
    channel_id: &str,
    web_app: &MutableWebState,
) -> Result<(), AppError> {
    let message = get_q_line_up_message_all(db_pool, month_to_check, users).await?;
    let request = PostMessageRequest::new(channel_id, message.blocks);
    web_app.post_message(request).await?;
    Ok(())
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

/// send message to requested channel for q line up for an ao.
pub async fn send_ao_q_line_up_message(
    db_pool: &PgPool,
    ao: AO,
    start_date: &NaiveDate,
    users: &HashMap<String, String>,
    channel_id: &str,
    web_app: &MutableWebState,
) -> Result<(), AppError> {
    let message = get_q_line_up_for_ao(db_pool, ao, start_date, users).await?;
    let request = PostMessageRequest::new(channel_id, message.blocks);
    web_app.post_message(request).await?;
    Ok(())
}

/// get q line up for specific ao
pub async fn get_q_line_up_for_ao(
    db_pool: &PgPool,
    ao: AO,
    start_date: &NaiveDate,
    users: &HashMap<String, String>,
) -> Result<BlockBuilder, AppError> {
    let days_ahead: i64 = match ao {
        AO::RuckershipEast | AO::RuckershipWest | AO::CamelsBack => 30,
        _ => 20,
    };
    let end_date = (*start_date)
        .checked_add_signed(Duration::days(days_ahead))
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
            let friendly_date = format_q_line_up_date(&date_to_check);
            let action_combo = InteractionTypes::new_q_line_up(&date_to_check, &ao);
            if let Some(existing) =
                someone_signed_up_for_ao(&existing_line_up, &date_to_check, users)
            {
                // someone signed up
                let text = map_q_line_up_existing(friendly_date.as_str(), None, existing);
                let action_id = action_combo.to_string();
                block_builder.add_section(SectionBlock::new_markdown_with_overflow(
                    text.as_str(),
                    action_id.as_str(),
                    get_existing_q_overflow_options(),
                ));
            } else {
                // no one signed up yet.
                let text = format_q_empty_row(friendly_date.as_str(), None);
                let action_id = action_combo.to_string();
                block_builder.add_section(SectionBlock::new_markdown_with_btn(
                    text.as_str(),
                    Q_LINE_UP_BTN_TEXT,
                    action_id.as_str(),
                ));
            }
        }
        date_to_check = date_to_check.succ();
    }

    block_builder.add_divider();
    block_builder
        .add_section_markdown(get_message_data_text(start_date, &end_date, Some(ao)).as_str());

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
                let friendly_date = format_q_line_up_date(&date_to_check);
                let action_combo = InteractionTypes::new_q_line_up(&date_to_check, &ao);
                if let Some(existing) =
                    someone_signed_up(&existing_line_up, &date_to_check, ao_string.as_str(), users)
                {
                    // someone signed up
                    let text =
                        map_q_line_up_existing(&friendly_date, Some(ao_string.as_str()), existing);
                    let action_id = action_combo.to_string();
                    block_builder.add_section(SectionBlock::new_markdown_with_overflow(
                        text.as_str(),
                        action_id.as_str(),
                        get_existing_q_overflow_options(),
                    ));
                } else {
                    // no one signed up yet.
                    let text = format_q_empty_row(friendly_date.as_str(), Some(ao_string.as_str()));
                    let action_id = action_combo.to_string();
                    block_builder.add_section(SectionBlock::new_markdown_with_btn(
                        text.as_str(),
                        Q_LINE_UP_BTN_TEXT,
                        action_id.as_str(),
                    ));
                }
            }
        }
        date_to_check = date_to_check.succ();
    }

    block_builder.add_divider();
    block_builder.add_section_markdown(get_message_data_text(start_date, &end_date, None).as_str());
    Ok(block_builder)
}

fn get_message_data_text(start_date: &NaiveDate, end_date: &NaiveDate, ao: Option<AO>) -> String {
    let ao_type = ao
        .map(|ao| ao.to_string())
        .unwrap_or_else(|| String::from("all"));
    format!("{}::{}::{}", ao_type, start_date, end_date)
}

/// unwrap message data from text in message block.
pub fn unwrap_message_data(text: &str) -> (NaiveDate, NaiveDate, Option<AO>) {
    let split: Vec<&str> = text.splitn(3, "::").collect();
    if split.len() != 3 {
        return (NaiveDate::MIN, NaiveDate::MIN, None);
    }

    let ao = split[0];
    let start = split[1];
    let end = split[2];

    let start = NaiveDate::from_str(start).unwrap_or_default();
    let end = NaiveDate::from_str(end).unwrap_or_default();
    let ao = match ao {
        "all" => None,
        other => Some(AO::from(other.to_string())),
    };
    (start, end, ao)
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
                        map_slack_id_to_link(slack_id)
                    } else {
                        // if custom text, try to extract f3 names
                        let words: Vec<&str> = q.split_whitespace().collect();
                        map_words_to_slack_names(words, users)
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

fn map_words_to_slack_names(words: Vec<&str>, users: &HashMap<String, String>) -> String {
    let words: Vec<String> = words
        .iter()
        .map(|word| {
            if let Some(slack_id) = users.get(&word.to_string()) {
                map_slack_id_to_link(slack_id)
            } else {
                word.to_string()
            }
        })
        .collect();
    words.join(" ")
}
