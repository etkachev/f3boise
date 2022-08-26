//! Module containing implementations for the common `AppError` wrapper to better consume different error types.

use std::{error, fmt, io};

/// Wrapper for common errors that may be encountered in application
#[derive(Debug)]
pub enum AppError {
    /// Basic IO error
    Io(io::Error),
    Csv(csv::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> Self {
        AppError::Csv(err)
    }
}
