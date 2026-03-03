use axum::{Json, Router, extract::State, http::StatusCode, routing::get};

use crate::{db::services::get_all, dto::services::ServicesResponse, state::SharedState};

pub async fn get_services(
    State(state): State<SharedState>,
) -> (StatusCode, Json<ServicesResponse>) {
    let all_services = get_all(&state.db).await.unwrap();

    (
        StatusCode::OK,
        Json(ServicesResponse {
            services: all_services,
        }),
    )
}

pub fn router() -> Router<SharedState> {
    Router::new().route("/services", get(get_services))
}
