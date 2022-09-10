//! Db implementation of persisting state

pub mod db_back_blast;
pub mod init;
pub mod queries;
pub mod save_back_blast;

/// Main Db Store
#[derive(Debug)]
pub struct DbStore {}

impl DbStore {
    pub fn new() -> Self {
        DbStore {}
    }
}

impl Default for DbStore {
    fn default() -> Self {
        Self::new()
    }
}
