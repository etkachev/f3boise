use f3_api_rs::app_state::ao_data::AO;
use f3_api_rs::app_state::backblast_data::BackBlastData;
use f3_api_rs::configuration::get_configuration;
use f3_api_rs::db::DbStore;
use f3_api_rs::migrate_old::{clean_sheet_name, save_old_back_blasts, AOLIST};
use f3_api_rs::shared::common_errors::AppError;
use f3_api_rs::web_api_run::get_connection_pool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PaxCount {
    pub name: String,
    #[serde(rename = "Post Count")]
    pub post_count: u16,
}

fn pax_counts_path(folder: &str) -> String {
    format!("{}/PAX counts.csv", folder)
}

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read config");
    let connection_pool = get_connection_pool(&config.database);
    let db = DbStore::new();
    db.init_db().expect("Could not init db");
    if let Err(err) = save_old_back_blasts(&connection_pool).await {
        println!("Error saving bb: {:?}", err);
    }

    // TODO update
    match db.get_all_back_blast_data() {
        Ok(mut results) => {
            results.sort_by(|a, b| a.date.cmp(&b.date));

            for (ao, ao_path) in AOLIST.iter() {
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
    let mut pax_counts = Vec::<PaxCount>::new();
    for item in rdr.deserialize() {
        let item: PaxCount = item?;
        let pax_count_name = clean_sheet_name(item.name.as_str());
        let pax_count_name = pax_count_name.to_lowercase();
        let pax_data = data
            .iter()
            .filter(|data| {
                data.ao == ao
                    && data
                        .get_pax()
                        .iter()
                        .map(|name| name.to_lowercase())
                        .any(|name| name == pax_count_name)
            })
            .count();
        if pax_data != item.post_count as usize {
            println!("Mismatch for {} in {}", item.name, ao.to_string());
            println!("Calculated: {} | Recorded: {}", pax_data, item.post_count);
        }
        pax_counts.push(item);
    }

    let pax_counts_names: Vec<String> = pax_counts
        .iter()
        .filter_map(|pc| {
            if pc.post_count > 0 {
                let name = clean_sheet_name(pc.name.as_str());
                Some(name.to_lowercase())
            } else {
                None
            }
        })
        .collect();
    let pax_db_names: Vec<String> =
        data.iter()
            .filter(|item| item.ao == ao)
            .fold(Vec::new(), |mut acc, item| {
                for name in item.get_pax() {
                    if !acc.contains(&name) {
                        acc.push(name.to_lowercase());
                    }
                }
                acc
            });

    // check pax counts data
    for name in pax_counts_names.iter() {
        if !pax_db_names.contains(name) {
            println!(
                "{}: Found in PAX counts but not backblasts: {}",
                ao.to_string(),
                name
            );
        }
    }

    // check db data
    for name in pax_db_names.iter() {
        if !pax_counts_names.contains(name) {
            println!(
                "{}: Found in bb names but not PAX counts: {}",
                ao.to_string(),
                name
            );
        }
    }
    Ok(())
}
