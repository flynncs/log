use crate::model::{LogEntry, LogLevel, NewLogEntry};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, QueryBuilder, query_as};

// Single insert - keeping as a reference for `query_as`
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

pub async fn insert_many(
    db: &PgPool,
    entries: Vec<NewLogEntry>,
) -> Result<Vec<LogEntry>, sqlx::Error> {
    if entries.is_empty() {
        return Ok(vec![]);
    }

    let mut query_builder = QueryBuilder::new(
        "INSERT INTO log_entries (level, service, message, attributes, trace_id, span_id) ",
    );
    query_builder.push_values(entries, |mut builder, entry| {
        builder
            .push_bind(entry.level as LogLevel)
            .push_bind(entry.service)
            .push_bind(entry.message)
            .push_bind(sqlx::types::Json(entry.attributes))
            .push_bind(entry.trace_id)
            .push_bind(entry.span_id);
    });

    query_builder
        .push("RETURNING id, timestamp, level, service, message, attributes, trace_id, span_id");

    query_builder
        .build_query_as::<LogEntry>()
        .fetch_all(db)
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
        AND ($2::text IS NULL OR level = $2::log_level)
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
