use std::{fmt, fs::File, io::Read, time::Instant};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::utils::db_operations::PgOperations;

pub enum Error {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    SqlxError(sqlx::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::SerdeError(e) => write!(f, "Serialization error: {}", e),
            Error::SqlxError(e) => write!(f, "SQLx error: {}", e),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::SqlxError(err)
    }
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub active: bool,
}

#[derive(Debug, Serialize)]
pub struct BenchmarkResult {
    pub insert_time_ms: u128,
    pub read_time_ms: u128,
    pub clear_time_ms: u128,
}

// Wrapper struct for Vec<User> with benchmark methods
pub struct Users {
    data: Vec<User>,
}

impl Users {
    /// Load users from a JSON file
    pub fn load(filepath: &str) -> Result<Self, Error> {
        let mut file = File::open(filepath).map_err(Error::IoError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(Error::IoError)?;

        let data: Vec<User> = serde_json::from_str(&contents).map_err(Error::SerdeError)?;

        Ok(Users { data })
    }

    /// Benchmark PostgreSQL database operations
    pub async fn postgres_benchmark(&self, pool: &PgPool) -> Result<BenchmarkResult, Error> {
        // Clear existing data
        let start_clear = Instant::now();
        pool.clear_users().await?;
        let clear_time_ms = start_clear.elapsed().as_millis();

        // Insert all users
        let start_write = Instant::now();
        for user in &self.data {
            pool.insert_users(user).await?;
        }

        let insert_time_ms = start_write.elapsed().as_millis();

        // Read all users
        let start_read = Instant::now();
        pool.read_users().await?;
        let read_time_ms = start_read.elapsed().as_millis();

        Ok(BenchmarkResult {
            insert_time_ms,
            read_time_ms,
            clear_time_ms,
        })
    }
}
