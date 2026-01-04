use std::{fmt, fs::File, io::Read, sync::Arc, time::Instant};

use mongodb::Database;
use rocksdb::DB as RocksDB;
use rusty_leveldb::DB as LevelDB;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Mutex;
use surrealdb::{Surreal, engine::remote::ws};

use crate::utils::{
    level_db_operations::{LevelError, LevelOperations},
    mongo_db_operations::MongoOperations,
    psql_db_operations::PgOperations,
    rocks_db_operations::{RocksError, RocksOperations},
    surreal_db_operations::SurrealOperations,
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
    pub insert_time_s: f64,
    pub read_time_s: f64,
    pub clear_time_s: f64,
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
        let clear_time_s = start_clear.elapsed().as_secs_f64();

        // Insert all users
        let start_write = Instant::now();
        for user in &self.data {
            pool.insert_users(user).await?;
        }

        let insert_time_s = start_write.elapsed().as_secs_f64();

        // Read all users
        let start_read = Instant::now();
        pool.read_users().await?;
        let read_time_s = start_read.elapsed().as_secs_f64();

        Ok(BenchmarkResult {
            insert_time_s,
            read_time_s,
            clear_time_s,
        })
    }

    /// Benchmark MongoDB database operations
    pub async fn mongo_benchmark(&self, db: &Database) -> Result<BenchmarkResult, Error> {
        let start_clear = Instant::now();
        db.clear_users().await?;
        let clear_time_s = start_clear.elapsed().as_secs_f64();
        let start_write = Instant::now();
        for user in &self.data {
            db.insert_user(user).await?;
        }
        let insert_time_s = start_write.elapsed().as_secs_f64();
        let start_read = Instant::now();
        db.read_users().await?;
        let read_time_s = start_read.elapsed().as_secs_f64();
        Ok(BenchmarkResult {
            insert_time_s,
            read_time_s,
            clear_time_s,
        })
    }

    /// Benchmark SurrealDB database operations
    pub async fn surreal_benchmark(
        &self,
        db: &Surreal<ws::Client>,
    ) -> Result<BenchmarkResult, Error> {
        let start_clear = Instant::now();
        db.clear_users().await?;
        let clear_time_s = start_clear.elapsed().as_secs_f64();
        let start_write = Instant::now();
        for user in &self.data {
            db.insert_user(user).await?;
        }
        let insert_time_s = start_write.elapsed().as_secs_f64();
        let start_read = Instant::now();
        db.read_users().await?;
        let read_time_s = start_read.elapsed().as_secs_f64();
        Ok(BenchmarkResult {
            insert_time_s,
            read_time_s,
            clear_time_s,
        })
    }

    pub fn rocks_benchmark(&self, db: &Arc<RocksDB>) -> Result<BenchmarkResult, Error> {
        let start_clear = Instant::now();
        db.clear_users()?;
        let clear_time_s = start_clear.elapsed().as_secs_f64();
        let start_write = Instant::now();
        for user in &self.data {
            db.insert_user(user)?;
        }
        let insert_time_s = start_write.elapsed().as_secs_f64();
        let start_read = Instant::now();
        db.read_users()?;
        let read_time_s = start_read.elapsed().as_secs_f64();
        Ok(BenchmarkResult {
            insert_time_s,
            read_time_s,
            clear_time_s,
        })
    }

    pub fn level_benchmark(&self, db: &Arc<Mutex<LevelDB>>) -> Result<BenchmarkResult, Error> {
        let start_clear = Instant::now();
        db.clear_users()?;
        let clear_time_s = start_clear.elapsed().as_secs_f64();
        let start_write = Instant::now();
        for user in &self.data {
            db.insert_user(user)?;
        }
        let insert_time_s = start_write.elapsed().as_secs_f64();
        let start_read = Instant::now();
        db.read_users()?;
        let read_time_s = start_read.elapsed().as_secs_f64();
        Ok(BenchmarkResult {
            insert_time_s,
            read_time_s,
            clear_time_s,
        })
    }
}
