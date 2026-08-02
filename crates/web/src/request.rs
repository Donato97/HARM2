use anyhow::anyhow;
use app_core::responses::Error;
use axum::extract::Request;
use tower_sessions::Session;

pub trait RequestSession {
    fn session(&self) -> Result<Session, Error>;
}

impl RequestSession for Request {
    fn session(&self) -> Result<Session, Error> {
        self.extensions()
            .get::<Session>()
            .cloned()
            .ok_or(Error::Internal(anyhow!("Session layer not found")))
    }
}
