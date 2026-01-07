use core::fmt;

use crate::utils::db_operations::{
    level_db_operations::LevelError, rocks_db_operations::RocksError,
};

pub enum Error {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    SqlxError(sqlx::Error),
    MongoError(mongodb::error::Error),
    SurrealError(surrealdb::Error),
    RocksError(RocksError),
    LevelError(LevelError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::SerdeError(e) => write!(f, "Serialization error: {}", e),
            Error::SqlxError(e) => write!(f, "SQLx error: {}", e),
            Error::MongoError(e) => write!(f, "MongoDB error: {}", e),
            Error::SurrealError(e) => write!(f, "SurrealDB error: {}", e),
            Error::RocksError(e) => write!(f, "RocksDB error: {}", e),
            Error::LevelError(e) => write!(f, "LevelDB error: {}", e),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::SqlxError(err)
    }
}
impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        Error::MongoError(err)
    }
}

impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Self {
        Error::SurrealError(err)
    }
}
impl From<RocksError> for Error {
    fn from(err: RocksError) -> Self {
        Error::RocksError(err)
    }
}
impl From<LevelError> for Error {
    fn from(err: LevelError) -> Self {
        Error::LevelError(err)
    }
}
