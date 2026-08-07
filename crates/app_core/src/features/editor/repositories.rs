use sea_query::{Expr, InsertStatement, OnConflict, Query};

use super::models::NewNode;
use crate::{
    features::editor::models::{NewFile, Nodes},
    state::AppState,
};

pub struct FileSystemRepository {
    state: AppState,
}

impl FileSystemRepository {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn all(&self, user_id: u64) -> Result<Nodes, sqlx::Error> {
        let query = Query::select()
            .columns([
                "id",
                "parent_id",
                "name",
                "type",
                "created_at",
                "updated_at",
            ])
            .from("nodes")
            .and_where(Expr::col("user_id").eq(user_id))
            .to_owned();

        let nodes: Nodes = self.state.exe_select(query).await?;

        Ok(nodes)
    }

    pub async fn create_node(&self, node: NewNode) -> Result<u64, sqlx::Error> {
        let query = self.get_node_query(node);
        self.state.exe_insert(query).await
    }

    pub async fn create_file(&self, file: NewFile) -> Result<u64, sqlx::Error> {
        let query = self.get_file_query(file);
        self.state.exe_insert(query).await
    }

    pub async fn create_node_with_file(
        &self,
        node: NewNode,
        file: NewFile,
    ) -> Result<(), sqlx::Error> {
        self.state
            .exe_transaction(vec![
                self.state.build(self.get_node_query(node)),
                self.state.build(self.get_file_query(file)),
            ])
            .await
    }

    pub async fn update(
        &self,
        node_id: &str,
        new_name: &str,
        user_id: u64,
    ) -> Result<u64, sqlx::Error> {
        let query = Query::update()
            .table("nodes")
            .values([("name", new_name.into())])
            .and_where(Expr::col("id").eq(node_id))
            .and_where(Expr::col("user_id").eq(user_id))
            .to_owned();

        self.state.exe_update(query).await
    }

    pub async fn delete(&self, node_id: &str, user_id: u64) -> Result<(), sqlx::Error> {
        let query = Query::delete()
            .from_table("nodes")
            .and_where(Expr::col("id").eq(node_id))
            .and_where(Expr::col("user_id").eq(user_id))
            .to_owned();

        self.state.exe_delete(query).await
    }

    fn get_node_query(&self, node: NewNode) -> InsertStatement {
        Query::insert()
            .into_table("nodes")
            .columns(["id", "user_id", "parent_id", "name", "type"])
            .values_panic([
                node.id.into(),
                node.user_id.into(),
                node.parent_id.into(),
                node.name.into(),
                node.type_.into(),
            ])
            .on_conflict(OnConflict::column("id").do_nothing().to_owned())
            .to_owned()
    }

    fn get_file_query(&self, file: NewFile) -> InsertStatement {
        Query::insert()
            .into_table("notes")
            .columns(["id", "user_id", "content"])
            .values_panic([file.id.into(), file.user_id.into(), file.content.into()])
            .to_owned()
    }
}
