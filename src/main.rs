use axum::Router;
use std::sync::{Arc, Mutex};

mod routes;

mod state;
use state::{AppState, SharedState};

mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state: SharedState = Arc::new(AppState {
        ingested_logs: Mutex::new(vec![]),
    });

    let app = Router::new()
        .merge(routes::health::router())
        .merge(routes::log::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}
