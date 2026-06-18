use axum::{extract::OriginalUri, response::IntoResponse};

use crate::{features::movies::views, helper::markup_errors::AppResponse};

pub async fn index(OriginalUri(uri): OriginalUri) -> AppResponse {
    let view_props = views::Props {
        path: uri.path().to_string(),
    };
    Ok(views::index(view_props).into_response())
}
