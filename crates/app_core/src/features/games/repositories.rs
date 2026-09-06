use super::models::{CacheInsertRow, CacheResult, Game};
use crate::{
    features::games::models::{Item, SteamSearchGames},
    helper::now,
    responses::Error,
    state::AppState,
};
use sea_query::{Expr, OnConflict, Order, Query};
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

    pub async fn all(&self, user_id: u64) -> Result<Vec<Game>, sqlx::Error> {
        let query = Query::select()
            .columns([
                ("games", "id"),
                ("games", "name"),
                ("games", "assets"),
                ("games", "tiny_image"),
                ("games", "synced_at"),
                ("games", "created_at"),
                ("games", "updated_at"),
            ])
            .from("games")
            .left_join(
                "games_users",
                Expr::col(("games_users", "game_id")).equals(("games", "id")),
            )
            .and_where(Expr::col(("games_users", "user_id")).eq(user_id))
            .to_owned();

        self.state.exe_select(query).await
    }

    pub async fn recently_added(&self, user_id: u64) -> Result<Vec<Game>, sqlx::Error> {
        let query = Query::select()
            .columns([
                ("games", "id"),
                ("games", "name"),
                ("games", "assets"),
                ("games", "tiny_image"),
                ("games", "synced_at"),
                ("games", "created_at"),
                ("games", "updated_at"),
            ])
            .from("games")
            .left_join(
                "games_users",
                Expr::col(("games_users", "game_id")).equals(("games", "id")),
            )
            .and_where(Expr::col(("games_users", "user_id")).eq(user_id))
            .order_by(("games_users", "created_at"), Order::Desc)
            .limit(6)
            .to_owned();

        self.state.exe_select(query).await
    }

    pub async fn not_synced(&self, user_id: u64) -> Result<Vec<Game>, sqlx::Error> {
        let query = Query::select()
            .columns([
                ("games", "id"),
                ("games", "name"),
                ("games", "assets"),
                ("games", "tiny_image"),
                ("games", "synced_at"),
                ("games", "created_at"),
                ("games", "updated_at"),
            ])
            .from("games")
            .left_join(
                "games_users",
                Expr::col(("games_users", "game_id")).equals(("games", "id")),
            )
            .and_where(Expr::col(("games_users", "user_id")).eq(user_id))
            .and_where(Expr::col("synced_at").is_null())
            .to_owned();

        self.state.exe_select(query).await
    }

    pub async fn find(&self, id: u64) -> Result<Option<Game>, sqlx::Error> {
        let query = Query::select()
            .columns([
                "id",
                "name",
                "assets",
                "tiny_image",
                "synced_at",
                "created_at",
                "updated_at",
            ])
            .from("games")
            .and_where(Expr::col("id").eq(id))
            .to_owned();

        Ok(self.state.exe_select(query).await?.into_iter().next())
    }

    pub async fn attach_to_user(&self, game_id: u64, user_id: u64) -> Result<u64, sqlx::Error> {
        let query = Query::insert()
            .into_table("games_users")
            .columns(["game_id", "user_id"])
            .values_panic([game_id.into(), user_id.into()])
            .on_conflict(
                OnConflict::columns(["game_id", "user_id"])
                    .do_nothing()
                    .to_owned(),
            )
            .to_owned();

        self.state.exe_insert_affected(query).await
    }

    pub async fn insert_many(&self, data: &SteamSearchGames) -> Result<u64, sqlx::Error> {
        if data.is_empty() {
            return Ok(0);
        }

        let mut query = Query::insert()
            .into_table("games")
            .columns(["id", "name", "tiny_image"])
            .to_owned();

        for game in data {
            query.values_panic([
                game.id.into(),
                game.name.as_str().into(),
                game.tiny_image.as_str().into(),
            ]);
        }

        query.on_conflict(OnConflict::column("id").do_nothing().to_owned());

        self.state.exe_insert(query).await
    }

    pub async fn update_details(&self, details: Vec<Item>) -> Result<(), Error> {
        for data in details {
            let assets = serde_json::to_string(&data.assets)?;
            let query = Query::update()
                .table("games")
                .values([("assets", assets.into()), ("synced_at", now().into())])
                .and_where(Expr::col("id").eq(data.id))
                .to_owned();

            self.state.exe_update(query).await?;
        }

        Ok(())
    }
}
