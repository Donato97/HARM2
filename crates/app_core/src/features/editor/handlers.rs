use axum::{extract::Request, response::IntoResponse};

use crate::{request::RequestContext, responses::markup::AppResponse};

use super::{
    models::{NewNode, NodeType, Subtree},
    services::EditorService,
    views,
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
    pub type_: NodeType,
}

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

pub async fn create(mut req: Request) -> AppResponse {
    let user = req.user()?;
    let state = req.state()?;
    let body: CreateBody = req.form().await?;

    let new_node = NewNode {
        id: body.id.clone(),
        user_id: user.id,
        parent_id: body.parent_id,
        name: body.name,
        type_: body.type_.clone(),
    };
    let service = EditorService::new(state);
    service.create(new_node).await?;

    let node = service.find(&body.id, user.id).await?;

    let markup = match body.type_ {
        NodeType::Folder => views::folder_markup(&Subtree {
            node: &node,
            folders: vec![],
            files: vec![],
        }),
        NodeType::File => views::file_markup(&node),
    };

    Ok(markup.rendered().into_response())
}

pub async fn update(mut req: Request) -> AppResponse {
    let node_id: String = req.path().await?;
    let state = req.state()?;
    let user = req.user()?;
    let body: UpdateBody = req.form().await?;

    let service = EditorService::new(state);
    let updated_node = service.update_node(&node_id, &body.name, user.id).await?;

    let markup = match body.type_ {
        NodeType::Folder => views::folder_button(&updated_node.name),
        NodeType::File => views::file_button(&updated_node.id, &updated_node.name),
    };

    Ok(markup.rendered().into_response())
}

pub async fn delete(mut req: Request) -> AppResponse {
    let node_id: String = req.path().await?;
    let state = req.state()?;
    let user = req.user()?;

    let service = EditorService::new(state);
    service.delete_node(&node_id, user.id).await?;

    Ok(().into_response())
}
