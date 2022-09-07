use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::BackBlastData;
use crate::db::save_back_blast::save;
use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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
    (AO::Ruckership, RUCKERSHIP_BB_PATH),
];

pub async fn save_old_back_blasts(db_pool: &PgPool) -> Result<(), AppError> {
    for (ao, file_path) in AOLIST.iter() {
        let ao_name = ao.to_string();
        let bb = read_back_blasts(ao, &back_blast_path(file_path))?;
        save(db_pool, &bb).await?;
        println!("Saved: {}", ao_name);
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
