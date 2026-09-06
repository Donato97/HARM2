use super::services::GameService;
use crate::{
    features::games::{
        repositories::GameRepository,
        views::{self, game_added_card, search_results_list},
    },
    request::RequestContext,
    responses::markup::AppResponse,
};
use axum::{extract::Request, http::StatusCode, response::IntoResponse};
use hypertext::Renderable;
use validator::Validate;

#[derive(Debug, Validate, serde::Deserialize)]
struct SearchQuery {
    #[validate(length(min = 3))]
    query: String,
}

pub async fn index(mut req: Request) -> AppResponse {
    let state = req.state()?;
    let repo = GameRepository::new(state);
    let user = req.user()?;

    let view_props = views::Props {
        path: req.uri().path().to_string(),
        recents: repo.recently_added(user.id).await?,
        games: repo.all(user.id).await?,
        search_results: Vec::new(),
    };
    Ok(views::index(view_props).into_response())
}

pub async fn search(mut req: Request) -> AppResponse {
    let SearchQuery { query } = req.query().await?;
    let state = req.state()?;
    let user = req.user()?;

    let service = GameService::new(state, user);
    let results = service.search(&query).await?;

    Ok(search_results_list(&results).render().into_response())
}

pub async fn add(mut req: Request) -> AppResponse {
    let state = req.state()?;
    let user = req.user()?;
    let appid: u64 = req.path().await?;

    let service = GameService::new(state.clone(), user);
    let game = service.add(appid).await?;

    if let Some(game) = game {
        return Ok(game_added_card(&game).render().into_response());
    }
    Ok(StatusCode::NO_CONTENT.into_response())
}
