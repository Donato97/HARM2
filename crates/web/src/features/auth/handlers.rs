use app_core::{
    features::auth::models::{SessionUser, User},
    helper::markup_errors::{bad_request, server_error, AppResponse},
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect},
    Form,
};
use sea_query::{Expr, Query};
use tower_sessions::Session;

use crate::AppState;

#[derive(serde::Deserialize)]
pub struct SignUpBody {
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct SignInBody {
    email: String,
    password: String,
}

pub async fn sign_up(
    session: Session,
    State(state): State<AppState>,
    Form(body): Form<SignUpBody>,
) -> AppResponse {
    let password_hash = {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(body.password.as_bytes(), &salt)
            .map_err(server_error)?
            .to_string()
    };

    let query = Query::insert()
        .into_table("users")
        .columns(["email", "password"])
        .values_panic([body.email.clone().into(), password_hash.into()])
        .to_owned();

    let id = state.exe_insert(query).await.map_err(|e| {
        if let Some(error) = e.as_database_error() {
            if error.is_unique_violation() {
                return bad_request(None);
            }
        }
        server_error(e)
    })?;

    let new_user = SessionUser {
        id,
        email: body.email,
    };

    session
        .insert("user", new_user)
        .await
        .map_err(server_error)?;

    Ok(Redirect::to("/").into_response())
}

pub async fn sign_in(
    session: Session,
    State(state): State<AppState>,
    Form(body): Form<SignInBody>,
) -> AppResponse {
    let query = Query::select()
        .from("users")
        .columns(["id", "email", "password"])
        .and_where(Expr::column("email").eq(&body.email))
        .to_owned();

    let result = state
        .exe_select::<_, User>(query)
        .await
        .map_err(server_error)?;

    let user = result
        .first()
        .ok_or_else(|| bad_request(Some("Invalid credentials")))?;

    let password_hash = PasswordHash::new(&user.password).map_err(server_error)?;

    Argon2::default()
        .verify_password(body.password.as_bytes(), &password_hash)
        .map_err(|_| bad_request(Some("Invalid credentials")))?;

    let session_user = SessionUser {
        id: user.id,
        email: body.email,
    };

    session
        .insert("user", session_user)
        .await
        .map_err(server_error)?;

    Ok(Redirect::to("/").into_response())
}

pub async fn sign_out(session: Session) -> AppResponse {
    session.flush().await.map_err(server_error)?;

    Ok(Redirect::to("/sign-in").into_response())
}

pub async fn middleware(session: Session, req: Request, next: Next) -> AppResponse {
    let session_user = session
        .get::<SessionUser>("user")
        .await
        .map_err(server_error)?;

    let path = req.uri().path();

    if session_user.is_some() {
        if path != "/sign-in" && path != "/sign-up" {
            Ok(next.run(req).await.into_response())
        } else {
            Ok(Redirect::to("/").into_response())
        }
    } else {
        if path == "/sign-in" || path == "/sign-up" {
            Ok(next.run(req).await.into_response())
        } else {
            Ok(Redirect::to("/sign-in").into_response())
        }
    }
}
