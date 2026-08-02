use super::auth::models::SessionUser;
use crate::state::AppState;
use anyhow::anyhow;
use axum::{
    extract::{Path, Query, Request},
    Form, Json, RequestExt,
};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, future::Future};

pub trait RequestContext {
    fn state(&mut self) -> Result<AppState, anyhow::Error>;
    fn user(&mut self) -> Result<SessionUser, anyhow::Error>;
    fn path<T: DeserializeOwned + Send + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<T, anyhow::Error>>;
    fn params(&mut self) -> impl Future<Output = Result<HashMap<String, String>, anyhow::Error>>;
    fn body<T: DeserializeOwned + 'static>(self) -> impl Future<Output = Result<T, anyhow::Error>>;
    fn form<T: DeserializeOwned + 'static>(self) -> impl Future<Output = Result<T, anyhow::Error>>;
}

impl RequestContext for Request {
    fn state(&mut self) -> Result<AppState, anyhow::Error> {
        self.extensions()
            .get::<AppState>()
            .cloned()
            .ok_or(anyhow!("AppState extension not found").into())
    }

    fn user(&mut self) -> Result<SessionUser, anyhow::Error> {
        self.extensions()
            .get::<SessionUser>()
            .cloned()
            .ok_or(anyhow!("SessionUser extension not found").into())
    }

    async fn path<T: DeserializeOwned + Send + 'static>(&mut self) -> Result<T, anyhow::Error> {
        let Path(value) = self
            .extract_parts::<Path<T>>()
            .await
            .map_err(|_| anyhow!("Invalid path parameters"))?;
        Ok(value)
    }

    async fn params(&mut self) -> Result<HashMap<String, String>, anyhow::Error> {
        let Query(params) = self
            .extract_parts::<Query<_>>()
            .await
            .map_err(|_| anyhow!("Invalid query parameters"))?;

        Ok(params)
    }

    async fn body<T: DeserializeOwned + 'static>(self) -> Result<T, anyhow::Error> {
        let Json(value) = self
            .extract()
            .await
            .map_err(|e| anyhow!("Invalid JSON body: {e}"))?;
        Ok(value)
    }

    async fn form<T: DeserializeOwned + 'static>(self) -> Result<T, anyhow::Error> {
        let Form(value) = self
            .extract()
            .await
            .map_err(|_| anyhow!("Invalid form body"))?;
        Ok(value)
    }
}
