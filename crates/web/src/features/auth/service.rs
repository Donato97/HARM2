use app_core::{
    features::auth::{models::SessionUser, repositories::UserRepository},
    responses::markup::AppError,
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
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_unique_violation() {
                        return AppError::BadRequest("Email already exists");
                    }
                }
                AppError::InternalServerError(e.into())
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
            .await?
            .ok_or(AppError::BadRequest("Invalid credentials"))?;

        self.verify_password(&body.password, &user.password)?;

        Ok(SessionUser {
            id: user.id,
            steam_id: user.steam_id.clone(),
            email: body.email,
        })
    }

    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(hash)
    }

    fn verify_password(&self, password: &str, stored_hash: &str) -> Result<(), AppError> {
        let password_hash = PasswordHash::new(stored_hash)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &password_hash)
            .map_err(|_| AppError::BadRequest("Invalid credentials"))?;

        Ok(())
    }
}
