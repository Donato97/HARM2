use axum::{
    extract::{FromRequest, Request},
    Json, RequestExt,
};

use crate::{responses::api::ApiError, AppState};

use super::super::auth::models::SessionUser;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateOrUpdateBody {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub type_: String,
}

pub struct CreateOrUpdateRequest {
    pub state: AppState,
    pub user: SessionUser,
    pub body: CreateOrUpdateBody,
}

impl FromRequest<AppState> for CreateOrUpdateRequest {
    type Rejection = ApiError;

    async fn from_request(mut req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let user =
            req.extensions_mut()
                .remove::<SessionUser>()
                .ok_or(ApiError::InternalServerError(anyhow::anyhow!(
                    "User extension not found"
                )))?;

        let Json(body) = req.extract().await?;

        Ok(Self {
            state: state.clone(),
            user,
            body,
        })
    }
}
