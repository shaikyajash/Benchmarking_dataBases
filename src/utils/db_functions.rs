use std::sync::{Arc, Mutex};

use mongodb::Database;
use sqlx::PgPool;
use surrealdb::Surreal;

use crate::utils::connect_to_db::{
    connect_to_leveldb, connect_to_mongodb, connect_to_pgsql, connect_to_rocksdb,
    connect_to_surrealdb,
};

use rocksdb::DB as RocksDB;
use rusty_leveldb::DB as LevelDB;

pub struct Databases {
    pub pg_pool: PgPool,
    pub mongo_db_connection: Database,
    pub surreal_db_connection: Surreal<surrealdb::engine::remote::ws::Client>,
    pub rocks_db_connection: RocksDB,
    pub level_db_connection: Arc<Mutex<LevelDB>>,
}

impl Databases {
    pub async fn new() -> Self {
        let pg_pool = match connect_to_pgsql().await {
            Ok(pool) => {
                eprint!("Connected to PostgreSQL\n");
                pool
            }
            Err(e) => {
                eprintln!("Failed to connect to PostgreSQL: {}", e);
                std::process::exit(1);
            }
        };

        let mongo_db_connection = match connect_to_mongodb().await {
            Ok(db) => {
                eprint!("Connected to MongoDB\n");
                db
            }
            Err(e) => {
                eprintln!("Failed to connect to MongoDB: {}", e);
                std::process::exit(1);
            }
        };

        let surreal_db_connection = match connect_to_surrealdb().await {
            Ok(db) => {
                eprint!("Connected to SurrealDB\n");
                db
            }
            Err(e) => {
                eprintln!("Failed to connect to SurrealDB: {}", e);
                std::process::exit(1);
            }
        };
        let rocks_db_connection = match connect_to_rocksdb() {
            Ok(db) => {
                eprint!("Connected to RocksDB\n");
                db
            }
            Err(e) => {
                eprintln!("Failed to connect to RocksDB: {}", e);
                std::process::exit(1);
            }
        };

        let level_db_connection = match connect_to_leveldb() {
            Ok(db) => {
                eprint!("Connected to LevelDB\n");
                db
            }
            Err(e) => {
                eprintln!("Failed to connect to LevelDB: {}", e);
                std::process::exit(1);
            }
        };

        Self {
            pg_pool,
            mongo_db_connection,
            surreal_db_connection,
            rocks_db_connection,
            level_db_connection,
        }
    }

    pub async fn postgres_tables_setup(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            age INT NOT NULL,
            active BOOLEAN NOT NULL
        )
    "#,
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }
}
