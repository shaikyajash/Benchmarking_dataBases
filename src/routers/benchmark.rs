use axum::{Router, routing::get};

use crate::{handlers::benchmark_handler::benchmark_handler, store::shared_state::AppState};

pub fn benchmark_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(benchmark_handler))
        .with_state(state)
}
