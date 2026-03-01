use axum::{Router, routing::get};

use crate::state::SharedState;

pub async fn health() -> String {
    "OK".to_string()
}

pub fn router() -> Router<SharedState> {
    Router::new().route("/health", get(health))
}
