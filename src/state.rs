use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub greeting: String,
}

pub type SharedState = Arc<AppState>;
