use anyhow::anyhow;
use axum::extract::Request;
use tower_sessions::Session;

pub trait RequestSession {
    fn session(&self) -> Result<Session, anyhow::Error>;
}

impl RequestSession for Request {
    fn session(&self) -> Result<Session, anyhow::Error> {
        self.extensions()
            .get::<Session>()
            .cloned()
            .ok_or(anyhow!("Session not found. Is `SessionManagerLayer` enabled?").into())
    }
}
