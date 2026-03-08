use axum::Router;
use std::sync::Arc;
use tokio::sync::broadcast;

mod routes;

mod state;
use state::AppState;

use crate::{model::LogEntry, state::SharedState};

mod db;
mod dto;
mod errors;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set in .env");

    let pg_pool = sqlx::PgPool::connect(&db_url).await?;
    let channel = broadcast::Sender::<LogEntry>::new(1000);

    let state: SharedState = Arc::new(AppState {
        db: pg_pool,
        channel: channel,
    });

    let app = Router::new()
        .merge(routes::health::router())
        .merge(routes::log::router())
        .merge(routes::service::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}
