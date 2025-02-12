use axum::{body::Bytes, http::StatusCode};

pub async fn echo(body: Bytes) -> Result<String, StatusCode> {
    if body.is_empty() {
        return Ok("Received empty body".to_string());
    }
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
