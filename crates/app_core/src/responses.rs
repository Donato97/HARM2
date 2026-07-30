pub mod markup {
    use std::fmt::Display;

    use axum::response::{IntoResponse, Response};
    use hypertext::prelude::*;
    use reqwest::StatusCode;

    pub type AppResponse = Result<Response, AppError>;

    pub enum AppError {
        NotFound,
        BadRequest(&'static str),
        InternalServerError(anyhow::Error),
    }

    impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
    {
        fn from(err: E) -> Self {
            AppError::InternalServerError(err.into())
        }
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            let (status, body) = match self {
                AppError::NotFound => not_found(),
                AppError::InternalServerError(err) => server_error(err),
                AppError::BadRequest(msg) => bad_request(Some(msg)),
            };

            (status, body).into_response()
        }
    }

    fn bad_request(msg: Option<&str>) -> (StatusCode, Rendered<String>) {
        let markup = rsx! {
            <h1> "Bad Request" </h1>
            <p> (msg.unwrap_or("")) </p>
        }
        .render();

        (StatusCode::BAD_REQUEST, markup)
    }

    fn not_found() -> (StatusCode, Rendered<String>) {
        let markup = rsx! {
            <h1> "Not Found" </h1>
        }
        .render();

        (StatusCode::NOT_FOUND, markup)
    }

    fn server_error(e: impl Display) -> (StatusCode, Rendered<String>) {
        let markup = rsx! {
            <h1> "Server Error" </h1>
            <p> (e.to_string()) </p>
        }
        .render();

        (StatusCode::INTERNAL_SERVER_ERROR, markup)
    }
}

pub mod api {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use serde_json::json;

    pub type ApiResponse = Result<Response, ApiError>;

    pub enum ApiError {
        NotFound,
        BadRequest(&'static str),
        InternalServerError(anyhow::Error),
    }

    impl<E> From<E> for ApiError
    where
        E: Into<anyhow::Error>,
    {
        fn from(err: E) -> Self {
            ApiError::InternalServerError(err.into())
        }
    }

    impl IntoResponse for ApiError {
        fn into_response(self) -> Response {
            let (status, body) = match self {
                ApiError::NotFound => not_found(),
                ApiError::InternalServerError(err) => server_error(err),
                ApiError::BadRequest(msg) => bad_request(Some(msg)),
            };

            (status, body).into_response()
        }
    }

    pub fn bad_request(msg: Option<&str>) -> (StatusCode, Json<serde_json::Value>) {
        let body = json!({
            "error": "Bad Request",
            "message": msg.unwrap_or("")
        });

        (StatusCode::BAD_REQUEST, Json(body))
    }

    pub fn not_found() -> (StatusCode, Json<serde_json::Value>) {
        let body = json!({
            "error": "Not Found"
        });

        (StatusCode::NOT_FOUND, Json(body))
    }

    pub fn server_error(e: impl std::fmt::Display) -> (StatusCode, Json<serde_json::Value>) {
        let body = json!({
            "error": "Server Error",
            "message": e.to_string()
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
    }
}
