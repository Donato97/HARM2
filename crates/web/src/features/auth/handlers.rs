use app_core::{features::auth::models::SessionUser, responses::markup::AppResponse};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

use super::{requests::AuthRequest, service::AuthService};
use crate::AppState;

pub async fn sign_up(State(state): State<AppState>, req: AuthRequest) -> AppResponse {
    let service = AuthService::new(state);
    let new_user = service.sign_up(req.body).await?;

    req.session.cycle_id().await?;
    req.session.insert("user", new_user).await?;

    Ok(Redirect::to("/").into_response())
}

pub async fn sign_in(State(state): State<AppState>, req: AuthRequest) -> AppResponse {
    let service = AuthService::new(state);
    let session_user = service.sign_in(req.body).await?;

    req.session.cycle_id().await?;
    req.session.insert("user", session_user).await?;

    Ok(Redirect::to("/").into_response())
}

pub async fn sign_out(session: Session) -> AppResponse {
    session.flush().await?;

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
