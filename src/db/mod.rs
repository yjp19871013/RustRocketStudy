pub mod user;

mod schema;

use std;
use std::fmt;
use crate::rocket_sync_db_pools::{database};

#[database("simple_web")]
pub struct DBConn(diesel::MysqlConnection);

#[derive(Debug, Clone)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.0
    }
}

