use axum::{extract::Request, response::IntoResponse, Json};

use crate::{
    features::file_system::{
        models::{NewNode, NodeType},
        services::FileSystemService,
    },
    request::RequestContext,
    responses::api::ApiResponse,
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateBody {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub type_: NodeType,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateBody {
    pub name: String,
}

pub async fn index(mut req: Request) -> ApiResponse {
    let state = req.state()?;
    let user = req.user()?;

    let service = FileSystemService::new(state);
    let nodes = service.all(user.id).await?;

    Ok(Json(nodes).into_response())
}

pub async fn create(mut req: Request) -> ApiResponse {
    let user = req.user()?;
    let state = req.state()?;
    let body: CreateBody = req.body().await?;

    let new_node = NewNode {
        id: body.id,
        user_id: user.id,
        parent_id: body.parent_id,
        name: body.name,
        type_: body.type_,
    };
    let service = FileSystemService::new(state);
    service.create(new_node).await?;

    Ok(Json(()).into_response())
}

pub async fn update(mut req: Request) -> ApiResponse {
    let node_id: String = req.path().await?;
    let state = req.state()?;
    let user = req.user()?;
    let body: UpdateBody = req.body().await?;

    let service = FileSystemService::new(state);
    service.update_node(&node_id, &body.name, user.id).await?;

    Ok(Json(()).into_response())
}

pub async fn delete(mut req: Request) -> ApiResponse {
    let node_id: String = req.path().await?;
    let state = req.state()?;
    let user = req.user()?;

    let service = FileSystemService::new(state);
    service.delete_node(&node_id, user.id).await?;

    Ok(Json(()).into_response())
}
