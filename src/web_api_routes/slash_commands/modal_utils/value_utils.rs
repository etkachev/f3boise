use crate::app_state::ao_data::AO;
use crate::app_state::equipment::AoEquipment;
use crate::web_api_routes::interactive_events::interaction_payload::BasicValue;
use crate::web_api_routes::slash_commands::modal_utils::BlastWhere;
use chrono::{NaiveDate, NaiveTime};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

/// gets the value via the mapper function you pass in
pub fn get_value<T, F>(values: &HashMap<String, BasicValue>, action_id: &str, f: F) -> Option<T>
where
    F: Fn(&BasicValue) -> T,
{
    values.get(action_id).map(f)
}

pub fn get_single_string(value: &BasicValue) -> String {
    value.get_single().unwrap_or_default()
}

pub fn get_single_date(value: &BasicValue) -> NaiveDate {
    NaiveDate::from_str(value.get_single().unwrap_or_default().as_str()).unwrap_or_default()
}

pub fn get_single_time(value: &BasicValue) -> NaiveTime {
    NaiveTime::parse_from_str(&value.get_single().unwrap_or_default(), "%H:%M").unwrap_or_default()
}

pub fn get_ao_value(value: &BasicValue) -> AO {
    AO::from_channel_id(value.get_single().unwrap_or_default().as_str())
}

/// get hashset of strings from multi value
pub fn get_hash_set_strings_from_multi(value: &BasicValue) -> HashSet<String> {
    value
        .get_multi_value()
        .map(|items| {
            items
                .iter()
                .fold(HashSet::<String>::new(), |mut acc, item| {
                    let item = item.trim();
                    if !item.is_empty() {
                        acc.insert(item.to_string());
                    }
                    acc
                })
        })
        .unwrap_or_default()
}

/// get hashset of strings, splitting comma separated then to hashset
pub fn get_hash_set_from_single_comma_split(value: &BasicValue) -> HashSet<String> {
    value.get_single().unwrap_or_default().split(',').fold(
        HashSet::<String>::new(),
        |mut acc, item| {
            let item = item.trim();
            if !item.is_empty() {
                acc.insert(item.to_string());
            }
            acc
        },
    )
}

pub fn get_blast_where_value(value: &BasicValue) -> BlastWhere {
    BlastWhere::from_str(value.get_single().unwrap_or_default().as_str()).unwrap_or_default()
}

pub fn get_equipment_multi_value(value: &BasicValue) -> HashSet<AoEquipment> {
    value
        .get_multi_value()
        .map(|list| {
            list.iter()
                .fold(HashSet::<AoEquipment>::new(), |mut acc, item| {
                    acc.insert(AoEquipment::from_str(item).unwrap());
                    acc
                })
        })
        .unwrap_or_default()
}
