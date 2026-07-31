use sea_query::{OnConflict, Query};

use super::models::NewNode;
use crate::{features::file_system::models::NewFile, AppState};

pub struct FileSystemRepository {
    state: AppState,
}

impl FileSystemRepository {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn create_node(&self, node: NewNode) -> Result<u64, sqlx::Error> {
        let query = Query::insert()
            .into_table("nodes")
            .columns(["id", "user_id", "parent_id", "name", "type"])
            .values_panic([
                node.id.into(),
                node.user_id.into(),
                node.parent_id.into(),
                node.name.into(),
                node.type_.into(),
            ])
            .on_conflict(OnConflict::column("id").update_columns(["name"]).to_owned())
            .to_owned();

        self.state.exe_insert(query).await
    }

    pub async fn create_file(&self, file: NewFile) -> Result<u64, sqlx::Error> {
        let query = Query::insert()
            .into_table("notes")
            .columns(["id", "user_id", "content"])
            .values_panic([file.id.into(), file.user_id.into(), file.content.into()])
            .to_owned();

        self.state.exe_insert(query).await
    }
}
