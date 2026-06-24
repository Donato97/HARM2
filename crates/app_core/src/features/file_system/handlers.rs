use axum::{extract::State, response::IntoResponse, Extension, Json};
use sea_query::{Expr, OnConflict, Query};

use crate::{
    features::{auth::models::SessionUser, file_system::models::Nodes},
    helper::{api_errors::server_error, ApiResponse},
    AppState,
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateBody {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub type_: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateBody {
    pub id: String,
    pub name: String,
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

pub async fn create_or_update(
    Extension(user): Extension<SessionUser>,
    State(state): State<AppState>,
    Json(body): Json<CreateBody>,
) -> ApiResponse {
    let body_clone = body.clone();
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
        .on_conflict(OnConflict::column("id").update_columns(["name"]).to_owned())
        .to_owned();

    state.exe_insert(query).await.map_err(server_error)?;

    if body_clone.type_ == "file" {
        let content_query = Query::insert()
            .into_table("notes")
            .columns(["id", "user_id", "content"])
            .values_panic([body_clone.id.into(), user.id.into(), "".into()])
            .to_owned();

        state
            .exe_insert(content_query)
            .await
            .map_err(server_error)?;
    }

    Ok(().into_response())
}

pub async fn update(
    Extension(user): Extension<SessionUser>,
    State(state): State<AppState>,
    Json(body): Json<UpdateBody>,
) -> ApiResponse {
    let query = Query::update()
        .table("nodes")
        .values([("name", body.name.into())])
        .and_where(Expr::col("id").eq(body.id.clone()))
        .and_where(Expr::col("user_id").eq(user.id))
        .to_owned();

    state.exe_update(query).await.map_err(server_error)?;

    Ok(().into_response())
}
