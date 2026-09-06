use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;

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

// ====================================

#[derive(Debug, Deserialize)]
pub struct SteamDetailsResponse {
    pub response: Items,
}

#[derive(Debug, Deserialize)]
pub struct Items {
    pub store_items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: u64,
    pub assets: Assets,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Assets {
    pub asset_url_format: String,
    pub main_capsule: Option<String>,
    pub small_capsule: Option<String>,
    pub header: Option<String>,
    pub page_background: Option<String>,
    pub hero_capsule: Option<String>,
    pub library_capsule: Option<String>,
    pub library_hero: Option<String>,
    pub community_icon: Option<String>,
}

// ====================================
#[derive(Debug, sqlx::FromRow)]
pub struct CacheResult {
    #[sqlx(json)]
    pub results: SteamSearchGames,
}

pub struct CacheInsertRow {
    pub query: String,
    pub results: SteamSearchGames,
}

#[derive(Debug, FromRow)]
pub struct Game {
    pub id: u64,
    pub name: String,
    #[sqlx(json(nullable))]
    pub assets: Option<Assets>,
    pub tiny_image: Option<String>,
    pub synced_at: Option<PrimitiveDateTime>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
