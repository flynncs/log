use axum::Json;
use axum::{Router, routing::get, routing::post};
use serde::Deserialize;

#[derive(Deserialize)]
struct LogIngest {
    message: String,
    level: String,
}

async fn ingest(Json(payload): Json<LogIngest>) -> String {
    format!("got: {} ({})", payload.message, payload.level)
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/health", get(health))
        .route("/log", post(ingest));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
