use crate::app_state::{
    ao_data::AO,
    backblast_data::{BackBlastData, BACK_BLAST_TAG},
};
use crate::slack_api::channels::list::response::ChannelData;
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::users::f3_user::F3User;
use chrono::NaiveDate;
use std::collections::{HashMap, HashSet};

pub fn parse_back_blast(
    text: &str,
    users: &HashMap<String, F3User>,
    channels: &HashMap<PublicChannels, ChannelData>,
) -> BackBlastData {
    let text = text.trim();
    let mut back_blast = BackBlastData::default();
    for (index, text_line) in text.lines().enumerate() {
        // parse first line for backblast name
        if index == 0 {
            if let Some(stripped) = text_line.to_lowercase().strip_prefix(BACK_BLAST_TAG) {
                let stripped = stripped.trim();
                back_blast.ao = AO::from(stripped.to_string());
            }
        } else {
            let split_line_char = |c| c == ' ' || c == ':' || c == '-';
            let line_parse = text_line.split_once("*:").unwrap_or_else(|| {
                text_line
                    .split_once(split_line_char)
                    .unwrap_or(("", text_line))
            });
            match line_parse {
                ("", rest_of_line) => {
                    // maybe date or time
                    if let Some(date) = parse_date(rest_of_line) {
                        back_blast.date = date;
                        continue;
                    }

                    if let Some(stripped) = rest_of_line.trim().strip_prefix('#') {
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
                (prefix, rest_of_line) => {
                    let prefix_lower = prefix.strip_prefix('*').unwrap_or(prefix).trim();
                    match prefix_lower.to_lowercase().as_str() {
                        "ao" => {
                            let ao = parse_channels_list(rest_of_line, channels);
                            back_blast.ao = ao;
                            continue;
                        }
                        "q" | "qs" if back_blast.qs.is_empty() => {
                            let users = parse_users_list(rest_of_line, users);
                            back_blast.qs = users;
                            continue;
                        }
                        "pax" if !back_blast.has_pax() => {
                            let users = parse_users_list(rest_of_line, users);
                            back_blast.set_pax(users);
                            continue;
                        }
                        "date" => {
                            if let Some(date) = parse_date(rest_of_line) {
                                back_blast.date = date;
                                continue;
                            }
                        }
                        _ => {
                            // maybe date
                            if let Some(date) = parse_date(text_line) {
                                back_blast.date = date;
                                continue;
                            }
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
    let date_parse_formats = ["%m-%d-%y", "%m.%d.%y", "%Y-%m-%d"];
    for date_format in date_parse_formats {
        if let Ok(parsed) = NaiveDate::parse_from_str(date, date_format) {
            return Some(parsed);
        }
    }
    None
}

/// extract id of potential slack user reference
fn extract_slack_user_ref(name: &str) -> String {
    let name = name.strip_prefix('<').unwrap_or(name);
    let name = name.strip_suffix('>').unwrap_or(name);
    let name = name.strip_prefix('@').unwrap_or(name);
    name.to_string()
}

/// extract id of potential slack channel reference
fn extract_slack_channel_ref(channel: &str) -> String {
    let channel = channel.strip_prefix('<').unwrap_or(channel);
    let channel = channel.strip_suffix('>').unwrap_or(channel);
    let channel = channel.strip_prefix('#').unwrap_or(channel);
    // sometimes the format is "<#C03UBFXVBGD|ao-gem>"
    let channel = channel
        .split_once('|')
        .map(|(first, _)| first)
        .unwrap_or(channel);
    channel.to_string()
}

fn parse_users_list(text: &str, users: &HashMap<String, F3User>) -> HashSet<String> {
    let text = text.trim();
    let split_names = text
        .split(|c| c == ' ' || c == ',')
        .into_iter()
        .filter(|c| !c.trim().is_empty())
        .filter(|c| {
            c.starts_with('@')
                || c.starts_with("<@")
                || c.starts_with(|ch: char| ch.is_alphabetic())
        })
        .map(extract_slack_user_ref)
        .fold(HashSet::<String>::new(), |mut acc, name| {
            if let Some(matching_slack_user) = users.get(name.as_str()) {
                acc.insert(matching_slack_user.name.to_string());
            } else {
                let name = name
                    .split_once('(')
                    .map(|(n, _)| n.trim())
                    .unwrap_or_else(|| name.trim());
                acc.insert(name.to_string());
            }
            acc
        });
    split_names
}

fn parse_channels_list(text: &str, channels: &HashMap<PublicChannels, ChannelData>) -> AO {
    let text = text.trim();
    let channel_id = extract_slack_channel_ref(text);
    let ao = channels
        .iter()
        .find_map(|(public_channel, channel_data)| {
            if channel_data.id == channel_id {
                Some(AO::from(public_channel))
            } else {
                None
            }
        })
        .unwrap_or_else(|| AO::Unknown("Unknown".to_string()));
    ao
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn empty_channels() -> HashMap<PublicChannels, ChannelData> {
        HashMap::from([])
    }

    fn hash_set_user(id: &str, name: &str) -> (String, F3User) {
        (
            id.to_string(),
            F3User {
                id: Some(id.to_string()),
                name: name.to_string(),
                email: "email@test.com".to_string(),
            },
        )
    }

    #[test]
    fn variant_one() {
        let text = "#backblast #bleach\n\n08-20-22\n\n0600-0700\n\nQ: @Stinger\n\nPAX: @Timney <@U03SR452HL7>\n\nConditions: nice\n\nFocus on the workout was a modified version of the Elk Fit workout. High intensity with short recovery periods with heavy weights.\n\nWarm up: waterfall, couch stretch, Michael Phelps\n\nThe Thang:\n1 mile run for time.\n\nFirst Phase - Tabata\nBent over row with SB - 20sec x 8 sets (10s rest)\nPush ups with ruck - 20sec x 8 sets (10s rest)\nOverhead press - 20sec x 8 sets (10s rest)\nTricep dips - 20sec x 8 sets (10s rest)\nGood mornings - 20sec x 8 sets (10s rest)\nWeighted sit-ups - 20sec x 8 sets (10s rest)";

        let users = HashMap::<String, F3User>::from([hash_set_user("U03SR452HL7", "Backslash")]);
        let channels = empty_channels();
        let parsed = parse_back_blast(text, &users, &channels);
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
    fn variant_two() {
        let text = "#Backblast #rebel\n8.22.22\nQ: @Slice, @Doppler\nPax: @Daft @Bacon Bacon Bacon @Focker @Lawsuit";
        let users = HashMap::<String, F3User>::from([]);
        let channels = empty_channels();
        let parsed = parse_back_blast(text, &users, &channels);
        assert_eq!(
            parsed,
            BackBlastData::new(
                AO::Rebel,
                HashSet::from(["Slice".to_string(), "Doppler".to_string()]),
                HashSet::from([
                    "Daft".to_string(),
                    "Bacon".to_string(),
                    "Focker".to_string(),
                    "Lawsuit".to_string()
                ]),
                NaiveDate::from_ymd(2022, 8, 22),
            )
        );
    }

    #[test]
    fn bot_bb_variant() {
        let text = "*Slackblast*: \n*Billy Madison*\n*DATE*: 2022-08-22\n*AO*: <#C03UR7GM7Q9>\n*Q*: <@U03SR452HL7>\n*PAX*: <@U03T87KHRFE> , Cliffhanger, Firewall\n*FNGs*: 1 Firewall\n*COUNT*: 3\nDisclaimer \nMission Statement \n5 principles\n\nWarmup:\n10 Grass grabbers IC\n10 Tin soldiers IC\n10 Michael Phelps IC\n\nThang:\nMosey w/ sandbags + rucks to the track\n\nBilly Madison:\n1st grade: 400m mosey + SSH x12\n2nd grade: 400m mosey + Curls (for the girls) x12\n3rd grade: 400m mosey + Merkins x12\n4th grade: 400m mosey + Sandbag Squats x12 5th grade: 400m mosey + Ruck burpees x12\n\nCountarama\n Namearama\n FNG ritual\n Announcements \nTAPS\n\n  \\";
        let users = HashMap::<String, F3User>::from([
            hash_set_user("U03SR452HL7", "Backslash"),
            hash_set_user("U03T87KHRFE", "Stinger"),
        ]);
        let channels = HashMap::<PublicChannels, ChannelData>::from([(
            PublicChannels::Bleach,
            ChannelData {
                id: "C03UR7GM7Q9".to_string(),
                name: "ao-bleach".to_string(),
            },
        )]);
        let parsed = parse_back_blast(text, &users, &channels);
        assert_eq!(
            parsed,
            BackBlastData::new(
                AO::Bleach,
                HashSet::from(["Backslash".to_string()]),
                HashSet::from([
                    "Stinger".to_string(),
                    "Cliffhanger".to_string(),
                    "Firewall".to_string()
                ]),
                NaiveDate::from_ymd(2022, 8, 22),
            )
        );
    }

    #[test]
    fn bot_variant_2() {
        let text = "*Slackblast*: \n*Testing*\n*DATE*: 2022-09-04\n*AO*: <#C03UR7GM7Q9>\n*Q*: <@U03T87KHRFE>, <@U03SR452HL7> \n*PAX*: <@U0410479LG2> <@U040AL30FA8> <@U03SR452HL7> , Retina, Atlas\n*FNGs*: 1 Atlas\n*COUNT*: 5\n\n*WARMUP:* \n*THE THANG:* \n*MARY:* \n*ANNOUNCEMENTS:* \n*COT:* ";
        let users = HashMap::<String, F3User>::from([
            hash_set_user("U03SR452HL7", "Backslash"),
            hash_set_user("U03T87KHRFE", "Stinger"),
            hash_set_user("U040AL30FA8", "Tenor"),
            hash_set_user("U0410479LG2", "Canuck"),
        ]);
        let channels = HashMap::<PublicChannels, ChannelData>::from([(
            PublicChannels::Bleach,
            ChannelData {
                id: "C03UR7GM7Q9".to_string(),
                name: "ao-bleach".to_string(),
            },
        )]);
        let parsed = parse_back_blast(text, &users, &channels);
        let expected = BackBlastData::new(
            AO::Bleach,
            HashSet::from(["Backslash".to_string(), "Stinger".to_string()]),
            HashSet::from([
                "Stinger".to_string(),
                "Backslash".to_string(),
                "Tenor".to_string(),
                "Canuck".to_string(),
                "Retina".to_string(),
                "Atlas".to_string(),
            ]),
            NaiveDate::from_ymd(2022, 9, 4),
        );
        assert_eq!(parsed.ao, expected.ao);
        assert_eq!(parsed.qs, expected.qs);
        assert_eq!(parsed.date, expected.date);
        assert_eq!(parsed.get_pax(), expected.get_pax());
    }

    #[test]
    fn bot_variation_3() {
        let text = "*Slackblast*:\n*Nice and simple BD*\n*DATE*: 2022-09-08\n*AO*: <#C03UBFXVBGD|ao-gem>\n*Q*: <@U041CMSLS2V>\n*PAX*: <@U040X6299FX> <@U041CMSLS2V> <@U040X05NJ85> <@U040QCPQM1V> <@U0405B5P7MK> <@U041H4NEP8R> <@U041E4057RT> <@U041W347TBK> <@U0411DA459U> <@U04173V1E02> <@U0416JH2T36> , others\n*FNGs*: None\n*COUNT*: 18\n\n*WARMUP:*\n*THE THANG:*\nThe Thang\n25 around the world merkins (5 of each - 2 hands on ground, 1 hand on coupon, both hands on coupon, 1 hand on coupon, both hands on ground)\n20 BBSU\n15 curls in cadence\n15 squats in cadence\n10 overhead press\nMosey\nRepeat - I think we did 4 or 5 sets\n*MARY:*\n*ANNOUNCEMENTS:*\n*COT:*";
        let users = HashMap::<String, F3User>::from([
            hash_set_user("U041CMSLS2V", "Samwise"),
            hash_set_user("U040X6299FX", "Chubbs"),
            hash_set_user("U040X05NJ85", "Defrost"),
            hash_set_user("U040QCPQM1V", "Donatello"),
            hash_set_user("U0405B5P7MK", "Slice"),
            hash_set_user("U041H4NEP8R", "Daft"),
            hash_set_user("U041E4057RT", "Watts"),
            hash_set_user("U041W347TBK", "Stone Cold"),
            hash_set_user("U0411DA459U", "Freighter"),
            hash_set_user("U04173V1E02", "Napoleon"),
            hash_set_user("U0416JH2T36", "Gopher"),
        ]);
        let channels = HashMap::<PublicChannels, ChannelData>::from([(
            PublicChannels::Gem,
            ChannelData {
                id: "C03UBFXVBGD".to_string(),
                name: "ao-gem".to_string(),
            },
        )]);
        let parsed = parse_back_blast(text, &users, &channels);
        let expected = BackBlastData::new(
            AO::Gem,
            HashSet::from(["Samwise".to_string()]),
            HashSet::from([
                "Samwise".to_string(),
                "Chubbs".to_string(),
                "Defrost".to_string(),
                "Donatello".to_string(),
                "Slice".to_string(),
                "Daft".to_string(),
                "Watts".to_string(),
                "Stone Cold".to_string(),
                "Freighter".to_string(),
                "Napoleon".to_string(),
                "Gopher".to_string(),
                "others".to_string(),
            ]),
            NaiveDate::from_ymd(2022, 9, 8),
        );

        assert_eq!(parsed.ao, expected.ao);
        assert_eq!(parsed.qs, expected.qs);
        assert_eq!(parsed.date, expected.date);
        assert_eq!(parsed.get_pax(), expected.get_pax());
    }

    #[test]
    fn parse_variant_2_with_futher_q_line() {
        let text = "*Slackblast*: \n*Iron Mountain BD*\n*DATE*: 2022-09-08\n*AO*: <#C03TZV5RRF1>\n*Q*: <@U041ACAGYDC>\n*PAX*: <@U041Z1HFL1F> <@U04133WBEHG> <@U04173A973L>\n*FNGs*: Atlas (@Stinger)\n*COUNT*: 6\n\n*WARMUP:* \n*THE THANG:* \n*MARY:* \n*ANNOUNCEMENTS:* \n*Q:* Duplicate\n*PAX*: Otherfake";
        let users = HashMap::<String, F3User>::from([
            hash_set_user("U041ACAGYDC", "Revere"),
            hash_set_user("U041Z1HFL1F", "Hightower"),
            hash_set_user("U04133WBEHG", "Big Sky"),
            hash_set_user("U04173A973L", "Escobar"),
        ]);
        let channels = HashMap::<PublicChannels, ChannelData>::from([(
            PublicChannels::IronMountain,
            ChannelData {
                id: "C03TZV5RRF1".to_string(),
                name: "ao-iron-mountain".to_string(),
            },
        )]);
        let parsed = parse_back_blast(text, &users, &channels);
        let expected = BackBlastData::new(
            AO::IronMountain,
            HashSet::from(["Revere".to_string()]),
            HashSet::from([
                "Hightower".to_string(),
                "Big Sky".to_string(),
                "Escobar".to_string(),
            ]),
            NaiveDate::from_ymd(2022, 9, 8),
        );
        assert_eq!(parsed.ao, expected.ao);
        assert_eq!(parsed.qs, expected.qs);
        assert_eq!(parsed.date, expected.date);
        assert_eq!(parsed.get_pax(), expected.get_pax());
    }

    #[test]
    fn full_parse_variant_1() {
        let text = "*Slackblast*: \n*Iron Mountain BD test*\n*DATE*: 2022-09-08\n*AO*: <#C03TZV5RRF1>\n*Q*: <@U041ACAGYDC>, <@U040AL30FA8> <@U03T87KHRFE> \n*PAX*: <@U041Z1HFL1F> <@U04133WBEHG> <@U04173A973L> <@U040X7RD605> , Preacherman, Atlas, Test one (@Stinger), Test two (@Gopher)\n*FNGs*: 2 Test one (@Stinger), Test two (@Gopher)\n*COUNT*: 6\n\n*WARMUP:* \n*THE THANG:* \n*MARY:* \n*ANNOUNCEMENTS:* \n*COT:* ";
        let users = HashMap::<String, F3User>::from([
            hash_set_user("U041ACAGYDC", "Revere"),
            hash_set_user("U040AL30FA8", "Canuck"),
            hash_set_user("U03T87KHRFE", "Stinger"),
            hash_set_user("U041Z1HFL1F", "Hightower"),
            hash_set_user("U04133WBEHG", "Big Sky"),
            hash_set_user("U04173A973L", "Escobar"),
            hash_set_user("U040X7RD605", "Telecaster"),
        ]);
        let channels = HashMap::<PublicChannels, ChannelData>::from([(
            PublicChannels::IronMountain,
            ChannelData {
                id: "C03TZV5RRF1".to_string(),
                name: "ao-iron-mountain".to_string(),
            },
        )]);
        let parsed = parse_back_blast(text, &users, &channels);
        let expected = BackBlastData::new(
            AO::IronMountain,
            HashSet::from([
                "Revere".to_string(),
                "Canuck".to_string(),
                "Stinger".to_string(),
            ]),
            HashSet::from([
                "Hightower".to_string(),
                "Big Sky".to_string(),
                "Escobar".to_string(),
                "Telecaster".to_string(),
                "Preacherman".to_string(),
                "Atlas".to_string(),
                "Test".to_string(),
                "two".to_string(),
                "one".to_string(),
            ]),
            NaiveDate::from_ymd(2022, 9, 8),
        );
        assert_eq!(parsed.ao, expected.ao);
        assert_eq!(parsed.qs, expected.qs);
        assert_eq!(parsed.date, expected.date);
        assert_eq!(parsed.get_pax(), expected.get_pax());
    }

    #[test]
    fn parsing_channels_id() {
        let text = "<#C03UR7GM7Q9>";
        let channels = HashMap::<PublicChannels, ChannelData>::from([(
            PublicChannels::Bleach,
            ChannelData {
                id: "C03UR7GM7Q9".to_string(),
                name: "ao-bleach".to_string(),
            },
        )]);
        let ao = parse_channels_list(text, &channels);
        assert_eq!(ao, AO::Bleach);
    }

    #[test]
    fn parse_channel_id_variant_1() {
        let text = "<#C03UBFXVBGD|ao-gem>";
        let channels = HashMap::<PublicChannels, ChannelData>::from([(
            PublicChannels::Gem,
            ChannelData {
                id: "C03UBFXVBGD".to_string(),
                name: "ao-gem".to_string(),
            },
        )]);
        let ao = parse_channels_list(text, &channels);
        assert_eq!(ao, AO::Gem);
    }

    #[test]
    fn parse_year_date() {
        let parsed = parse_date("2022-08-22");
        assert_eq!(parsed, Some(NaiveDate::from_ymd(2022, 8, 22)))
    }

    #[test]
    fn cleaning_names() {
        let cleaned = extract_slack_user_ref("@Timney");
        assert_eq!(cleaned, "Timney".to_string());
    }

    #[test]
    fn user_list_parsing() {
        let text = "@Timney <@U03SR452HL7>";
        let users = HashMap::<String, F3User>::from([hash_set_user("U03SR452HL7", "Backslash")]);
        let parsed = parse_users_list(text, &users);
        assert_eq!(
            parsed,
            HashSet::from(["Timney".to_string(), "Backslash".to_string()])
        );
    }

    #[test]
    fn slack_user_parsing() {
        // TODO need to update slackbot for standardizing format
        let text = "<@U041Z1HFL1F> <@U04133WBEHG> <@U04173A973L> <@U040X7RD605> , Preacherman, Atlas, Test one (@Stinger), Test two (@Gopher)";
        let users = HashMap::<String, F3User>::from([
            hash_set_user("U041Z1HFL1F", "Hightower"),
            hash_set_user("U04133WBEHG", "Big Sky"),
            hash_set_user("U04173A973L", "Telecaster"),
            hash_set_user("U040X7RD605", "Escobar"),
        ]);

        let parsed = parse_users_list(text, &users);
        assert_eq!(
            parsed,
            HashSet::from([
                "Hightower".to_string(),
                "Big Sky".to_string(),
                "Telecaster".to_string(),
                "Escobar".to_string(),
                "Preacherman".to_string(),
                "Atlas".to_string(),
                "Test".to_string(),
                "one".to_string(),
                "two".to_string()
            ])
        )
    }
}
