use axum::{http::StatusCode, response::IntoResponse};

pub struct AppError(anyhow::Error);

// This allows ? to automatically convert anyhow::Error to AppError
impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{}", self.0.to_string());
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}
