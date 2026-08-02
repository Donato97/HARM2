use super::service::AuthService;
use crate::request::RequestSession;
use app_core::{
    features::auth::models::SessionUser, request::RequestContext, responses::markup::AppResponse,
};
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

#[derive(serde::Deserialize)]
pub struct AuthBody {
    pub email: String,
    pub password: String,
}

pub async fn sign_up(mut req: Request) -> AppResponse {
    let state = req.state()?;
    let session = req.session()?;
    let body: AuthBody = req.form().await?;

    let service = AuthService::new(state);
    let new_user = service.sign_up(body).await?;

    session.cycle_id().await?;
    session.insert("user", new_user).await?;

    Ok(Redirect::to("/").into_response())
}

pub async fn sign_in(mut req: Request) -> AppResponse {
    let state = req.state()?;
    let session = req.session()?;
    let body: AuthBody = req.form().await?;

    let service = AuthService::new(state);
    let session_user = service.sign_in(body).await?;

    session.cycle_id().await?;
    session.insert("user", session_user).await?;

    Ok(Redirect::to("/").into_response())
}

pub async fn sign_out(req: Request) -> AppResponse {
    req.session()?.flush().await?;

    Ok(Redirect::to("/sign-in").into_response())
}

pub async fn middleware(session: Session, mut req: Request, next: Next) -> AppResponse {
    let session_user = session.get::<SessionUser>("user").await?;

    let path = req.uri().path();

    match session_user {
        Some(session_user) => {
            if path != "/sign-in" && path != "/sign-up" {
                req.extensions_mut().insert(session_user);
                Ok(next.run(req).await.into_response())
            } else {
                Ok(Redirect::to("/").into_response())
            }
        }
        None => {
            if path == "/sign-in" || path == "/sign-up" {
                Ok(next.run(req).await.into_response())
            } else {
                Ok(Redirect::to("/sign-in").into_response())
            }
        }
    }
}
