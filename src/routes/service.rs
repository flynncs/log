use axum::{Json, Router, extract::State, http::StatusCode, routing::get};

use crate::{
    db::services::get_all, dto::services::ServicesResponse, errors::AppError, state::SharedState,
};

pub async fn get_services(
    State(state): State<SharedState>,
) -> Result<(StatusCode, Json<ServicesResponse>), AppError> {
    let all_services = get_all(&state.db).await?;

    Ok((
        StatusCode::OK,
        Json(ServicesResponse {
            services: all_services,
        }),
    ))
}

pub fn router() -> Router<SharedState> {
    Router::new().route("/services", get(get_services))
}
