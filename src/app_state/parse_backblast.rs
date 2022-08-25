use crate::app_state::{
    ao_data::AO,
    backblast_data::{BackBlastData, BACK_BLAST_TAG},
};
use crate::users::f3_user::F3User;
use chrono::NaiveDate;
use std::collections::{HashMap, HashSet};

pub fn parse_back_blast(text: &str, users: &HashMap<String, F3User>) -> BackBlastData {
    let text = text.trim();
    let mut back_blast = BackBlastData::default();
    for (index, text_line) in text.lines().enumerate() {
        // parse first line for backblast name
        if index == 0 {
            if let Some(stripped) = text_line.strip_prefix(BACK_BLAST_TAG) {
                let stripped = stripped.trim();
                back_blast.ao = AO::from(stripped.to_string());
            }
        } else {
            // parse everything else
            if let Some((prefix, rest_of_line)) =
                text_line.split_once(|c| c == ' ' || c == ':' || c == '-')
            {
                match prefix.trim().to_lowercase().as_str() {
                    "q" | "qs" => {
                        let users = parse_users_list(rest_of_line, users);
                        back_blast.qs = users;
                        continue;
                    }
                    "pax" => {
                        let users = parse_users_list(rest_of_line, users);
                        back_blast.set_pax(users);
                        continue;
                    }
                    _ => {
                        // maybe date
                        if let Some(date) = parse_date(text_line) {
                            back_blast.date = date;
                            continue;
                        }
                    }
                }
            } else {
                // maybe date or time
                if let Some(date) = parse_date(text_line) {
                    back_blast.date = date;
                    continue;
                }

                if let Some(stripped) = text_line.trim().strip_prefix('#') {
                    let possible_ao = AO::from(stripped.to_string());
                    match possible_ao {
                        AO::Unknown(name) if name == "EMPTY" => {}
                        _ => {
                            back_blast.ao = possible_ao;
                            continue;
                        }
                    }
                }
            }
        }
    }

    back_blast
}

fn parse_date(date: &str) -> Option<NaiveDate> {
    let date = date.trim();
    if let Ok(parsed) = NaiveDate::parse_from_str(date, "%m-%d-%y") {
        return Some(parsed);
    }

    if let Ok(parsed) = NaiveDate::parse_from_str(date, "%m.%d.%y") {
        return Some(parsed);
    }
    None
}

fn clean_name(name: &str) -> String {
    let name = name.strip_prefix('<').unwrap_or_else(|| name);
    let name = name.strip_suffix('>').unwrap_or_else(|| name);
    let name = name.strip_prefix('@').unwrap_or_else(|| name);
    name.to_string()
}

fn parse_users_list(text: &str, users: &HashMap<String, F3User>) -> HashSet<String> {
    let text = text.trim();
    let split_names = text
        .split(|c| c == ' ' || c == ',')
        .into_iter()
        .filter(|c| !c.trim().is_empty())
        .filter(|c| c.starts_with("@") || c.starts_with("<@"))
        .map(|name| clean_name(name))
        .fold(HashSet::<String>::new(), |mut acc, name| {
            if let Some(matching_slack_user) = users.get(name.as_str()) {
                acc.insert(matching_slack_user.name.to_string());
            } else {
                acc.insert(name.to_string());
            }
            acc
        });
    split_names
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn variant_one() {
        let text = "#backblast #bleach\n\n08-20-22\n\n0600-0700\n\nQ: Stinger\n\nPAX: @Timney <@U03SR452HL7>\n\nConditions: nice\n\nFocus on the workout was a modified version of the Elk Fit workout. High intensity with short recovery periods with heavy weights.\n\nWarm up: waterfall, couch stretch, Michael Phelps\n\nThe Thang:\n1 mile run for time.\n\nFirst Phase - Tabata\nBent over row with SB - 20sec x 8 sets (10s rest)\nPush ups with ruck - 20sec x 8 sets (10s rest)\nOverhead press - 20sec x 8 sets (10s rest)\nTricep dips - 20sec x 8 sets (10s rest)\nGood mornings - 20sec x 8 sets (10s rest)\nWeighted sit-ups - 20sec x 8 sets (10s rest)";

        let users = HashMap::<String, F3User>::from([(
            "U03SR452HL7".to_string(),
            F3User {
                id: Some("U03SR452HL7".to_string()),
                name: "Backslash".to_string(),
                email: "backslash@gmail.com".to_string(),
            },
        )]);
        let parsed = parse_back_blast(text, &users);
        assert_eq!(
            parsed,
            BackBlastData::new(
                AO::Bleach,
                HashSet::from(["Stinger".to_string()]),
                HashSet::from(["Timney".to_string(), "Backslash".to_string()]),
                NaiveDate::from_ymd(2022, 8, 20),
            )
        );
    }

    #[test]
    fn cleaning_names() {
        let cleaned = clean_name("@Timney");
        assert_eq!(cleaned, "Timney".to_string());
    }

    #[test]
    fn user_list_parsing() {
        let text = "@Timney <@U03SR452HL7>";
        let users = HashMap::<String, F3User>::from([(
            "U03SR452HL7".to_string(),
            F3User {
                id: Some("U03SR452HL7".to_string()),
                name: "Backslash".to_string(),
                email: "backslash@gmail.com".to_string(),
            },
        )]);
        let parsed = parse_users_list(text, &users);
        assert_eq!(
            parsed,
            HashSet::from(["Timney".to_string(), "Backslash".to_string()])
        );
    }
}
