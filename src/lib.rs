pub mod app;
pub mod error;

pub use app::*;

use axum::extract::State;
use error::{AppError, Result};
use sqlx::PgPool;

pub async fn hello_world(State(pool): State<PgPool>) -> Result<String> {
    tracing::debug!("Hello world");

    let hello_pg = sqlx::query_scalar("select 'Hello, World!'")
        .fetch_one(&pool)
        .await?;

    Ok(hello_pg)
}

pub async fn hello_error() -> Result<String> {
    Err(AppError::Unknown)
}
