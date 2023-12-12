use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::{BackBlastData, BackBlastType};
use crate::db::queries::all_back_blasts::recent_bd_for_pax::get_recent_bd_for_pax;
use crate::db::queries::all_back_blasts::{
    get_all_dd_within_date_range, get_all_within_date_range,
};
use crate::db::queries::users::get_slack_id_map;
use crate::db::save_q_line_up::NewQLineUpDbEntry;
use crate::db::{save_back_blast, save_q_line_up};
use crate::shared::common_errors::AppError;
use crate::shared::string_utils::string_split_hash;
use crate::shared::time::local_boise_time;
use crate::slack_api::channels::history::request::ChannelHistoryRequest;
use crate::slack_api::channels::kick::request::KickFromChannelRequest;
use crate::web_api_run::init_web_state;
use chrono::{Months, NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashSet;
use std::ops::Sub;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct OldBackBlast {
    pub date: String,
    pub q: String,
    pub count: u16,
    pub fngs: Option<u16>,
    pub pax: String,
}

#[derive(Serialize, Deserialize)]
struct OldQSheetRow {
    pub date: String,
    pub gem: Option<String>,
    pub oldglory: Option<String>,
    pub backyard: Option<String>,
    pub rebel: Option<String>,
    pub bleach: Option<String>,
    pub ruckership: Option<String>,
    pub ironmountain: Option<String>,
    pub rise: Option<String>,
    pub lakeview_park: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ProdCSVEntry {
    pub ao: String,
    pub q: String,
    pub pax: String,
    pub date: String,
    pub fngs: Option<String>,
    pub title: Option<String>,
    pub moleskine: Option<String>,
    pub bb_type: Option<String>,
}

const BACK_YARD_BB_PATH: &str = "migration_files/old/Backyard";
const BLEACH_BB_PATH: &str = "migration_files/old/Bleach";
const GEM_BB_PATH: &str = "migration_files/old/Gem";
const IR_BB_PATH: &str = "migration_files/old/Iron Mountain";
const OLD_GLORY_BB_PATH: &str = "migration_files/old/Old Glory";
const REBEL_BB_PATH: &str = "migration_files/old/Rebel";
const RUCKERSHIP_BB_PATH: &str = "migration_files/old/Ruckership";
pub const AOLIST: [(AO, &str); 7] = [
    (AO::Backyard, BACK_YARD_BB_PATH),
    (AO::Bleach, BLEACH_BB_PATH),
    (AO::Gem, GEM_BB_PATH),
    (AO::IronMountain, IR_BB_PATH),
    (AO::OldGlory, OLD_GLORY_BB_PATH),
    (AO::Rebel, REBEL_BB_PATH),
    (AO::RuckershipWest, RUCKERSHIP_BB_PATH),
];

pub async fn save_old_back_blasts(db_pool: &PgPool) -> Result<(), AppError> {
    for (ao, file_path) in AOLIST.iter() {
        let ao_name = ao.to_string();
        let bb = read_back_blasts(ao, &back_blast_path(file_path))?;
        save_back_blast::save_multiple(db_pool, &bb).await?;
        println!("Saved: {}", ao_name);
    }

    Ok(())
}

/// v2 sync method to sync prod db to local
pub async fn sync_prod_db(db_pool: &PgPool) -> Result<(), AppError> {
    let bb = read_back_blast_csv()?;
    save_back_blast::save_multiple(db_pool, &bb).await?;
    println!("Saved all");
    Ok(())
}

pub async fn cleanup_pax_in_channels(db_pool: &PgPool) -> Result<(), AppError> {
    let pax = get_slack_id_map(db_pool).await?;
    let now = local_boise_time().date_naive();
    let ninety_days_ago = now.sub(Months::new(3));
    let (start, end) = (ninety_days_ago, now);

    let bds = get_all_within_date_range(db_pool, &start, &end)
        .await
        .unwrap_or_default();
    let dd = get_all_dd_within_date_range(db_pool, &start, &end)
        .await
        .unwrap_or_default();

    dotenvy::dotenv().ok();
    let api = init_web_state();

    let ninety_days_ts = NaiveDateTime::new(ninety_days_ago, NaiveTime::default());
    println!("90 days: {:?}", ninety_days_ts);
    for ao in AO_LIST {
        println!("Checking {}", ao.to_string());

        let users_in_channel = api
            .get_channel_members(ao.channel_id())
            .await
            .unwrap_or_default();

        if users_in_channel.is_empty() {
            println!("======");
            println!("NO users!!!");
            println!("======");
        }

        let request = ChannelHistoryRequest::new(ao.channel_id())
            .with_limit(1000)
            .with_oldest(ninety_days_ts);

        match api.get_history(request).await {
            Ok(history) => {
                if let Some(messages) = history.messages {
                    let mut active_users = HashSet::<String>::new();

                    for message in messages {
                        if let Some(user) = message.user {
                            active_users.insert(user.to_string());
                        }
                    }

                    println!("active messages: {}", active_users.len());

                    for pax_id in users_in_channel {
                        if active_users.contains(&pax_id) {
                            // they messaged, so continue to next pax.
                            // println!("====");
                            // println!("{} has messaged to {}", pax_id, ao.to_string());
                            // println!("====");
                            continue;
                        }

                        // now check if they posted at bd or dd.
                        if let Some(pax_name) = pax.get(&pax_id) {
                            let pax_name = pax_name.to_lowercase();
                            let most_recent_bd = get_recent_bd_for_pax(db_pool, &pax_name)
                                .await
                                .unwrap_or_default();
                            if let Some(most_recent_bd) = most_recent_bd {
                                let bd = BackBlastData::from(most_recent_bd);
                                if bd.ao == ao {
                                    println!(
                                        "This is {pax_name}'s most recent bd at {}",
                                        ao.to_string()
                                    );
                                    continue;
                                }
                            }
                            // bd
                            let attended_bds = bds
                                .iter()
                                .filter(|bd| {
                                    let bd = BackBlastData::from(*bd);

                                    bd.ao == ao && bd.includes_pax(&pax_name)
                                })
                                .count();

                            // dd
                            let attended_dds = dd
                                .iter()
                                .filter(|data| {
                                    let dd_data = BackBlastData::from(*data);
                                    dd_data.ao == ao && dd_data.includes_pax(&pax_name)
                                })
                                .count();

                            if attended_bds == 0 && attended_dds == 0 {
                                // kick them out.
                                let request =
                                    KickFromChannelRequest::new(pax_id.as_str(), ao.channel_id());
                                println!("{} - {:?}", pax_name, request);
                                match api.kick_user_from_channel(request).await {
                                    Ok(_) => {
                                        // println!()
                                    }
                                    Err(err) => {
                                        println!("Error kicking: {:?}", err);
                                    }
                                }
                            } else {
                                println!("====");
                                println!("{} has been to {}", pax_name, ao.to_string());
                                println!("====");
                            }
                        } else {
                            println!("Unknown pax: {}", pax_id);
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    }
                } else {
                    println!("======");
                    println!("NO MESSAGES!!!");
                    println!("======");
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

        println!("Finished with {}", ao.to_string());
        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    }

    println!("Cleaned up!");
    Ok(())
}

fn read_back_blast_csv() -> Result<Vec<BackBlastData>, AppError> {
    let mut rdr = csv::ReaderBuilder::new().from_path("migration_files/prod_db/f3-boise.csv")?;
    let mut results: Vec<BackBlastData> = vec![];
    for record in rdr.deserialize() {
        let record: ProdCSVEntry = record?;
        let date = NaiveDate::parse_from_str(record.date.as_str(), "%Y-%m-%d").unwrap();
        let ao = AO::from(record.ao.to_string());
        let qs = string_split_hash(record.q.as_str(), ',');
        let pax = string_split_hash(record.pax.as_str(), ',');
        let mut mapped = BackBlastData::new(ao, qs, pax, date);
        mapped.title = record.title.clone();
        mapped.moleskine = record.moleskine.clone();
        mapped.fngs = string_split_hash(record.fngs.unwrap_or_default().as_str(), ',');
        mapped.bb_type = record
            .bb_type
            .map(|bb_type| BackBlastType::from(bb_type.as_str()))
            .unwrap_or_default();
        results.push(mapped);
    }
    Ok(results)
}

pub async fn save_old_q_line_up(db_pool: &PgPool) -> Result<(), AppError> {
    let mut rdr = csv::ReaderBuilder::new().from_path("migration_files/q_line_up/q_sheet.csv")?;
    let mut results = Vec::<NewQLineUpDbEntry>::new();
    for record in rdr.deserialize() {
        let record: OldQSheetRow = record?;
        let date = NaiveDate::parse_from_str(record.date.as_str(), "%m/%d/%Y").unwrap();
        if let Some(q) = record.gem {
            add_q_entry(&mut results, AO::Gem, q, &date);
        }

        if let Some(q) = record.oldglory {
            add_q_entry(&mut results, AO::OldGlory, q, &date);
        }

        if let Some(q) = record.backyard {
            add_q_entry(&mut results, AO::Backyard, q, &date);
        }

        if let Some(q) = record.rebel {
            add_q_entry(&mut results, AO::Rebel, q, &date);
        }

        if let Some(q) = record.bleach {
            add_q_entry(&mut results, AO::Bleach, q, &date);
        }

        if let Some(q) = record.ruckership {
            add_q_entry(&mut results, AO::RuckershipWest, q, &date);
        }

        if let Some(q) = record.ironmountain {
            add_q_entry(&mut results, AO::IronMountain, q, &date);
        }

        if let Some(q) = record.rise {
            add_q_entry(&mut results, AO::Rise, q, &date);
        }

        if let Some(q) = record.lakeview_park {
            add_q_entry(&mut results, AO::WarHorse, q, &date);
        }
    }

    println!("saving {} entries for q line up", results.len());
    save_q_line_up::save_list(db_pool, &results).await?;
    Ok(())
}

fn add_q_entry(list: &mut Vec<NewQLineUpDbEntry>, ao: AO, q: String, date: &NaiveDate) {
    let new_entry = NewQLineUpDbEntry::new(vec![q], &ao, date, ao.channel_id());
    list.push(new_entry);
}

fn read_back_blasts(ao: &AO, path: &str) -> Result<Vec<BackBlastData>, AppError> {
    let path = std::path::Path::new(path);
    let mut results = Vec::<BackBlastData>::new();
    let mut rdr = csv::ReaderBuilder::new().from_path(path)?;

    for record in rdr.deserialize() {
        let record: OldBackBlast = record?;
        if record
            .pax
            .split(',')
            .map(|name| name.trim())
            .next()
            .is_none()
        {
            if record.q.split(',').map(|name| name.trim()).next().is_some() {
                println!("Qs but no pax");
            }
        } else if let Some(bb) = map_to_bb(ao, record) {
            results.push(bb);
        }
    }

    Ok(results)
}

pub fn clean_sheet_name(name: &str) -> &str {
    let name = name.trim();
    let name = if let Some((name, _)) = name.split_once(&['(', '|'][..]) {
        name.trim()
    } else {
        name
    };
    name
}

fn extract_names(list: &str) -> Vec<&str> {
    list.split(',').map(clean_sheet_name).collect()
}

fn back_blast_path(folder: &str) -> String {
    format!("{}/Backblasts.csv", folder)
}

fn map_to_bb(ao: &AO, old: OldBackBlast) -> Option<BackBlastData> {
    // date format: 10/8/2021
    let mut date_parsed = NaiveDate::parse_from_str(&old.date, "%m/%d/%Y").unwrap();
    if date_parsed < NaiveDate::from_ymd(2000, 1, 1) {
        date_parsed = NaiveDate::parse_from_str(&old.date, "%m/%d/%y").unwrap();
    }
    let pax = extract_names(&old.pax);
    let qs = extract_names(&old.q);
    if pax.is_empty() {
        if !qs.is_empty() {
            println!("Qs but no pax");
        }
        return None;
    }
    let mut q_set = HashSet::<String>::new();
    for q in qs {
        q_set.insert(q.to_string());
    }
    let mut pax_set = HashSet::<String>::new();
    for p in pax {
        pax_set.insert(p.to_string());
    }
    let data = BackBlastData::new(ao.clone(), q_set, pax_set, date_parsed);
    Some(data)
}
