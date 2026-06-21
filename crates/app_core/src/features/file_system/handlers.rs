use axum::{extract::State, response::IntoResponse, Extension, Json};
use sea_query::{Expr, Query};

use crate::{
    features::{auth::models::SessionUser, file_system::models::Nodes},
    helper::{api_errors::server_error, ApiResponse},
    AppState,
};

#[derive(Debug, serde::Deserialize)]
pub struct CreateBody {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub type_: String,
}

pub async fn index(
    Extension(user): Extension<SessionUser>,
    State(state): State<AppState>,
) -> ApiResponse {
    let query = Query::select()
        .columns([
            "id",
            "parent_id",
            "name",
            "type",
            "created_at",
            "updated_at",
        ])
        .from("nodes")
        .and_where(Expr::col("user_id").eq(user.id))
        .to_owned();

    let nodes: Nodes = state.exe_select(query).await.map_err(server_error)?;

    Ok(Json(nodes).into_response())
}

pub async fn create(
    Extension(user): Extension<SessionUser>,
    State(state): State<AppState>,
    Json(body): Json<CreateBody>,
) -> ApiResponse {
    dbg!(&body);
    let query = Query::insert()
        .into_table("nodes")
        .columns(["id", "user_id", "parent_id", "name", "type"])
        .values_panic([
            body.id.into(),
            user.id.into(),
            body.parent_id.into(),
            body.name.into(),
            body.type_.into(),
        ])
        .to_owned();

    state.exe_insert(query).await.map_err(server_error)?;

    Ok(().into_response())
}
