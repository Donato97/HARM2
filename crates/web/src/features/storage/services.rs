use app_core::responses::api::ApiError;
use axum::body::Bytes;

pub struct StorageService;

impl StorageService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn save(&self, path: &str, data: Bytes) -> Result<String, ApiError> {
        let ext = self.type_or_throw(&data)?;

        let filename = format!("{}.{}", uuid::Uuid::new_v4(), ext);

        let path = std::path::Path::new(path).join(&filename);
        if let Some(dir) = path.parent() {
            tokio::fs::create_dir_all(dir).await?;
        }

        tokio::fs::write(&path, data).await?;

        Ok(path.to_string_lossy().into_owned())
    }

    fn type_or_throw(&self, data: &Bytes) -> Result<&str, ApiError> {
        let kind = infer::get(data).ok_or(ApiError::BadRequest("Mime type not supported"))?;
        let ext = self
            .ext_from_mime(kind.mime_type())
            .ok_or(ApiError::BadRequest("Mime type not supported"))?;

        Ok(ext)
    }

    fn ext_from_mime(&self, mime: &str) -> Option<&str> {
        match mime {
            "image/png" => Some("png"),
            "image/jpeg" => Some("jpg"),
            "image/webp" => Some("webp"),
            "image/gif" => Some("gif"),
            "image/avif" => Some("avif"),
            _ => None,
        }
    }
}
