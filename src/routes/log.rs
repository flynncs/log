use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

use crate::{
    model::{LogEntry, LogLevel},
    state::SharedState,
};

#[derive(Deserialize)]
pub struct LogIngest {
    message: String,
    level: LogLevel,
    service: String,
    attributes: serde_json::Value,
}

#[derive(Serialize)]
pub struct IngestResponse {
    status: String,
}

pub async fn ingest(
    State(state): State<SharedState>,
    Json(payload): Json<LogIngest>,
) -> (StatusCode, Json<IngestResponse>) {
    let log_entry: LogEntry = LogEntry {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: payload.level,
        message: payload.message,
        service: payload.service,
        attributes: payload.attributes,
    };

    let response_status: String = format!("got: {} ({})", &log_entry.message, "ok");

    let mut ingested_logs = state.ingested_logs.lock().unwrap();
    ingested_logs.push(log_entry);

    (
        StatusCode::CREATED,
        Json(IngestResponse {
            status: response_status,
        }),
    )
}

pub fn router() -> Router<SharedState> {
    Router::new().route("/log", post(ingest))
}
