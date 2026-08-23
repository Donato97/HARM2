use super::models::{CacheInsertRow, CacheResult};
use crate::{
    features::games::models::SteamSearchGames, helper::now, responses::Error, state::AppState,
};
use sea_query::{Expr, OnConflict, Query};
use time::Duration;

pub struct CacheRepository {
    pub state: AppState,
}

impl CacheRepository {
    pub fn new(state: AppState) -> Self {
        CacheRepository { state }
    }

    pub async fn find(&self, query: &str) -> Result<Vec<CacheResult>, sqlx::Error> {
        let query = Query::select()
            .column("results")
            .from("steam_search_cache")
            .and_where(Expr::col("query").eq(query))
            .and_where(Expr::col("created_at").gt(now() - Duration::hours(24)))
            .to_owned();

        self.state.exe_select(query).await
    }

    pub async fn upsert(&self, data: &CacheInsertRow) -> Result<(), Error> {
        let results = serde_json::to_string(&data.results)?;
        let query = Query::insert()
            .into_table("steam_search_cache")
            .columns(["query", "results", "created_at"])
            .values_panic([(&data.query).into(), results.into(), now().into()])
            .on_conflict(
                OnConflict::column("query")
                    .update_columns(["results", "created_at"])
                    .to_owned(),
            )
            .to_owned();

        self.state.exe_insert(query).await?;

        Ok(())
    }
}

pub struct GameRepository {
    pub state: AppState,
}

impl GameRepository {
    pub fn new(state: AppState) -> Self {
        GameRepository { state }
    }

    pub async fn insert_many(&self, data: &SteamSearchGames) -> Result<u64, sqlx::Error> {
        if data.is_empty() {
            return Ok(0);
        }

        let mut query = Query::insert()
            .into_table("games")
            .columns(["id", "name"])
            .to_owned();

        for game in data {
            query.values_panic([game.id.into(), game.name.as_str().into()]);
        }

        query.on_conflict(OnConflict::column("id").do_nothing().to_owned());

        self.state.exe_insert(query).await
    }
}
