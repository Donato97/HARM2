use super::auth::models::SessionUser;
use crate::{responses::Error, state::AppState};
use anyhow::anyhow;
use axum::{
    extract::{Path, Query, Request},
    Form, Json, RequestExt,
};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, future::Future};

pub trait RequestContext {
    fn state(&mut self) -> Result<AppState, Error>;
    fn user(&mut self) -> Result<SessionUser, Error>;
    fn path<T: DeserializeOwned + Send + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<T, Error>>;
    fn params(&mut self) -> impl Future<Output = Result<HashMap<String, String>, Error>>;
    fn body<T: DeserializeOwned + 'static>(self) -> impl Future<Output = Result<T, Error>>;
    fn form<T: DeserializeOwned + 'static>(self) -> impl Future<Output = Result<T, Error>>;
}

impl RequestContext for Request {
    fn state(&mut self) -> Result<AppState, Error> {
        self.extensions()
            .get::<AppState>()
            .cloned()
            .ok_or(Error::Internal(anyhow!("AppState extension not found")))
    }

    fn user(&mut self) -> Result<SessionUser, Error> {
        self.extensions()
            .get::<SessionUser>()
            .cloned()
            .ok_or(Error::Internal(anyhow!("SessionUser extension not found")))
    }

    async fn path<T: DeserializeOwned + Send + 'static>(&mut self) -> Result<T, Error> {
        let Path(value) = self.extract_parts::<Path<T>>().await?;
        Ok(value)
    }

    async fn params(&mut self) -> Result<HashMap<String, String>, Error> {
        let Query(params) = self.extract_parts::<Query<_>>().await?;

        Ok(params)
    }

    async fn body<T: DeserializeOwned + 'static>(self) -> Result<T, Error> {
        let Json(value) = self.extract().await?;
        Ok(value)
    }

    async fn form<T: DeserializeOwned + 'static>(self) -> Result<T, Error> {
        let Form(value) = self.extract().await?;
        Ok(value)
    }
}
