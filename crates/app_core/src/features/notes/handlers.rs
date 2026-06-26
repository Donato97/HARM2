use super::models::Notes;
use crate::{
    features::auth::models::SessionUser,
    helper::{api_errors::server_error, ApiResponse},
    AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use sea_query::{Expr, Query};

#[derive(Debug, serde::Deserialize)]
pub struct UpdateBody {
    pub content: String,
}

pub async fn find(
    Path(id): Path<String>,
    Extension(user): Extension<SessionUser>,
    State(state): State<AppState>,
) -> ApiResponse {
    let query = Query::select()
        .columns(["content"])
        .from("notes")
        .and_where(Expr::col("id").eq(id))
        .and_where(Expr::col("user_id").eq(user.id))
        .to_owned();

    let result: Notes = state.exe_select(query).await.map_err(server_error)?;

    Ok(Json(result.first()).into_response())
}

pub async fn update(
    Path(id): Path<String>,
    Extension(user): Extension<SessionUser>,
    State(state): State<AppState>,
    Json(body): Json<UpdateBody>,
) -> ApiResponse {
    let query = Query::update()
        .table("notes")
        .values([("content", body.content.into())])
        .and_where(Expr::col("id").eq(id))
        .and_where(Expr::col("user_id").eq(user.id))
        .to_owned();

    state.exe_insert(query).await.map_err(server_error)?;

    Ok(().into_response())
}
