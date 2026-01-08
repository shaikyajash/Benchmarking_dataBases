use mongodb::Database;
use sqlx::PgPool;
use std::sync::Arc;
use std::sync::Mutex;
use surrealdb::{Surreal, engine::remote::ws};

use leveldb::database::Database as LevelDB;
use rocksdb::DB as RocksDB;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub mongo_db: Database,
    pub surreal_db: Surreal<ws::Client>,
    pub rocks_db: Arc<RocksDB>,
    pub level_db: Arc<Mutex<LevelDB<i32>>>,
}

impl AppState {
    pub fn new(
        pg_pool: PgPool,
        mongo_db: Database,
        surreal_db: Surreal<ws::Client>,
        rocks_db: RocksDB,
        level_db: Arc<Mutex<LevelDB<i32>>>,
    ) -> Self {
        let rocks_db = Arc::new(rocks_db);
        Self {
            pg_pool,
            mongo_db,
            surreal_db,
            rocks_db,
            level_db,
        }
    }
}
