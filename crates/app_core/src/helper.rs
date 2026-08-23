use time::{OffsetDateTime, PrimitiveDateTime};

pub fn now() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();
    PrimitiveDateTime::new(now.date(), now.time())
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
                @let url = format!("/{file}");
                @if url.ends_with(".css") {
                    <link rel="stylesheet" href=(url)>
                } @else {
                    <script src=(url) type="module"></script>
                }
            }
        }
    }
}
