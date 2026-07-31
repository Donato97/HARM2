use app_core::responses::api::ApiError;
use axum::body::Bytes;

pub async fn save(path: &str, data: Bytes) -> Result<String, ApiError> {
    let ext = detect_extension(&data)?;

    let filename = format!("{}.{}", uuid::Uuid::new_v4(), ext);

    let path = std::path::Path::new(path).join(&filename);
    if let Some(dir) = path.parent() {
        tokio::fs::create_dir_all(dir).await?;
    }

    tokio::fs::write(&path, data).await?;

    Ok(path.to_string_lossy().into_owned())
}

fn detect_extension(data: &Bytes) -> Result<&str, ApiError> {
    let kind = infer::get(data).ok_or(ApiError::BadRequest("Mime type not supported"))?;
    let ext =
        ext_from_mime(kind.mime_type()).ok_or(ApiError::BadRequest("Mime type not supported"))?;

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
