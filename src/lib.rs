pub mod app;
pub mod error;

pub use app::*;

use error::{AppError, Result};

pub async fn hello_world() -> Result<String> {
    tracing::debug!("Hello world");
    Ok(format!("Hello, World!"))
}

pub async fn hello_error() -> Result<String> {
    Err(AppError::Unknown)
}
