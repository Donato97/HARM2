use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Note {
    pub content: String,
}

pub type Notes = Vec<Note>;
