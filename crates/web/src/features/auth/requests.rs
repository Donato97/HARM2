use app_core::{responses::markup::AppError, AppState};
use axum::{
    extract::{FromRequest, FromRequestParts, Request},
    Form,
};
use tower_sessions::Session;

#[derive(serde::Deserialize)]
pub struct AuthBody {
    pub email: String,
    pub password: String,
}

pub struct AuthRequest {
    pub session: Session,
    pub body: AuthBody,
}

impl FromRequest<AppState> for AuthRequest {
    type Rejection = AppError;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let session = Session::from_request_parts(&mut parts, state)
            .await
            .map_err(|(_, msg)| AppError::InternalServerError(anyhow::anyhow!("{msg}")))?;

        let req = Request::from_parts(parts, body);
        let Form(body) = Form::<AuthBody>::from_request(req, state).await?;

        Ok(Self { session, body })
    }
}
