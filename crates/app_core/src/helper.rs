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
