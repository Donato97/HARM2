#[derive(Debug, serde::Deserialize)]
pub struct SteamSearchResponse {
    pub items: Vec<SteamSearchGame>,
}

pub type SteamSearchGames = Vec<SteamSearchGame>;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SteamSearchGame {
    pub id: u32,
    pub name: String,
    pub tiny_image: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CacheResult {
    #[sqlx(json)]
    pub results: SteamSearchGames,
}

pub struct CacheInsertRow {
    pub query: String,
    pub results: SteamSearchGames,
}
