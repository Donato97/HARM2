use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Note {
    pub name: String,
    pub content: String,
}

pub type Notes = Vec<Note>;
