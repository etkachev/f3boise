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
