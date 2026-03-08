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
    dto::logs::{LogIngest, LogIngestResponse, LogQuery, LogResponse, LogStreamQuery},
    errors::AppError,
    model::{LogEntry, LogLevel, NewLogEntry},
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

fn matches_filters(entry: &LogEntry, level: &Option<LogLevel>, service: &Option<String>) -> bool {
    let matches_level = match level {
        Some(l) => l == &entry.level,
        None => true,
    };

    let matches_service = match service {
        Some(s) => s == &entry.service,
        None => true,
    };

    matches_level && matches_service
}

pub async fn stream_logs(
    State(state): State<SharedState>,
    Query(params): Query<LogStreamQuery>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let receiver = state.channel.subscribe();
    let stream = BroadcastStream::new(receiver);

    let level_clone = params.level.clone();
    let service_clone = params.service.clone();

    let stream = stream.filter_map(move |item| {
        let level_clone = level_clone.clone();
        let service_clone = service_clone.clone();
        async move {
            match item {
                Ok(log_entry) if matches_filters(&log_entry, &level_clone, &service_clone) => {
                    Some(Ok(Event::default().json_data(log_entry).unwrap()))
                }
                _ => None,
            }
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
