use axum::{Router, routing::get};

use crate::state::SharedState;

pub async fn health() -> &'static str {
    "OK"
}

pub fn router() -> Router<SharedState> {
    Router::new().route("/health", get(health))
}
