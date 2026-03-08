use crate::model::{LogEntry, LogLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LogIngest {
    pub message: String,
    pub level: LogLevel,
    pub service: String,
    pub attributes: serde_json::Value,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

#[derive(Serialize)]
pub struct LogIngestResponse {
    pub log: LogEntry,
}

#[derive(Serialize)]
pub struct LogResponse {
    pub logs: Vec<LogEntry>,
}

#[derive(Deserialize)]
pub struct LogQuery {
    pub level: Option<LogLevel>,
    pub service: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct LogStreamQuery {
    pub level: Option<LogLevel>,
    pub service: Option<String>,
}
