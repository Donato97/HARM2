use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromRow, serde::Serialize)]
pub struct SessionUser {
    pub id: u64,
    pub email: String,
}
