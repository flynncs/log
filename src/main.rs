use axum::{Router, routing::get, routing::post};

mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/log", post(routes::log::ingest));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}
