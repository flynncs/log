use std::sync::Arc;

pub struct AppState {
    pub db: sqlx::PgPool,
}

pub type SharedState = Arc<AppState>;
