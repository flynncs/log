use axum::Json;
use axum::{Router, http::StatusCode, routing::get, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LogIngest {
    message: String,
    level: String,
}

#[derive(Serialize)]
struct IngestResponse {
    status: String,
}

async fn ingest(Json(payload): Json<LogIngest>) -> (StatusCode, Json<IngestResponse>) {
    (
        StatusCode::CREATED,
        Json(IngestResponse {
            status: format!("got: {} ({})", payload.message, payload.level),
        }),
    )
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let app = Router::new()
        .route("/health", get(health))
        .route("/log", post(ingest));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}
