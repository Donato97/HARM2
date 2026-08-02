use axum::response::IntoResponse;
use hypertext::prelude::*;

use crate::{layouts, responses::markup::AppResponse};

pub async fn index() -> AppResponse {
    let markup = rsx! {
        <div class="h-full" id="root"></div>
    };

    let layout_props = layouts::default::Props {
        slot: markup,
        path: "".to_string(),
    };

    Ok(layouts::default(layout_props).into_response())
}
