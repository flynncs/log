use std::sync::Arc;

use tokio::sync::broadcast;

use crate::model::LogEntry;

pub struct AppState {
    pub db: sqlx::PgPool,
    pub channel: broadcast::Sender<LogEntry>,
}

pub type SharedState = Arc<AppState>;
