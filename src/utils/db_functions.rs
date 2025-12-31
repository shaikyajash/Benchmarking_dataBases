use sqlx::PgPool;

use crate::utils::connect_to_db::connect_to_pgsql;

pub struct Databases {
    pub pg_pool: PgPool,
}

impl Databases {
    pub async fn new() -> Self {
        let pool = match connect_to_pgsql().await {
            Ok(pool) => {
                eprint!("Connected to PostgreSQL\n");
                pool
            }
            Err(e) => {
                eprintln!("Failed to connect to PostgreSQL: {}", e);
                std::process::exit(1);
            }
        };

        Self { pg_pool: pool }
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
