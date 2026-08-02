use super::models::Notes;
use crate::{request::RequestContext, responses::api::ApiResponse};
use axum::{extract::Request, response::IntoResponse, Json};
use sea_query::{Expr, Query};

#[derive(Debug, serde::Deserialize)]
pub struct UpdateBody {
    pub content: String,
}

pub async fn find(mut req: Request) -> ApiResponse {
    let id: String = req.path().await?;
    let user = req.user()?;
    let state = req.state()?;

    let query = Query::select()
        .columns(["content"])
        .from("notes")
        .and_where(Expr::col("id").eq(id))
        .and_where(Expr::col("user_id").eq(user.id))
        .to_owned();

    let result: Notes = state.exe_select(query).await?;

    Ok(Json(result.first()).into_response())
}

pub async fn update(mut req: Request) -> ApiResponse {
    let id: String = req.path().await?;
    let user = req.user()?;
    let state = req.state()?;
    let body: UpdateBody = req.body().await?;

    let query = Query::update()
        .table("notes")
        .values([("content", body.content.into())])
        .and_where(Expr::col("id").eq(id))
        .and_where(Expr::col("user_id").eq(user.id))
        .to_owned();

    state.exe_insert(query).await?;

    Ok(().into_response())
}
