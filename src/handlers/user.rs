use axum::extract::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> String {
    format!("Bonjour {}", payload.email)
}
