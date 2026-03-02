use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
};

use crate::{
    db::logs::{find_all, insert},
    dto::logs::{LogIngest, LogIngestResponse, LogQuery, LogResponse},
    model::NewLogEntry,
    state::SharedState,
};

pub async fn ingest(
    State(state): State<SharedState>,
    Json(payload): Json<LogIngest>,
) -> (StatusCode, Json<LogIngestResponse>) {
    let log_entry = NewLogEntry {
        level: payload.level,
        message: payload.message,
        service: payload.service,
        attributes: payload.attributes,
        trace_id: payload.trace_id,
        span_id: payload.span_id,
    };

    let created_log = insert(&state.db, log_entry).await.unwrap();

    (
        StatusCode::CREATED,
        Json(LogIngestResponse { log: created_log }),
    )
}

pub async fn get_logs(
    State(state): State<SharedState>,
    Query(params): Query<LogQuery>,
) -> (StatusCode, Json<LogResponse>) {
    let log_response = find_all(&state.db, params.service, params.level)
        .await
        .unwrap();

    (StatusCode::OK, Json(LogResponse { logs: log_response }))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/log", post(ingest))
        .route("/logs", get(get_logs))
}
