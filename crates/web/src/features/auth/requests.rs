use app_core::{
    helper::markup_errors::{bad_request, server_error},
    AppState,
};
use axum::{
    extract::{FromRequest, FromRequestParts, Request},
    http::StatusCode,
    Form,
};
use hypertext::Rendered;
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
    type Rejection = (StatusCode, Rendered<String>);

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let session = Session::from_request_parts(&mut parts, state)
            .await
            .map_err(|_| server_error(std::io::Error::other("session layer missing")))?;

        let req = Request::from_parts(parts, body);
        let Form(body) = Form::<AuthBody>::from_request(req, state)
            .await
            .map_err(|_| bad_request(Some("Invalid body")))?;

        Ok(Self { session, body })
    }
}
