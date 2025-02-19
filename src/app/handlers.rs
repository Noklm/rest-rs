use axum::{routing::post, Json, Router};

use super::models::UserPayload;

async fn create_user(Json(payload): Json<UserPayload>) -> String {
    format!("Bonjour {}", payload.email)
}

pub fn routes() -> Router {
    Router::new().route("/users", post(create_user))
}
