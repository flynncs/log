use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::{Sse, sse::Event},
    routing::{get, post},
};
use futures::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    db::logs::{find_all, insert},
    dto::logs::{LogIngest, LogIngestResponse, LogQuery, LogResponse},
    errors::AppError,
    model::NewLogEntry,
    state::SharedState,
};

pub async fn ingest(
    State(state): State<SharedState>,
    Json(payload): Json<LogIngest>,
) -> Result<(StatusCode, Json<LogIngestResponse>), AppError> {
    if payload.message.trim().is_empty() {
        return Err(AppError::ValidationError(
            "message cannot be empty".to_string(),
        ));
    }

    if payload.service.trim().is_empty() {
        return Err(AppError::ValidationError(
            "service cannot be empty".to_string(),
        ));
    }

    let log_entry = NewLogEntry {
        level: payload.level,
        message: payload.message,
        service: payload.service,
        attributes: payload.attributes,
        trace_id: payload.trace_id,
        span_id: payload.span_id,
    };

    let created_log = insert(&state.db, log_entry).await?;

    let _ = state.channel.send(created_log.clone());

    Ok((
        StatusCode::CREATED,
        Json(LogIngestResponse { log: created_log }),
    ))
}

pub async fn get_logs(
    State(state): State<SharedState>,
    Query(params): Query<LogQuery>,
) -> Result<(StatusCode, Json<LogResponse>), AppError> {
    // validate input
    let log_response = find_all(
        &state.db,
        params.service,
        params.level,
        params.limit,
        params.offset,
        params.from,
        params.to,
    )
    .await?;

    Ok((StatusCode::OK, Json(LogResponse { logs: log_response })))
}

pub async fn stream_logs(
    State(state): State<SharedState>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let receiver = state.channel.subscribe();
    let stream = BroadcastStream::new(receiver);

    let stream = stream.filter_map(|item| async move {
        match item {
            Ok(log_entry) => Some(Ok(Event::default().json_data(log_entry).unwrap())),
            Err(_) => None,
        }
    });

    Sse::new(stream)
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/log", post(ingest))
        .route("/logs", get(get_logs))
        .route("/logs/stream", get(stream_logs))
}
