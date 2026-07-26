use axum::{http::StatusCode, response::Response, Json};
use hypertext::prelude::*;

pub type AppResponse = Result<Response, (StatusCode, Rendered<String>)>;
pub type ApiResponse = Result<Response, (StatusCode, Json<serde_json::Value>)>;

pub mod api_errors {
    use axum::{http::StatusCode, Json};
    use serde_json::json;

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

    pub fn server_error(e: impl std::error::Error) -> (StatusCode, Json<serde_json::Value>) {
        let body = json!({
            "error": "Server Error",
            "message": e.to_string()
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
    }
}

pub mod markup_errors {
    use axum::http::StatusCode;
    use hypertext::prelude::*;
    use std::error::Error;

    pub fn bad_request(msg: Option<&str>) -> (StatusCode, Rendered<String>) {
        let markup = rsx! {
            <h1> "Bad Request" </h1>
            <p> (msg.unwrap_or("")) </p>
        }
        .render();

        (StatusCode::BAD_REQUEST, markup)
    }

    pub fn not_found() -> (StatusCode, Rendered<String>) {
        let markup = rsx! {
            <h1> "Not Found" </h1>
        }
        .render();

        (StatusCode::NOT_FOUND, markup)
    }

    pub fn server_error(e: impl Error) -> (StatusCode, Rendered<String>) {
        let markup = rsx! {
            <h1> "Server Error" </h1>
            <p> (e.to_string()) </p>
        }
        .render();

        (StatusCode::INTERNAL_SERVER_ERROR, markup)
    }
}

pub mod vite {
    use hypertext::prelude::*;
    use serde::Deserialize;
    use std::{collections::HashMap, error::Error};

    #[derive(Deserialize, Debug)]
    struct ManifestEntry {
        pub file: String,
    }

    type Manifest = HashMap<String, ManifestEntry>;

    fn load_manifest() -> Result<Manifest, Box<dyn Error>> {
        let manifest_path = "dist/.vite/manifest.json";
        let manifest = std::fs::read_to_string(manifest_path)?;
        let json: Manifest = serde_json::from_str(&manifest)?;
        Ok(json)
    }

    pub fn load_manifest_entry(file_name: &str) -> impl Renderable {
        let file = load_manifest()
            .ok()
            .and_then(|m| m.get(file_name).map(|e| e.file.clone()));

        rsx! {
            @if let Some(file) = &file {
                @if file.ends_with(".css") {
                    <link rel="stylesheet" href=(file)>
                } @else {
                    <script src=(file) type="module"></script>
                }
            }
        }
    }
}
