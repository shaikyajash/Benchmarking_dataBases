use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::{
    config::NUM_RECORDS,
    store::{shared_state::AppState, users::Users},
};

#[derive(Serialize)]
pub struct BenchmarkResponse {
    database: String,
    insert_time_s: f64,
    read_time_s: f64,
    clear_time_s: f64,
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
        insert_time_s: result.insert_time_s,
        read_time_s: result.read_time_s,
        clear_time_s: result.clear_time_s,
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
        insert_time_s: mongo_result.insert_time_s,
        read_time_s: mongo_result.read_time_s,
        clear_time_s: mongo_result.clear_time_s,
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
        insert_time_s: surreal_result.insert_time_s,
        read_time_s: surreal_result.read_time_s,
        clear_time_s: surreal_result.clear_time_s,
        entries: NUM_RECORDS,
    });

    // RockDB Benchmark
    let rocks_result = match users.rocks_benchmark(&state.rocks_db) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("RocksDB Benchmark failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    results.push(BenchmarkResponse {
        database: "RocksDB".to_string(),
        insert_time_s: rocks_result.insert_time_s,
        read_time_s: rocks_result.read_time_s,
        clear_time_s: rocks_result.clear_time_s,
        entries: NUM_RECORDS,
    });

    // LevelDB Benchmark
    let level_result = match users.level_benchmark(&state.level_db) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("LevelDB Benchmark failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    results.push(BenchmarkResponse {
        database: "LevelDB".to_string(),
        insert_time_s: level_result.insert_time_s,
        read_time_s: level_result.read_time_s,
        clear_time_s: level_result.clear_time_s,
        entries: NUM_RECORDS,
    });

    Ok(Json(results))
}
