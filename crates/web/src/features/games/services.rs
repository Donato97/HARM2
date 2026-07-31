use std::collections::HashMap;

use anyhow::Context;
use app_core::{
    features::auth::repositories::UserRepository, responses::markup::AppError, AppState,
};
use reqwest::Client;

type Params = HashMap<String, String>;

pub struct SteamService {
    http_client: Client,
    user_repo: UserRepository,
}

impl SteamService {
    pub fn new(state: AppState) -> Self {
        Self {
            http_client: state.http_client.clone(),
            user_repo: UserRepository::new(state),
        }
    }

    pub async fn login(&self, user_id: u64, params: Params) -> Result<String, AppError> {
        let steam_id = self.validate_open_id(params).await?;

        self.user_repo
            .update_steam_id(user_id, steam_id.as_str())
            .await?;

        Ok(steam_id)
    }

    async fn validate_open_id(&self, mut params: Params) -> Result<String, AppError> {
        if params.get("openid.mode").is_some_and(|s| s != "id_res") {
            return Err(AppError::BadRequest("Invalid openid.mode"));
        }

        params.insert("openid.mode".into(), "check_authentication".into());

        let body = self
            .http_client
            .post("https://steamcommunity.com/openid/login")
            .form(&params)
            .send()
            .await
            .context("Steam verification failed")?
            .text()
            .await
            .context("Failed to parse response")?;

        if !body.contains("is_valid:true") {
            return Err(AppError::BadRequest("Invalid openid"));
        }

        let steam_id = params
            .get("openid.claimed_id")
            .ok_or(AppError::BadRequest("Missing openid.claimed_id"))?
            .split('/')
            .next_back()
            .context("Steam id missing from openid.claimed_id")?;

        Ok(steam_id.to_string())
    }
}
