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
) -> Result<Json<Vec<BenchmarkResponse>>, StatusCode> {
    // Load users data
    let users = match Users::load("users.json") {
        Ok(users) => users,
        Err(e) => {
            eprintln!("Failed to load users: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut results = Vec::new();

    // Run PostgreSQL benchmark - clean API!
    let result = match users.postgres_benchmark(&state.pg_pool).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Benchmark failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    results.push(BenchmarkResponse {
        database: "PostgreSQL".to_string(),
        insert_time_ms: result.insert_time_ms,
        read_time_ms: result.read_time_ms,
        clear_time_ms: result.clear_time_ms,
        entries: NUM_RECORDS,
    });

    //MongoDB Benchmark
    let mongo_result = match users.mongo_benchmark(&state.mongo_db).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Benchmark failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    results.push(BenchmarkResponse {
        database: "MongoDB".to_string(),
        insert_time_ms: mongo_result.insert_time_ms,
        read_time_ms: mongo_result.read_time_ms,
        clear_time_ms: mongo_result.clear_time_ms,
        entries: NUM_RECORDS,
    });

    //SurrealDB Benchmark
    let surreal_result = match users.surreal_benchmark(&state.surreal_db).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Benchmark failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    results.push(BenchmarkResponse {
        database: "SurrealDB".to_string(),
        insert_time_ms: surreal_result.insert_time_ms,
        read_time_ms: surreal_result.read_time_ms,
        clear_time_ms: surreal_result.clear_time_ms,
        entries: NUM_RECORDS,
    });

    Ok(Json(results))
}
