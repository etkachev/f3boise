use chrono::NaiveDate;
use f3_api_rs::app_state::ao_data::AO;
use f3_api_rs::app_state::backblast_data::BackBlastData;
use f3_api_rs::db::DbStore;
use f3_api_rs::shared::common_errors::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
#[serde(rename_all = "PascalCase")]
struct PaxCount {
    pub name: String,
    #[serde(rename = "Post Count")]
    pub post_count: u16,
}

fn map_to_bb(ao: &AO, old: OldBackBlast) -> Option<BackBlastData> {
    // date format: 10/8/2021
    let mut date_parsed = NaiveDate::parse_from_str(&old.date, "%m/%d/%Y").unwrap();
    if date_parsed < NaiveDate::from_ymd(2000, 1, 1) {
        date_parsed = NaiveDate::parse_from_str(&old.date, "%m/%d/%y").unwrap();
    }
    let pax: Vec<&str> = old.pax.split(',').map(|name| name.trim()).collect();
    let qs: Vec<&str> = old.q.split(',').map(|name| name.trim()).collect();
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

const BACK_YARD_BB_PATH: &str = "db_files/old/Backyard";
const BLEACH_BB_PATH: &str = "db_files/old/Bleach";
const GEM_BB_PATH: &str = "db_files/old/Gem";
const IR_BB_PATH: &str = "db_files/old/Iron Mountain";
const OLD_GLORY_BB_PATH: &str = "db_files/old/Old Glory";
const REBEL_BB_PATH: &str = "db_files/old/Rebel";
const RUCKERSHIP_BB_PATH: &str = "db_files/old/Ruckership";

fn back_blast_path(folder: &str) -> String {
    format!("{}/Backblasts.csv", folder)
}

fn pax_counts_path(folder: &str) -> String {
    format!("{}/PAX counts.csv", folder)
}

fn main() {
    let db = DbStore::new();
    db.init_db().expect("Could not init db");
    let aos = [
        (AO::Backyard, BACK_YARD_BB_PATH),
        (AO::Bleach, BLEACH_BB_PATH),
        (AO::Gem, GEM_BB_PATH),
        (AO::IronMountain, IR_BB_PATH),
        (AO::OldGlory, OLD_GLORY_BB_PATH),
        (AO::Rebel, REBEL_BB_PATH),
        (AO::Ruckership, RUCKERSHIP_BB_PATH),
    ];
    for (ao, file_path) in aos.iter() {
        let ao_name = ao.to_string();
        match read_back_blasts(ao, &back_blast_path(file_path)) {
            Ok(bb) => {
                for entry in bb {
                    if let Err(err) = db.resolve_new_back_blast(&entry) {
                        println!("Error saving bb: {:?}", err);
                    }
                }
                println!("Saved: {}", ao_name);
            }
            Err(err) => {
                println!("Err: {:?}", err);
            }
        }
    }

    match db.get_all_back_blast_data() {
        Ok(mut results) => {
            results.sort_by(|a, b| a.date.cmp(&b.date));

            for (ao, ao_path) in aos.iter() {
                if let Err(err) = verify_ao_stats(ao, &results, ao_path) {
                    println!("Error verifying: {:?}", err);
                }
            }
        }
        Err(err) => println!("Error getting all data: {:?}", err),
    }
}

fn verify_ao_stats(ao: &AO, data: &[BackBlastData], ao_file_path: &str) -> Result<(), AppError> {
    let ao = ao.clone();
    let pax_counts_file = pax_counts_path(ao_file_path);
    let mut rdr = csv::ReaderBuilder::new().from_path(pax_counts_file)?;
    for item in rdr.deserialize() {
        let item: PaxCount = item?;
        let pax_data = data
            .iter()
            .filter(|data| data.ao == ao && data.get_pax().contains(&item.name))
            .count();
        if pax_data != item.post_count as usize {
            println!("Mismatch for {} in {}", item.name, ao.to_string());
            println!("Calculated: {} | Recorded: {}", pax_data, item.post_count);
        }
    }
    Ok(())
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
