use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Sse, sse::Event},
    routing::{get, post},
};
use chrono::Utc;
use futures::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

use crate::{
    db::logs::{find_all, find_by_id, insert_many},
    dto::{
        logs::{LogIngest, LogIngestResponse, LogQuery, LogResponse, LogStreamQuery},
        otel::OtelLogsRequest,
    },
    errors::AppError,
    model::{LogEntry, LogLevel, NewLogEntry},
    otel::otel_to_log_entries,
    state::SharedState,
};

fn validate_log_ingest(log_ingest: &LogIngest) -> Result<(), AppError> {
    if log_ingest.message.trim().is_empty() {
        return Err(AppError::ValidationError(
            "message cannot be empty".to_string(),
        ));
    }

    if log_ingest.service.trim().is_empty() {
        return Err(AppError::ValidationError(
            "service cannot be empty".to_string(),
        ));
    }

    Ok(())
}

pub async fn ingest_logs(
    State(state): State<SharedState>,
    Json(payload): Json<Vec<LogIngest>>,
) -> Result<(StatusCode, Json<LogIngestResponse>), AppError> {
    let log_entries = payload
        .into_iter()
        .map(|entry| {
            validate_log_ingest(&entry)?;
            Ok(NewLogEntry {
                level: entry.level,
                message: entry.message,
                service: entry.service,
                attributes: entry.attributes,
                trace_id: entry.trace_id,
                span_id: entry.span_id,
                timestamp: Utc::now(),
            })
        })
        .collect::<Result<Vec<_>, AppError>>()?;

    let created_logs = insert_many(&state.db, log_entries).await?;

    if state.channel.receiver_count() > 0 {
        created_logs.iter().for_each(|entry| {
            let _ = state.channel.send(entry.clone());
        });
    }

    Ok((
        StatusCode::CREATED,
        Json(LogIngestResponse { logs: created_logs }),
    ))
}

pub async fn ingest_otel_logs(
    State(state): State<SharedState>,
    Json(payload): Json<OtelLogsRequest>,
) -> Result<StatusCode, AppError> {
    let log_entires = otel_to_log_entries(payload);

    let created_logs = insert_many(&state.db, log_entires).await?;

    if state.channel.receiver_count() > 0 {
        created_logs.iter().for_each(|entry| {
            let _ = state.channel.send(entry.clone());
        });
    }

    Ok(StatusCode::ACCEPTED)
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

pub async fn get_log(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<LogEntry>), AppError> {
    let log = find_by_id(&state.db, id).await?.ok_or(AppError::NotFound)?;
    Ok((StatusCode::OK, Json(log)))
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
        .route("/logs", post(ingest_logs).get(get_logs))
        .route("/logs/stream", get(stream_logs))
        .route("/v1/logs", post(ingest_otel_logs))
        .route("/logs/{id}", get(get_log))
}
