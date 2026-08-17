use super::{
    models::{NewFile, NewNode, Node, NodeType, Nodes, Subtree, Tree},
    repositories::FileSystemRepository,
};
use crate::{responses::Error, state::AppState};

pub struct EditorService {
    file_system_repo: FileSystemRepository,
}

impl EditorService {
    pub fn new(state: AppState) -> Self {
        let file_system_repo = FileSystemRepository::new(state);
        Self { file_system_repo }
    }

    pub async fn all(&self, user_id: u64) -> Result<Nodes, sqlx::Error> {
        self.file_system_repo.all(user_id).await
    }

    pub async fn find(&self, id: &str, user_id: u64) -> Result<Node, Error> {
        let nodes = self.file_system_repo.find(id, user_id).await?;
        nodes.into_iter().next().ok_or(Error::NotFound)
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
    ) -> Result<Node, Error> {
        self.file_system_repo
            .update(node_id, new_name, user_id)
            .await?;

        self.find(node_id, user_id).await
    }

    pub async fn delete_node(&self, node_id: &str, user_id: u64) -> Result<(), Error> {
        self.file_system_repo.delete(node_id, user_id).await?;
        Ok(())
    }

    pub fn build_tree<'a>(&self, nodes: &'a Nodes, parent_id: Option<&str>) -> Tree<'a> {
        let (folders, files): (Vec<&Node>, Vec<&Node>) = nodes
            .iter()
            .filter(|n| n.parent_id.as_deref() == parent_id)
            .partition(|n| n.type_ == NodeType::Folder);

        Tree {
            files,
            folders: folders
                .into_iter()
                .map(|folder| {
                    let Tree { files, folders } = self.build_tree(nodes, Some(&folder.id));
                    Subtree {
                        node: folder,
                        files,
                        folders,
                    }
                })
                .collect(),
        }
    }
}
