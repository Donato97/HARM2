use std::error::Error;

use app_core::helper::markup_errors::{bad_request, server_error, AppResponse};
use axum::{
    body::to_bytes,
    extract::Request,
    response::{IntoResponse, Redirect},
};
use reqwest::Method;

pub async fn sign_up(request: Request) -> AppResponse {
    let (parts, body) = request.into_parts();

    let headers = parts.headers;
    let bytes = to_bytes(body, usize::MAX).await.map_err(server_error)?;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(server_error)?;

    let result = client
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

pub async fn sign_in(request: Request) -> AppResponse {
    let (parts, body) = request.into_parts();

    let headers = parts.headers;
    let bytes = to_bytes(body, usize::MAX).await.map_err(server_error)?;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(server_error)?;

    let result = client
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

pub async fn sign_out() -> AppResponse {
    let keychain =
        keyring_core::Entry::new("com.harm2.desktop", "session_token").map_err(server_error)?;

    let token = keychain.get_password().map_err(server_error)?;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(server_error)?;

    client
        .request(Method::POST, "http://localhost:3000/logout")
        .header("Cookie", format!("id={}", token))
        .send()
        .await
        .map_err(server_error)?;

    keychain.delete_credential().map_err(server_error)?;

    Ok(Redirect::to("/").into_response())
}
