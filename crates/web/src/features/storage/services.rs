use std::sync::OnceLock;

use app_core::responses::Error;
use axum::body::Bytes;

pub async fn save(path: &str, data: Bytes) -> Result<String, Error> {
    let ext = detect_extension(&data)?;

    let filename = format!("{}.{}", uuid::Uuid::new_v4(), ext);

    let path = std::path::Path::new(path).join(&filename);
    if let Some(dir) = path.parent() {
        tokio::fs::create_dir_all(dir).await?;
    }

    tokio::fs::write(&path, data).await?;

    Ok(filename)
}

pub fn storage_root() -> &'static str {
    static ROOT: OnceLock<String> = OnceLock::new();
    ROOT.get_or_init(|| std::env::var("STORAGE_ROOT").expect("STORAGE_ROOT non impostata"))
}

fn detect_extension(data: &Bytes) -> Result<&str, Error> {
    let kind = infer::get(data).ok_or(Error::BadRequest("Mime type not supported".into()))?;
    let ext = ext_from_mime(kind.mime_type())
        .ok_or(Error::BadRequest("Mime type not supported".into()))?;

    Ok(ext)
}

fn ext_from_mime(mime: &str) -> Option<&'static str> {
    match mime {
        "image/png" => Some("png"),
        "image/jpeg" => Some("jpg"),
        "image/webp" => Some("webp"),
        "image/gif" => Some("gif"),
        "image/avif" => Some("avif"),
        _ => None,
    }
}
