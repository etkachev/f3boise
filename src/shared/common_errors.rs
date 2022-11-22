//! Module containing implementations for the common `AppError` wrapper to better consume different error types.

use std::{error, fmt, io};

/// Wrapper for common errors that may be encountered in application
#[derive(Debug)]
pub enum AppError {
    /// Basic IO error
    Io(io::Error),
    Csv(csv::Error),
    Sqlx(sqlx::Error),
    Serde(serde_json::Error),
    ChronoParse(chrono::ParseError),
    Usvg(resvg::usvg::Error),
    Reqwest(reqwest::Error),
    General(String),
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

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Sqlx(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serde(err)
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        AppError::ChronoParse(err)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::General(err)
    }
}

impl From<resvg::usvg::Error> for AppError {
    fn from(err: resvg::usvg::Error) -> Self {
        AppError::Usvg(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Reqwest(err)
    }
}
