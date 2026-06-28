use axum::{http::StatusCode, response::Response, Json};
use maud::Markup;

pub type AppResponse = Result<Response, (StatusCode, Markup)>;
pub type ApiResponse = Result<Response, (StatusCode, Json<serde_json::Value>)>;

pub mod api_errors {
    use axum::{Json, http::StatusCode};
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
    use maud::{html, Markup};
    use std::error::Error;

    pub fn bad_request(msg: Option<&str>) -> (StatusCode, Markup) {
        let markup = html! {
            h1 { "Bad Request" }
            p {
                (msg.unwrap_or(""))
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

pub mod vite {
    use maud::{html, Markup};
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

    pub fn load_manifest_entry(file_name: &str) -> Markup {
        let manifest = match load_manifest() {
            Ok(manifest) => manifest,
            Err(_) => return html! {},
        };

        match manifest.get(file_name) {
            Some(entry) => {
                if entry.file.ends_with(".css") {
                    let markup = html! { link rel="stylesheet" href=(entry.file) {}; };
                    markup
                } else {
                    let markup = html! { script src=(entry.file) type="module" {}; };
                    markup
                }
            }
            None => html! {},
        }
    }
}
