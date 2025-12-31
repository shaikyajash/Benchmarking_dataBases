use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::{
    config::NUM_RECORDS,
    store::{shared_state::AppState, users::Users},
};

#[derive(Serialize)]
pub struct BenchmarkResponse {
    database: String,
    insert_time_ms: u128,
    read_time_ms: u128,
    clear_time_ms: u128,
    entries: usize,
}

pub async fn benchmark_handler(
    State(state): State<AppState>,
) -> Result<Json<BenchmarkResponse>, StatusCode> {
    // Load users data
    let users = match Users::load("users.json") {
        Ok(users) => users,
        Err(e) => {
            eprintln!("Failed to load users: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Run PostgreSQL benchmark - clean API!
    let result = match users.postgres_benchmark(&state.pg_pool).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Benchmark failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(BenchmarkResponse {
        database: "PostgreSQL".to_string(),
        insert_time_ms: result.insert_time_ms,
        read_time_ms: result.read_time_ms,
        clear_time_ms: result.clear_time_ms,
        entries: NUM_RECORDS,
    }))
}
