use std::borrow::Cow;

use axum::extract::{
    multipart::MultipartError,
    rejection::{FormRejection, JsonRejection, PathRejection, QueryRejection},
};

pub enum Error {
    NotFound,
    BadRequest(Cow<'static, str>),
    Internal(anyhow::Error),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Error::NotFound, // 404
            other => Error::Internal(other.into()),      // 500
        }
    }
}

impl From<MultipartError> for Error {
    fn from(e: MultipartError) -> Self {
        Error::BadRequest(e.body_text().into())
    }
}

impl From<PathRejection> for Error {
    fn from(e: PathRejection) -> Self {
        Error::BadRequest(e.body_text().into())
    }
}

impl From<QueryRejection> for Error {
    fn from(e: QueryRejection) -> Self {
        Error::BadRequest(e.body_text().into())
    }
}

impl From<JsonRejection> for Error {
    fn from(e: JsonRejection) -> Self {
        Error::BadRequest(e.body_text().into())
    }
}

impl From<FormRejection> for Error {
    fn from(e: FormRejection) -> Self {
        Error::BadRequest(e.body_text().into())
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Internal(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => Error::NotFound,
            _ => Error::Internal(e.into()),
        }
    }
}

pub mod markup {
    use super::Error;
    use axum::response::{IntoResponse, Response};
    use hypertext::prelude::*;
    use reqwest::StatusCode;

    pub type AppResponse = Result<Response, AppError>;

    pub struct AppError(pub Error);

    impl<E: Into<Error>> From<E> for AppError {
        fn from(e: E) -> Self {
            Self(e.into())
        }
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            let (status, body) = match self.0 {
                Error::NotFound => not_found(),
                Error::BadRequest(msg) => bad_request(Some(&msg)),
                Error::Internal(err) => {
                    eprintln!("{err:?}"); // o tracing::error!
                    server_error()
                }
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

    fn server_error() -> (StatusCode, Rendered<String>) {
        let markup = rsx! {
            <h1> "Server Error" </h1>
        }
        .render();

        (StatusCode::INTERNAL_SERVER_ERROR, markup)
    }
}

pub mod api {
    use super::Error;
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use serde_json::json;

    pub type ApiResponse = Result<Response, ApiError>;

    pub struct ApiError(pub Error);

    impl<E: Into<Error>> From<E> for ApiError {
        fn from(e: E) -> Self {
            Self(e.into())
        }
    }

    impl IntoResponse for ApiError {
        fn into_response(self) -> Response {
            let (status, body) = match self.0 {
                Error::NotFound => not_found(),
                Error::BadRequest(msg) => bad_request(Some(&msg)),
                Error::Internal(err) => {
                    eprintln!("{err:?}"); // o tracing::error!
                    server_error()
                }
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

    pub fn server_error() -> (StatusCode, Json<serde_json::Value>) {
        let body = json!({
            "error": "Server Error"
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
    }
}
