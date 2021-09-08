use std::fmt::Formatter;
use std::num::ParseIntError;

use Error::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DBAccess(rusqlite::Error),
    RemindableValueParse(String)
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DBAccess(db_error) => write!(f, "Error while accessing the database: {}", db_error),
            RemindableValueParse(value) => write!(f, "Could not parse value while creating Remindable. Error: {}", value)
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        DBAccess(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        RemindableValueParse(err.to_string())
    }
}

impl From<Error> for rusqlite::Error {
    fn from(_: Error) -> Self {
        rusqlite::Error::ExecuteReturnedResults
    }
}