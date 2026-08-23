use super::services::GameService;
use crate::{
    features::games::views::{self, search_results_list},
    request::RequestContext,
    responses::markup::AppResponse,
};
use axum::{extract::Request, response::IntoResponse};
use hypertext::Renderable;
use validator::Validate;

#[derive(Debug, Validate, serde::Deserialize)]
struct SearchQuery {
    #[validate(length(min = 3))]
    query: String,
}

pub async fn index(req: Request) -> AppResponse {
    let view_props = views::Props {
        path: req.uri().path().to_string(),
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
