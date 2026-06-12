use serde::Deserialize;
use std::collections::HashMap;

pub mod markup_errors {
    use std::error::Error;

    use axum::{http::StatusCode, response::Response};
    use maud::{html, Markup};

    pub type AppResponse = Result<Response, (StatusCode, Markup)>;

    pub fn bad_request(mdg: Option<&str>) -> (StatusCode, Markup) {
        let markup = html! {
            h1 { "Bad Request" }
            p {
                (mdg.unwrap_or(""))
            }
        };

        (StatusCode::BAD_REQUEST, markup)
    }

    pub fn not_found() -> (StatusCode, Markup) {
        let markup = html! {
            h1 { "Not Found" }
        };

        (StatusCode::NOT_FOUND, markup)
    }

    pub fn server_error(e: impl Error) -> (StatusCode, Markup) {
        let markup = html! {
            h1 { "Server Error" }
            p { (e.to_string()) }
        };

        (StatusCode::INTERNAL_SERVER_ERROR, markup)
    }
}

#[derive(Deserialize, Debug)]
pub struct ManifestEntry {
    pub file: String,
    pub css: Option<Vec<String>>,
}

pub type Manifest = HashMap<String, ManifestEntry>;

pub fn load_manifest() -> Result<Manifest, String> {
    let manifest_path = "dist/.vite/manifest.json";
    let manifest = std::fs::read_to_string(manifest_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&manifest).map_err(|e| e.to_string())
}
