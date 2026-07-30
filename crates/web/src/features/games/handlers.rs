use crate::features::games::requests::SteamLoginRequest;

use super::{services::SteamService, views};
use app_core::{features::auth::models::SessionUser, responses::markup::AppResponse, AppState};
use axum::{
    extract::{OriginalUri, State},
    response::{IntoResponse, Redirect},
    Extension,
};

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

pub async fn steam_login(State(state): State<AppState>, req: SteamLoginRequest) -> AppResponse {
    let steam_service = SteamService::new(state);
    steam_service.login(req.user, req.params).await?;

    Ok(Redirect::to("/games").into_response())
}
