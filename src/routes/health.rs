use axum::{Router, extract::State, routing::get};

use crate::state::SharedState;

pub async fn health(State(state): State<SharedState>) -> String {
    state.greeting.clone()
}

pub fn router() -> Router<SharedState> {
    Router::new().route("/health", get(health))
}
