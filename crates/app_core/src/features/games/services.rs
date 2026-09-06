use reqwest::Client;

use super::{
    models::{CacheInsertRow, Game, SteamDetailsResponse, SteamSearchGames, SteamSearchResponse},
    repositories::{CacheRepository, GameRepository},
};
use crate::{features::auth::models::SessionUser, responses::Error, state::AppState};

pub struct GameService {
    pub state: AppState,
    pub user: SessionUser,
    pub steam_client: SteamClient,
    pub game_repo: GameRepository,
    pub cache_repo: CacheRepository,
}

impl GameService {
    pub fn new(state: AppState, user: SessionUser) -> Self {
        let steam_client = SteamClient::new(state.http_client.clone());
        let game_repo = GameRepository::new(state.clone());
        let cache_repo = CacheRepository::new(state.clone());

        GameService {
            state,
            user,
            steam_client,
            game_repo,
            cache_repo,
        }
    }

    pub async fn search(&self, query: &str) -> Result<SteamSearchGames, Error> {
        // se la ricerca è presente nella cache entro 24h
        // 		usa la cache
        // altrimenti
        // 		chiama le api di steam per la ricerca
        // 		aggiorna la cache
        // 		inseriscili nel DB
        //
        // 	incrociali con quelli posseduti
        // 	ritorna i risultati

        let query = &query.trim().to_lowercase().replace(" ", "");
        let cache = self.cache_repo.find(query).await?.into_iter().next();

        let games = match cache {
            Some(cache) => cache.results,
            None => {
                let result = self.steam_client.search(query).await?;
                self.game_repo.insert_many(&result.items).await?;

                let row = CacheInsertRow {
                    query: query.into(),
                    results: result.items,
                };
                self.cache_repo.upsert(&row).await?;

                row.results
            }
        };

        Ok(games)
    }

    pub async fn add(&self, appid: u64) -> Result<Option<Game>, Error> {
        // cerca il gioco nel DB
        // se non esiste
        // 		ritorna errore 400
        //
        // collegalo all'utente
        // se è già collegato
        // 		non fare niente
        //
        // in background aggiungi i dettagli dei giochi ancora non sincronizzati
        let game = self.game_repo.find(appid).await?.ok_or(Error::BadRequest(
            format!("The game {appid} doesn't exists!").into(),
        ))?;

        let rows_affected = self.game_repo.attach_to_user(game.id, self.user.id).await?;

        let state = self.state.clone();
        let user = self.user.clone();
        tokio::spawn(async move {
            let _ = GameService::new(state, user)
                .sync_details()
                .await
                .map_err(|e| tracing::error!(?e));
        });

        if rows_affected > 0 {
            return Ok(Some(game));
        }
        Ok(None)
    }

    pub async fn sync_details(&self) -> Result<(), Error> {
        // cerca i giochi non ancora completati con i dettagli e posseduti dall'utente
        // fetcha i dettagli
        // se la fetch da errore 429
        // 		ferma tutto
        // per ogni gioco trovato
        //		aggiorna il DB
        let games = self.game_repo.not_synced(self.user.id).await?;
        if games.is_empty() {
            return Ok(());
        }

        let details = self.steam_client.details(&games).await?;
        self.game_repo
            .update_details(details.response.store_items)
            .await?;

        Ok(())
    }
}

pub struct SteamClient {
    client: Client,
}

impl SteamClient {
    pub fn new(client: Client) -> Self {
        SteamClient { client }
    }

    pub async fn search(&self, query: &str) -> Result<SteamSearchResponse, reqwest::Error> {
        let url = format!("https://store.steampowered.com/api/storesearch/?term={query}&cc=IT");

        self.client
            .get(url)
            .send()
            .await?
            .json::<SteamSearchResponse>()
            .await
    }

    pub async fn details(&self, games: &[Game]) -> Result<SteamDetailsResponse, reqwest::Error> {
        let appids = games
            .iter()
            .map(|game| serde_json::json!({"appid": game.id}))
            .collect::<serde_json::Value>();

        let input_json = serde_json::json!({
            "ids": appids,
             "context":{"country_code":"IT"},
              "data_request":{"include_assets":true}
        });

        let url = format!(
            "https://api.steampowered.com/IStoreBrowseService/GetItems/v1/?input_json={input_json}"
        );

        self.client.get(url).send().await?.json().await
    }
}
