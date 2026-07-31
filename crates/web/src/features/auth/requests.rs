use app_core::{responses::markup::AppError, AppState};
use axum::{
    extract::{FromRequest, Request},
    Form, RequestExt,
};
use tower_sessions::Session;

#[derive(serde::Deserialize)]
pub struct AuthBody {
    pub email: String,
    pub password: String,
}

pub struct AuthRequest {
    pub state: AppState,
    pub session: Session,
    pub body: AuthBody,
}

impl FromRequest<AppState> for AuthRequest {
    type Rejection = AppError;

    async fn from_request(mut req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let session = req
            .extract_parts::<Session>()
            .await
            .map_err(|(_, msg)| AppError::InternalServerError(anyhow::anyhow!(msg)))?;

        let Form(body) = req
            .extract()
            .await
            .map_err(|_| AppError::BadRequest("Invalid body"))?;

        Ok(Self {
            state: state.clone(),
            session,
            body,
        })
    }
}
