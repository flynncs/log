use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::Type)]
#[sqlx(type_name = "log_level", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct LogEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    #[sqlx(rename = "level")]
    pub level: LogLevel,
    pub message: String,
    pub service: String,
    pub attributes: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct NewLogEntry {
    pub level: LogLevel,
    pub service: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub attributes: serde_json::Value,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}
