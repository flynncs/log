use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LogIngest {
    message: String,
    level: String,
}

#[derive(Serialize)]
pub struct IngestResponse {
    status: String,
}

pub async fn ingest(Json(payload): Json<LogIngest>) -> (StatusCode, Json<IngestResponse>) {
    (
        StatusCode::CREATED,
        Json(IngestResponse {
            status: format!("got: {} ({})", payload.message, payload.level),
        }),
    )
}
