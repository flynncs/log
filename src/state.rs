use std::sync::{Arc, Mutex};

use crate::model::LogEntry;

pub struct AppState {
    pub ingested_logs: Mutex<Vec<LogEntry>>,
}

pub type SharedState = Arc<AppState>;
