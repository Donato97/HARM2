use app_core::{
    features::auth::repositories::UserRepository,
    helper::{
        markup_errors::{bad_request, server_error},
        AppError,
    },
    AppState,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    extract::{FromRequest, FromRequestParts, Request},
    http::StatusCode,
    Form,
};
use hypertext::Rendered;
use tower_sessions::Session;

use crate::features::auth::handlers::SignUpBody;

pub struct AuthService {
    user_repo: UserRepository,
}

impl AuthService {
    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(server_error)?
            .to_string();

        Ok(hash)
    }

    fn verify_password(&self, password: &str) -> Result<bool, AppError> {
        let password_hash = PasswordHash::new(password).map_err(server_error)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &password_hash)
            .map_err(|_| bad_request(Some("Invalid credentials")))?;

        Ok(true)
    }
}

pub struct SignUpRequest {
    session: Session,
    service: AuthService,
    body: SignUpBody,
}

impl FromRequest<AppState> for SignUpRequest {
    type Rejection = (StatusCode, Rendered<String>);

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let session = Session::from_request_parts(&mut parts, state)
            .await
            .map_err(|_| server_error(std::io::Error::other("session layer missing")))?;

        let req = Request::from_parts(parts, body);
        let Form(body) = Form::<SignUpBody>::from_request(req, state)
            .await
            .map_err(|_| bad_request(Some("Invalid body")))?;

        let service = AuthService {
            user_repo: UserRepository::new(state.clone()),
        };

        Ok(Self {
            session,
            service,
            body,
        })
    }
}
