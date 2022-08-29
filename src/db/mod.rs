//! Db implementation of persisting state

use crate::app_state::ao_data::{const_names::AO_LIST, AoData};
use crate::app_state::backblast_data::BackBlastData;
use crate::db::db_back_blast::DbBackBlast;
use crate::shared::common_errors::AppError;
use crate::shared::constants::{DB_BACKBLAST_DIR_PATH, DB_DIR_PATH};
use crate::users::f3_user::F3User;
use chrono::{Datelike, Months, NaiveDate};
use std::collections::HashMap;

pub mod db_back_blast;

/// Main Db Store
#[derive(Debug)]
pub struct DbStore {}

impl DbStore {
    pub fn new() -> Self {
        DbStore {}
    }

    /// initialize main files and folders that need to exist.
    pub fn init_db(&self) -> Result<(), AppError> {
        self.check_db_dir()?;
        self.sync_ao_list()?;
        Ok(())
    }

    /// make sure db folder exists
    fn check_db_dir(&self) -> Result<(), AppError> {
        // if dir not setup yet.
        if !std::path::Path::new(DB_DIR_PATH).exists() {
            std::fs::create_dir(DB_DIR_PATH)?;
        }

        if !std::path::Path::new(DB_BACKBLAST_DIR_PATH).exists() {
            std::fs::create_dir(DB_BACKBLAST_DIR_PATH)?;
        }

        Ok(())
    }

    fn sync_ao_list(&self) -> Result<(), AppError> {
        let mut wrt = csv::WriterBuilder::new().from_path(self.ao_db_path())?;
        for ao in AO_LIST {
            wrt.serialize(AoData::from(&ao))?;
        }
        wrt.flush()?;

        Ok(())
    }

    fn ao_db_path(&self) -> String {
        format!("{}/ao_list.csv", DB_DIR_PATH)
    }

    fn users_db_path(&self) -> String {
        format!("{}/users.csv", DB_DIR_PATH)
    }

    pub fn resolve_new_back_blast(&self, back_blast: &BackBlastData) -> Result<(), AppError> {
        let date = &back_blast.date;
        let file_path = self.get_bb_db_file_path(date);
        let mut results = HashMap::<String, DbBackBlast>::new();
        // if already exists, fetch existing data first.
        if std::path::Path::new(&file_path).exists() {
            let mut reader = csv::ReaderBuilder::new().from_path(&file_path)?;

            for bb in reader.deserialize() {
                let bb: DbBackBlast = bb?;
                results.insert(bb.get_unique_id(), bb);
            }
        }
        let db_bb = DbBackBlast::from(back_blast);
        // now try to insert new backblast data
        results.insert(db_bb.get_unique_id(), db_bb);
        let mut wrt = csv::WriterBuilder::new().from_path(&file_path)?;
        let mut sorted_results: Vec<DbBackBlast> = results.into_values().into_iter().collect();
        sorted_results.sort_by(|a, b| a.date.cmp(&b.date));
        for record in sorted_results {
            wrt.serialize(record)?;
        }
        wrt.flush()?;
        Ok(())
    }

    pub fn get_all_back_blast_data(&self) -> Result<Vec<BackBlastData>, AppError> {
        let mut start_date = NaiveDate::from_ymd(2021, 1, 1);
        let mut file_path = self.get_bb_db_file_path(&start_date);
        let mut db_results = HashMap::<String, DbBackBlast>::new();

        while std::path::Path::new(&file_path).exists() {
            let mut rdr = csv::ReaderBuilder::new().from_path(&file_path)?;

            for bb in rdr.deserialize() {
                let bb: DbBackBlast = bb?;
                if db_results.contains_key(bb.get_unique_id().as_str()) {
                    println!("Dup record: {:?}", bb);
                }
                db_results.insert(bb.get_unique_id(), bb);
            }
            println!("Finished file fetching: {}", file_path);
            start_date = start_date
                .checked_add_months(Months::new(1))
                .expect("Could not increment month");
            file_path = self.get_bb_db_file_path(&start_date);
        }

        println!("Finished fetching");
        let mut list = Vec::<BackBlastData>::new();

        for (_, bb) in db_results.into_iter() {
            let data_bb = BackBlastData::from(bb);
            list.push(data_bb);
        }
        Ok(list)
    }

    fn get_bb_db_file_path(&self, date: &NaiveDate) -> String {
        let month = date.format("%m").to_string();
        let year = date.year();
        format!("{}{}-{}.csv", DB_BACKBLAST_DIR_PATH, year, month)
    }

    /// save current users state to local db
    pub fn sync_users_local(&self, users: &HashMap<String, F3User>) -> Result<(), AppError> {
        let mut wrt = csv::WriterBuilder::new().from_path(self.users_db_path())?;
        for (_id, user) in users.iter() {
            wrt.serialize(user)?;
        }
        wrt.flush()?;
        println!("Synced Users!");
        Ok(())
    }

    /// get hashmap of current users in db
    pub fn get_stored_users(&self) -> Result<HashMap<String, F3User>, AppError> {
        let mut reader = csv::ReaderBuilder::new().from_path(self.users_db_path())?;
        let mut results = HashMap::<String, F3User>::new();
        for user in reader.deserialize() {
            let user: F3User = user?;
            if let Some(id) = &user.id {
                results.insert(id.to_string(), user);
            }
        }
        Ok(results)
    }
}

impl Default for DbStore {
    fn default() -> Self {
        Self::new()
    }
}
