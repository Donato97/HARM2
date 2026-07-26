use std::collections::HashMap;

use super::views;
use app_core::{
    features::auth::models::SessionUser,
    helper::{markup_errors::server_error, AppResponse},
    AppState,
};
use axum::{
    extract::{OriginalUri, Query, State},
    response::{IntoResponse, Redirect},
    Extension,
};
use sea_query::Expr;

pub async fn index(
    Extension(user): Extension<SessionUser>,
    OriginalUri(uri): OriginalUri,
) -> AppResponse {
    let props = views::Props {
        user,
        path: uri.path().to_string(),
    };

    Ok(views::index(props).into_response())
}

pub async fn steam_login(
    State(state): State<AppState>,
    Extension(user): Extension<SessionUser>,
    Query(mut params): Query<HashMap<String, String>>,
) -> AppResponse {
    let redirect_error = Ok(Redirect::to("/login?error=steam").into_response());

    if params.get("openid.mode").map(String::as_str) != Some("id_res") {
        return redirect_error;
    }

    params.insert("openid.mode".into(), "check_authentication".into());

    let response = state
        .http_client
        .post("https://steamcommunity.com/openid/login")
        .form(&params)
        .send()
        .await;

    let body = match response {
        Ok(r) => r.text().await.unwrap_or_default(),
        Err(_) => return redirect_error,
    };

    if !body.contains("is_valid:true") {
        return redirect_error;
    }

    let claimed_id = match params.get("openid.claimed_id") {
        Some(id) => id,
        None => return redirect_error,
    };

    let steam_id = match claimed_id.split('/').last() {
        Some(id) => id,
        None => return redirect_error,
    };

    let query = sea_query::Query::update()
        .table("users")
        .values([("steam_id", steam_id.into())])
        .and_where(Expr::col("id").eq(user.id))
        .to_owned();

    state.exe_update(query).await.map_err(server_error)?;

    Ok(Redirect::to("/games").into_response())
}
