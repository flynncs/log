use std::sync::{Arc, Mutex};

pub struct AppState {
    pub greeting: String,
    pub ingest_count: Mutex<u64>,
}

pub type SharedState = Arc<AppState>;
