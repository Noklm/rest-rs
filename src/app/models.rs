use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserPayload {
    pub email: String,
    #[allow(dead_code)]
    pub password: String,
}
