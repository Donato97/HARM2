use app_core::{
    features::auth::{models::SessionUser, repositories::UserRepository},
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

use crate::features::auth::requests::AuthBody;

pub struct AuthService {
    user_repo: UserRepository,
}

impl AuthService {
    pub fn new(state: AppState) -> Self {
        let user_repo = UserRepository::new(state);
        Self { user_repo }
    }

    pub async fn sign_up(&self, body: AuthBody) -> Result<SessionUser, AppError> {
        let hash = self.hash_password(&body.password)?;

        let id = self
            .user_repo
            .create(&body.email, &hash)
            .await
            .map_err(|e| {
                if let Some(error) = e.as_database_error() {
                    if error.is_unique_violation() {
                        return bad_request(None);
                    }
                }
                server_error(e)
            })?;

        let session_user = SessionUser {
            id,
            steam_id: None,
            email: body.email,
        };

        Ok(session_user)
    }

    pub async fn sign_in(&self, body: AuthBody) -> Result<SessionUser, AppError> {
        let user = self
            .user_repo
            .find_by_email(&body.email)
            .await
            .map_err(server_error)?;

        match user {
            Some(user) => {
                self.verify_password(&body.password, &user.password)?;
                let session_user = SessionUser {
                    id: user.id,
                    steam_id: user.steam_id.clone(),
                    email: body.email,
                };
                Ok(session_user)
            }
            None => Err(bad_request(Some("Invalid credential"))),
        }
    }

    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(server_error)?
            .to_string();

        Ok(hash)
    }

    fn verify_password(&self, password: &str, stored_hash: &str) -> Result<(), AppError> {
        let password_hash = PasswordHash::new(stored_hash).map_err(server_error)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &password_hash)
            .map_err(|_| bad_request(Some("Invalid credentials")))?;

        Ok(())
    }
}
