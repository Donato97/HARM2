use axum::{extract::Request, response::IntoResponse};

use crate::{request::RequestContext, responses::markup::AppResponse};

use super::{services::EditorService, views};

pub async fn index(mut req: Request) -> AppResponse {
    let state = req.state()?;
    let user = req.user()?;
    let service = EditorService::new(state);
    let nodes = service.all(user.id).await?;

    let props = views::Props {
        tree: service.build_tree(&nodes, None),
    };

    Ok(views::index(props).into_response())
}
