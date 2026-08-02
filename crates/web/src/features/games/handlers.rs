use super::{services::SteamService, views};
use crate::request::RequestSession;
use app_core::{request::RequestContext, responses::markup::AppResponse};
use axum::{
    extract::Request,
    response::{IntoResponse, Redirect},
};

pub async fn index(mut req: Request) -> AppResponse {
    let user = req.user()?;
    let path = req.uri().path().to_string();
    let props = views::Props { user, path };

    Ok(views::index(props).into_response())
}

pub async fn steam_login(mut req: Request) -> AppResponse {
    let mut user = req.user()?;
    let state = req.state()?;
    let params = req.params().await?;
    let session = req.session()?;

    let steam_service = SteamService::new(state);
    let steam_id = steam_service.login(user.id, params).await?;

    user.steam_id = Some(steam_id);
    session.insert("user", user).await?;

    Ok(Redirect::to("/games").into_response())
}
