use sea_query::{MysqlQueryBuilder, QueryBuilder, QueryStatementBuilder, SqliteQueryBuilder};
use sea_query_binder::SqlxValues;
use sqlx::{mysql::MySqlRow, sqlite::SqliteRow, FromRow};

#[derive(Clone, Debug)]
pub enum CustomPool {
    Sqlite(sqlx::SqlitePool),
    Mysql(sqlx::MySqlPool),
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: CustomPool,
    pub http_client: reqwest::Client,
}

impl AppState {
    fn build<S: QueryStatementBuilder>(&self, stmt: S) -> (String, SqlxValues) {
        let builder: &dyn QueryBuilder = match &self.pool {
            CustomPool::Sqlite(_) => &SqliteQueryBuilder,
            CustomPool::Mysql(_) => &MysqlQueryBuilder,
        };
        let (query, values) = stmt.build_any(builder);
        (query, SqlxValues(values))
    }

    pub async fn exe_select<S, T>(&self, stmt: S) -> Result<Vec<T>, sqlx::Error>
    where
        S: QueryStatementBuilder,
        T: for<'r> FromRow<'r, SqliteRow> + for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
    {
        let (query, values) = self.build(stmt);
        match &self.pool {
            CustomPool::Sqlite(pool) => {
                sqlx::query_as_with::<_, T, _>(&query, values)
                    .fetch_all(pool)
                    .await
            }
            CustomPool::Mysql(pool) => {
                sqlx::query_as_with::<_, T, _>(&query, values)
                    .fetch_all(pool)
                    .await
            }
        }
    }

    pub async fn exe_insert<S>(&self, stmt: S) -> Result<u64, sqlx::Error>
    where
        S: QueryStatementBuilder,
    {
        let (query, values) = self.build(stmt);
        let id = match &self.pool {
            CustomPool::Sqlite(pool) => sqlx::query_with(&query, values)
                .execute(pool)
                .await?
                .last_insert_rowid() as u64,
            CustomPool::Mysql(pool) => sqlx::query_with(&query, values)
                .execute(pool)
                .await?
                .last_insert_id(),
        };

        Ok(id)
    }

    pub async fn exe_update<S>(&self, stmt: S) -> Result<u64, sqlx::Error>
    where
        S: QueryStatementBuilder,
    {
        let (query, values) = self.build(stmt);
        let id = match &self.pool {
            CustomPool::Sqlite(pool) => sqlx::query_with(&query, values)
                .execute(pool)
                .await?
                .rows_affected() as u64,
            CustomPool::Mysql(pool) => sqlx::query_with(&query, values)
                .execute(pool)
                .await?
                .rows_affected() as u64,
        };

        Ok(id)
    }

    pub async fn exe_delete<S>(&self, stmt: S) -> Result<(), sqlx::Error>
    where
        S: QueryStatementBuilder,
    {
        let (query, values) = self.build(stmt);
        match &self.pool {
            CustomPool::Sqlite(pool) => {
                sqlx::query_with(&query, values).execute(pool).await?;
            }
            CustomPool::Mysql(pool) => {
                sqlx::query_with(&query, values).execute(pool).await?;
            }
        };

        Ok(())
    }
}
