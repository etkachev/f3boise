use chrono::{Datelike, NaiveDate};
use sqlx::types::JsonValue;
use std::collections::HashSet;

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

/// convert month string to date that represents month to check against. Helpful for slash command inputs
pub fn map_month_str_to_date(month: &str, now: &NaiveDate) -> Option<NaiveDate> {
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
    if let Some(ao) = ao_string {
        format!("`{}` - *{}* - {}", friendly_date, ao, existing.join(","))
    } else {
        format!("`{}` - {}", friendly_date, existing.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_month_str_letters() {
        let now = NaiveDate::from_ymd(2022, 9, 1);
        let date = map_month_str_to_date("11/01", &now);
        assert_eq!(date, Some(NaiveDate::from_ymd(2022, 11, 1)));

        let date = map_month_str_to_date("08/15", &now);
        assert_eq!(date, Some(NaiveDate::from_ymd(2023, 8, 15)));
    }
}
