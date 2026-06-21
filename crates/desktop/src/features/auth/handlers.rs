use app_core::{
    features::auth::models::SessionUser,
    helper::markup_errors::{bad_request, server_error, AppResponse},
    AppState,
};
use axum::{
    body::to_bytes,
    extract::{Request, State},
    response::{IntoResponse, Redirect},
};
use reqwest::Method;

pub async fn sign_up(State(state): State<AppState>, request: Request) -> AppResponse {
    let (parts, body) = request.into_parts();

    let headers = parts.headers;
    let bytes = to_bytes(body, usize::MAX).await.map_err(server_error)?;

    let result = state
        .http_client
        .request(Method::POST, "http://localhost:3000/sign-up")
        .headers(headers)
        .body(bytes)
        .send()
        .await
        .map_err(|_| bad_request(Some("Invalid credentials")))?;

    let (_, cookie) = result
        .headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(";").next())
        .and_then(|v| v.split_once("="))
        .ok_or_else(|| bad_request(Some("Invalid credentials")))?;

    let _ = keyring_core::Entry::new("com.harm2.desktop", "session_token")
        .map_err(server_error)?
        .set_password(cookie)
        .map_err(server_error)?;

    Ok(Redirect::to("/").into_response())
}

pub async fn sign_in(State(state): State<AppState>, request: Request) -> AppResponse {
    let (parts, body) = request.into_parts();

    let headers = parts.headers;
    let bytes = to_bytes(body, usize::MAX).await.map_err(server_error)?;

    let result = state
        .http_client
        .request(Method::POST, "http://localhost:3000/sign-in")
        .headers(headers)
        .body(bytes)
        .send()
        .await
        .map_err(|_| bad_request(Some("Invalid credentials")))?;

    let (_, cookie) = result
        .headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(";").next())
        .and_then(|v| v.split_once("="))
        .ok_or_else(|| bad_request(Some("Invalid credentials")))?;

    let _ = keyring_core::Entry::new("com.harm2.desktop", "session_token")
        .map_err(server_error)?
        .set_password(cookie)
        .map_err(server_error)?;

    Ok(Redirect::to("/").into_response())
}

pub async fn sign_out(State(state): State<AppState>) -> AppResponse {
    let keychain =
        keyring_core::Entry::new("com.harm2.desktop", "session_token").map_err(server_error)?;

    let token = keychain.get_password().map_err(server_error)?;

    state
        .http_client
        .request(Method::POST, "http://localhost:3000/sign-out")
        .header("Cookie", format!("id={}", token))
        .send()
        .await
        .map_err(server_error)?;

    keychain.delete_credential().map_err(server_error)?;

    Ok(Redirect::to("/").into_response())
}

pub async fn middleware(mut req: Request, next: Next) -> AppResponse {
    let session_user = SessionUser {
        id: 1,
        email: "".to_string(),
    };

    req.extensions_mut().insert(session_user);
    Ok(next.run(req).await.into_response())
}
