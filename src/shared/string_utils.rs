use crate::shared::time::local_boise_time;
use chrono::{Datelike, Months, NaiveDate};
use sqlx::types::JsonValue;
use std::collections::HashSet;
use std::ops::{Add, Sub};

/// split text via character to a hashset of strings.
pub fn string_split_hash(text: &str, split_char: char) -> HashSet<String> {
    text.split(split_char)
        .fold(HashSet::<String>::new(), |mut acc, st| {
            acc.insert(st.trim().to_string());
            acc
        })
}

/// map string array to hashset
pub fn string_vec_to_hash(list: &[String]) -> HashSet<String> {
    list.iter().fold(HashSet::new(), |mut acc, st| {
        acc.insert(st.trim().to_string());
        acc
    })
}

pub fn json_value_to_string_vec(data: JsonValue) -> Vec<String> {
    match data {
        JsonValue::Array(list) => list
            .into_iter()
            .filter_map(|v| v.as_str().map(|st| st.to_string()))
            .collect(),
        _ => Vec::new(),
    }
}

/// convert month string to date that represents future month to check against. Helpful for slash command inputs
pub fn map_month_str_to_future_date(month: &str, now: &NaiveDate) -> Option<NaiveDate> {
    let possible_date = format!("{}/{}", now.year(), month);
    NaiveDate::parse_from_str(&possible_date, "%Y/%m/%d")
        .map(|date| {
            if date < *now {
                NaiveDate::from_ymd(date.year() + 1, date.month(), date.day())
            } else {
                date
            }
        })
        .ok()
}

/// string util to resolve range string to tuple. Format: 2023/04/05-2023-05/30
pub fn resolve_date_range(possible_range: &str, default_months_ago: u32) -> (NaiveDate, NaiveDate) {
    let now = local_boise_time().date_naive();
    let thirty_days_ago = now.sub(Months::new(default_months_ago));
    possible_range
        .split_once('-')
        .map(|(start, end)| {
            let date_format = "%Y/%m/%d";
            let start_date =
                NaiveDate::parse_from_str(start, date_format).unwrap_or(thirty_days_ago);
            let end_date = NaiveDate::parse_from_str(end, date_format).unwrap_or(now);

            // if start date is later than end date, return fallback
            if start_date > end_date {
                return (thirty_days_ago, now);
            }
            (start_date, end_date)
        })
        .unwrap_or((thirty_days_ago, now))
}

/// Get date range from string but also do beginning of month for start, and end of month for end.
pub fn floor_ceiling_date_range(
    possible_range: &str,
    default_months_ago: u32,
) -> (NaiveDate, NaiveDate) {
    let (start, end) = resolve_date_range(possible_range, default_months_ago);
    let start = NaiveDate::from_ymd(start.year(), start.month(), 1);
    let end_start = NaiveDate::from_ymd(end.year(), end.month(), 1).add(Months::new(1));
    let end = end_start.pred();
    (start, end)
}

/// map slack id to correct wrapper for being seen as link to user.
pub fn map_slack_id_to_link(slack_id: &str) -> String {
    format!("<@{}>", slack_id)
}

pub fn format_q_line_up_date(date: &NaiveDate) -> String {
    date.format("%m/%d (%a)").to_string()
}

pub fn format_q_empty_row(friendly_date: &str, ao_string: Option<&str>) -> String {
    if let Some(ao) = ao_string {
        format!("`{}` - *{}* - _EMPTY_", friendly_date, ao)
    } else {
        format!("`{}` - _EMPTY_", friendly_date)
    }
}

pub fn map_q_line_up_existing(
    friendly_date: &str,
    ao_string: Option<&str>,
    existing: Vec<String>,
) -> String {
    let line_up = existing
        .iter()
        .map(|item| {
            if item.as_str() == "closed" {
                "_CLOSED_ :x:"
            } else {
                item.as_str()
            }
        })
        .collect::<Vec<&str>>()
        .join(",");
    if let Some(ao) = ao_string {
        format!("`{}` - *{}* - {}", friendly_date, ao, line_up)
    } else {
        format!("`{}` - {}", friendly_date, line_up)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_month_str_letters() {
        let now = NaiveDate::from_ymd(2022, 9, 1);
        let date = map_month_str_to_future_date("11/01", &now);
        assert_eq!(date, Some(NaiveDate::from_ymd(2022, 11, 1)));

        let date = map_month_str_to_future_date("08/15", &now);
        assert_eq!(date, Some(NaiveDate::from_ymd(2023, 8, 15)));
    }

    #[test]
    fn correct_floor_ceiling_date_range() {
        let date_range = "2023/08/15-2023/11/03";
        let (start, end) = floor_ceiling_date_range(date_range, 1);
        assert_eq!(start, NaiveDate::from_ymd(2023, 8, 1));
        assert_eq!(end, NaiveDate::from_ymd(2023, 11, 30));
    }

    #[test]
    fn date_range_fallback_works() {
        let date_range = "2023/08/01-2023/07/01";
        let now = local_boise_time().date_naive();
        let thirty_days_ago = now.sub(Months::new(1));
        let (start, end) = resolve_date_range(date_range, 1);
        assert_eq!(end, now);
        assert_eq!(start, thirty_days_ago);
    }
}
