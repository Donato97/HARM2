use std::collections::HashMap;

use app_core::{features::auth::models::SessionUser, responses::markup::AppError, AppState};
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
    RequestPartsExt,
};
use tower_sessions::Session;

pub struct SteamLoginRequest {
    pub state: AppState,
    pub session: Session,
    pub params: HashMap<String, String>,
    pub user: SessionUser,
}

impl FromRequestParts<AppState> for SteamLoginRequest {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(params) = parts
            .extract()
            .await
            .map_err(|_| AppError::BadRequest("Invalid query parameters"))?;

        let session = parts
            .extract::<Session>()
            .await
            .map_err(|(_, msg)| AppError::InternalServerError(anyhow::anyhow!(msg)))?;

        let user = parts.extensions.remove::<SessionUser>().ok_or_else(|| {
            AppError::InternalServerError(anyhow::anyhow!("SessionUser extension missing"))
        })?;

        Ok(Self {
            state: state.clone(),
            session,
            params,
            user,
        })
    }
}

pub struct SteamLoginPageRequest {
    pub uri: String,
    pub user: SessionUser,
}

impl FromRequestParts<AppState> for SteamLoginPageRequest {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let uri = parts.uri.to_string();
        let user = parts.extensions.remove::<SessionUser>().ok_or_else(|| {
            AppError::InternalServerError(anyhow::anyhow!("SessionUser extension missing"))
        })?;

        Ok(Self { uri, user })
    }
}
