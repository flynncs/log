use sqlx::{PgPool, query_scalar};

pub async fn get_all(db: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    query_scalar!("SELECT DISTINCT service FROM log_entries",)
        .fetch_all(db)
        .await
}
