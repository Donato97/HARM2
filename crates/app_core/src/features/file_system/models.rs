use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Folder,
    File,
}

impl From<NodeType> for sea_query::Value {
    fn from(value: NodeType) -> Self {
        match value {
            NodeType::Folder => "folder".into(),
            NodeType::File => "file".into(),
        }
    }
}

pub struct NewNode {
    pub id: String,
    pub user_id: u64,
    pub parent_id: Option<String>,
    pub name: String,
    pub type_: NodeType,
}

pub struct NewFile {
    pub id: String,
    pub user_id: u64,
    pub content: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Node {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub type_: String,
    pub created_at: String,
    pub updated_at: String,
}

pub type Nodes = Vec<Node>;
