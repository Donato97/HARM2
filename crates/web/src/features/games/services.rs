use std::collections::HashMap;

use anyhow::Context;
use app_core::{
    features::auth::{models::SessionUser, repositories::UserRepository},
    responses::markup::AppError,
    AppState,
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

    pub async fn login(&self, user: SessionUser, params: Params) -> Result<(), AppError> {
        let steam_id = self.validate_open_id(params).await?;

        self.user_repo
            .update_steam_id(user.id, steam_id.as_str())
            .await?;

        Ok(())
    }

    async fn validate_open_id(&self, mut params: Params) -> Result<String, AppError> {
        if params.get("openid.mode").map(String::as_str) != Some("id_res") {
            return Err(AppError::BadRequest("Invalid openid.mode"));
        }

        params.insert("openid.mode".into(), "check_authentication".into());

        let body = self
            .http_client
            .post("https://steamcommunity.com/openid/login")
            .form(&params)
            .send()
            .await
            .context("Steam verification faild")?
            .text()
            .await
            .context("Failed to parse response")?;

        if !body.contains("is_valid:true") {
            return Err(AppError::BadRequest("Invalid openid"));
        }

        match params.get("openid.claimed_id") {
            Some(id) => Ok(id.clone()),
            None => Err(AppError::BadRequest("Missing openid.claimed_id")),
        }
    }
}
