use crate::model::{LogEntry, LogLevel, NewLogEntry};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, query_as};

pub async fn insert(db: &PgPool, entry: NewLogEntry) -> Result<LogEntry, sqlx::Error> {
    query_as!(
        LogEntry,
        "INSERT INTO log_entries (level, service, message, attributes, trace_id, span_id) 
        VALUES ($1, $2, $3, $4, $5, $6) 
        RETURNING id, timestamp, level as \"level: LogLevel\", service, message, attributes, trace_id, span_id",
        entry.level as LogLevel,
        entry.service,
        entry.message,
        entry.attributes,
        entry.trace_id,
        entry.span_id
    )
    .fetch_one(db)
    .await
}

pub async fn find_all(
    db: &PgPool,
    service: Option<String>,
    level: Option<LogLevel>,
    limit: Option<i32>,
    offset: Option<i32>,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
) -> Result<Vec<LogEntry>, sqlx::Error> {
    query_as!(
        LogEntry,
        "SELECT id, timestamp, level as \"level: LogLevel\", service, message, attributes, trace_id, span_id FROM log_entries 
        WHERE ($1::text IS NULL OR service = $1)
        AND ($2::text IS NULL OR level = $2::text)
        AND ($3::timestamptz IS NULL OR timestamp >= $3)
        AND ($4::timestamptz IS NULL OR timestamp <= $4)
        ORDER BY timestamp DESC
        LIMIT COALESCE($5, 50)
        OFFSET COALESCE($6, 0)
        ",
        service,
        level as Option<LogLevel>,
        from,
        to,
        limit,
        offset
    )
    .fetch_all(db)
    .await
}
