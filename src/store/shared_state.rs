use mongodb::Database;
use sqlx::PgPool;
use surrealdb::{Surreal, engine::remote::ws};

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub mongo_db: Database,
    pub surreal_db: Surreal<ws::Client>,
}

impl AppState {
    pub fn new(pg_pool: PgPool, mongo_db: Database, surreal_db: Surreal<ws::Client>) -> Self {
        Self {
            pg_pool,
            mongo_db,
            surreal_db,
        }
    }
}
