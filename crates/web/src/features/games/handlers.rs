use crate::features::games::requests::{SteamLoginPageRequest, SteamLoginRequest};

use super::{services::SteamService, views};
use app_core::responses::markup::AppResponse;
use axum::response::{IntoResponse, Redirect};

pub async fn index(req: SteamLoginPageRequest) -> AppResponse {
    let props = views::Props {
        user: req.user,
        path: req.uri,
    };

    Ok(views::index(props).into_response())
}

pub async fn steam_login(req: SteamLoginRequest) -> AppResponse {
    let mut user = req.user;
    let steam_service = SteamService::new(req.state);
    let steam_id = steam_service.login(user.id, req.params).await?;

    user.steam_id = Some(steam_id);
    req.session.insert("user", user).await?;

    Ok(Redirect::to("/games").into_response())
}
