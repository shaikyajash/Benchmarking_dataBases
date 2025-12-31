use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect_to_pgsql() -> Result<PgPool, sqlx::Error> {
    let pg_pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://admin:admin@localhost:5432/testdb")
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to PostgreSQL: {}", e);
            std::process::exit(1);
        }
    };

    Ok(pg_pool)
}
