use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

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
