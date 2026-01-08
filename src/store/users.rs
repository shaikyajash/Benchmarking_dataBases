use std::{fs::File, io::Read, sync::Arc, time::Instant};

use leveldb::database::Database as LevelDB;
use mongodb::Database;
use rocksdb::DB as RocksDB;
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Mutex;
use surrealdb::{Surreal, engine::remote::ws};

use crate::{
    store::{error, user_struct::User},
    utils::db_operations::{
        level_db_operations::LevelOperations, mongo_db_operations::MongoOperations,
        psql_db_operations::PgOperations, rocks_db_operations::RocksOperations,
        surreal_db_operations::SurrealOperations,
    },
};

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
    pub fn load(filepath: &str) -> Result<Self, error::Error> {
        let mut file = File::open(filepath).map_err(error::Error::IoError)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(error::Error::IoError)?;

        let data: Vec<User> = serde_json::from_str(&contents).map_err(error::Error::SerdeError)?;

        Ok(Users { data })
    }

    /// Benchmark PostgreSQL database operations
    pub async fn postgres_benchmark(&self, pool: &PgPool) -> Result<BenchmarkResult, error::Error> {
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
    pub async fn mongo_benchmark(&self, db: &Database) -> Result<BenchmarkResult, error::Error> {
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
    ) -> Result<BenchmarkResult, error::Error> {
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

    pub fn rocks_benchmark(&self, db: &Arc<RocksDB>) -> Result<BenchmarkResult, error::Error> {
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

    pub fn level_benchmark(
        &self,
        db: &Arc<Mutex<LevelDB<i32>>>,
    ) -> Result<BenchmarkResult, error::Error> {
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
