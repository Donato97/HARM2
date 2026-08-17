use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum NodeType {
    Folder,
    File,
}

impl NodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::Folder => "folder",
            NodeType::File => "file",
        }
    }
}

impl From<NodeType> for sea_query::Value {
    fn from(value: NodeType) -> Self {
        value.as_str().into()
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

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Node {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub type_: NodeType,
    pub created_at: String,
    pub updated_at: String,
}

pub type Nodes = Vec<Node>;

#[derive(Debug)]
pub struct Tree<'a> {
    pub files: Vec<&'a Node>,
    pub folders: Vec<Subtree<'a>>,
}

#[derive(Debug)]
pub struct Subtree<'a> {
    pub node: &'a Node,
    pub files: Vec<&'a Node>,
    pub folders: Vec<Subtree<'a>>,
}
