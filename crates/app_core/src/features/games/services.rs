use reqwest::Client;

use super::{
    models::{CacheInsertRow, SteamSearchGames, SteamSearchResponse},
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

    pub fn add(appid: &str) {
        // cerca il gioco nel DB
        // se non esiste
        // 		ritorna errore 400
        //
        // collegalo all'utente
        // se è già collegato
        // 		non fare niente
        //
        // in background aggiungi i dettagli dei giochi ancora non sincronizzati
    }

    pub fn sync_details() {
        // cerca i giochi non ancora completati con i dettagli e posseduti dall'utente
        // per ogni gioco trovato
        // 		fetcha i dettagli
        // 		se la fetch da errore 429
        // 			ferma tutto
        // 		se il body contiene success: false
        // 			aggiorna la data di sync nel db
        // 			passa al gioco successivo
        // 		se ci sono altri tipi di errore
        // 			passa al gioco successivo
        //
        // 		aggiorna il DB
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
}
