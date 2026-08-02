use super::views;
use crate::responses::markup::AppResponse;
use axum::response::IntoResponse;

pub async fn sign_up() -> AppResponse {
    Ok(views::sign_up().into_response())
}

pub async fn sign_in() -> AppResponse {
    Ok(views::sign_in().into_response())
}
