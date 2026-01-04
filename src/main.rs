mod config;
mod handlers;
mod routers;
mod store;
mod utils;

use axum::{Router, routing::get};

use crate::{
    handlers::health::health_check, routers::benchmark::benchmark_router,
    store::shared_state::AppState, utils::db_functions::Databases,
};

#[tokio::main]

async fn main() {
    //connect to Databases
    let db = Databases::new().await;

    //Tables Setup
    match db.postgres_tables_setup().await {
        Ok(_) => println!("Postgres Tables Setup Successfull"),
        Err(e) => println!("Postgres Tables Setup Failed: {}", e),
    };

    //Shared State
    let state = AppState::new(
        db.pg_pool,
        db.mongo_db_connection,
        db.surreal_db_connection,
        db.rocks_db_connection,
        db.level_db_connection,
    );

    //Router setup
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/benchmark", benchmark_router(state.clone()));

    println!("🚀 Server running on http://localhost:3000");

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 3000: {}", e);
            std::process::exit(1);
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server running on http://localhost:3000"),
        Err(e) => eprintln!("Failed to start server: {}", e),
    }
}
