use super::{
    models::{NewFile, NewNode, NodeType, Nodes},
    repositories::FileSystemRepository,
};
use crate::{responses::Error, state::AppState};

pub struct FileSystemService {
    file_system_repo: FileSystemRepository,
}

impl FileSystemService {
    pub fn new(state: AppState) -> Self {
        let file_system_repo = FileSystemRepository::new(state);
        Self { file_system_repo }
    }

    pub async fn all(&self, user_id: u64) -> Result<Nodes, sqlx::Error> {
        self.file_system_repo.all(user_id).await
    }

    pub async fn create(&self, node: NewNode) -> Result<(), Error> {
        match node.type_ {
            NodeType::Folder => {
                self.file_system_repo.create_node(node).await?;
            }
            NodeType::File => {
                let file = NewFile {
                    id: node.id.clone(),
                    user_id: node.user_id,
                    content: "".to_string(),
                };
                self.file_system_repo
                    .create_node_with_file(node, file)
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn update_node(
        &self,
        node_id: &str,
        new_name: &str,
        user_id: u64,
    ) -> Result<(), Error> {
        self.file_system_repo
            .update(node_id, new_name, user_id)
            .await?;
        Ok(())
    }

    pub async fn delete_node(&self, node_id: &str, user_id: u64) -> Result<(), Error> {
        self.file_system_repo.delete(node_id, user_id).await?;
        Ok(())
    }
}
