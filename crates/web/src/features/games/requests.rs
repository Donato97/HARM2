use std::collections::HashMap;

use app_core::{features::auth::models::SessionUser, responses::markup::AppError, AppState};
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
    Extension,
};

pub struct SteamLoginRequest {
    pub params: HashMap<String, String>,
    pub user: SessionUser,
}

impl FromRequestParts<AppState> for SteamLoginRequest {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(params) = Query::<HashMap<String, String>>::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::BadRequest("Invalid query parameters"))?;

        let Extension(user) = Extension::<SessionUser>::from_request_parts(parts, state).await?;

        Ok(Self { params, user })
    }
}
