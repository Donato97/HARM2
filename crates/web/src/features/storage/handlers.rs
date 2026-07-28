use app_core::{
    features::auth::models::SessionUser,
    helper::{
        api_errors::{bad_request, server_error},
        ApiResponse,
    },
};
use axum::{extract::Multipart, response::IntoResponse, Extension, Json};

pub async fn upload(Extension(user): Extension<SessionUser>, mut files: Multipart) -> ApiResponse {
    while let Ok(field) = files.next_field().await.map_err(server_error) {
        if let Some(file) = field {
            if file.name() != Some("file") {
                continue;
            }

            let data = file
                .bytes()
                .await
                .map_err(|e| bad_request(Some(&e.to_string())))?;

            let kind =
                infer::get(&data).ok_or_else(|| bad_request(Some("Mime type not supported")))?;
            let ext = ext_from_mime(kind.mime_type())
                .ok_or_else(|| bad_request(Some("Mime type not supported")))?;

            let filename = format!("{}.{}", uuid::Uuid::new_v4(), ext);

            let path = std::path::Path::new(&format!("storage/{}", user.id)).join(&filename);
            if let Some(dir) = path.parent() {
                tokio::fs::create_dir_all(dir).await.map_err(server_error)?;
            }

            tokio::fs::write(&path, data).await.map_err(server_error)?;

            let response_json = serde_json::json!({
                "url": format!("/storage/{}/{}", user.id, filename)
            });
            return Ok(Json(response_json).into_response());
        }
    }
    Ok(().into_response())
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
