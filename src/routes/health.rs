use axum::{Router, routing::get};

pub async fn health() -> &'static str {
    "ok"
}

pub fn router() -> Router {
    Router::new().route("/health", get(health))
}
