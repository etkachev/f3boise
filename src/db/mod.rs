//! Db implementation of persisting state

use crate::app_state::ao_data::{const_names::AO_LIST, AoData};
use crate::app_state::backblast_data::BackBlastData;
use crate::db::db_back_blast::DbBackBlast;
use crate::shared::common_errors::AppError;
use crate::shared::constants::{DB_BACKBLAST_DIR_PATH, DB_DIR_PATH};
use crate::users::f3_user::F3User;
use chrono::{Datelike, NaiveDate};
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
        println!("Saved backblast data to csv");
        Ok(())
    }

    fn get_bb_db_file_path(&self, date: &NaiveDate) -> String {
        let month = date.month();
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
