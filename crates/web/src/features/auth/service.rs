use anyhow::Context;
use app_core::{
    features::auth::{models::SessionUser, repositories::UserRepository},
    responses::Error,
    state::AppState,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::features::auth::handlers::AuthBody;

pub struct AuthService {
    user_repo: UserRepository,
}

impl AuthService {
    pub fn new(state: AppState) -> Self {
        let user_repo = UserRepository::new(state);
        Self { user_repo }
    }

    pub async fn sign_up(&self, body: AuthBody) -> Result<SessionUser, Error> {
        let hash = self.hash_password(body.password).await?;

        let id = self
            .user_repo
            .create(&body.email, &hash)
            .await
            .map_err(|e| {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_unique_violation() {
                        return Error::BadRequest("Email already exists".into());
                    }
                }
                Error::Internal(e.into())
            })?;

        let session_user = SessionUser {
            id,
            steam_id: None,
            email: body.email,
        };

        Ok(session_user)
    }

    pub async fn sign_in(&self, body: AuthBody) -> Result<SessionUser, Error> {
        let user = self
            .user_repo
            .find_by_email(&body.email)
            .await?
            .ok_or(Error::BadRequest("Invalid credentials".into()))?;

        self.verify_password(body.password, user.password).await?;

        Ok(SessionUser {
            id: user.id,
            steam_id: user.steam_id,
            email: body.email,
        })
    }

    async fn hash_password(&self, password: String) -> Result<String, anyhow::Error> {
        let hash = tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map(|h| h.to_string())
        })
        .await??;

        Ok(hash)
    }

    async fn verify_password(&self, password: String, stored_hash: String) -> Result<(), Error> {
        let result = tokio::task::spawn_blocking(move || {
            let password_hash = PasswordHash::new(&stored_hash)?;

            Argon2::default().verify_password(password.as_bytes(), &password_hash)
        })
        .await
        .context("Verify password task paniced")?;

        result.map_err(|_| Error::BadRequest("Invalid credential".into()))?;

        Ok(())
    }
}
