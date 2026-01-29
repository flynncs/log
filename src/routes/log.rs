use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

use crate::state::SharedState;

#[derive(Deserialize)]
pub struct LogIngest {
    message: String,
    level: String,
}

#[derive(Serialize)]
pub struct IngestResponse {
    status: String,
}

pub async fn ingest(
    State(state): State<SharedState>,
    Json(payload): Json<LogIngest>,
) -> (StatusCode, Json<IngestResponse>) {
    let mut count = state.ingest_count.lock().unwrap();
    *count += 1;

    (
        StatusCode::CREATED,
        Json(IngestResponse {
            status: format!("got: {} ({}) #{}", payload.message, payload.level, *count),
        }),
    )
}

pub fn router() -> Router<SharedState> {
    Router::new().route("/log", post(ingest))
}
