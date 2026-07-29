use sea_query::{Expr, Query};

use super::models::User;
use crate::AppState;

pub struct UserRepository {
    state: AppState,
}

impl UserRepository {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn create(&self, email: &str, password_hash: &str) -> Result<u64, sqlx::Error> {
        let query = Query::insert()
            .into_table("users")
            .columns(["email", "password"])
            .values_panic([email.into(), password_hash.into()])
            .to_owned();

        self.state.exe_insert(query).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let query = Query::select()
            .from("users")
            .columns(["id", "steam_id", "email", "password"])
            .and_where(Expr::column("email").eq(email))
            .to_owned();

        let result = self.state.exe_select::<_, User>(query).await?;

        Ok(result.into_iter().next())
    }
}
