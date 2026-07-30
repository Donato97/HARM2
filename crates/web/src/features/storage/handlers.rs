use super::services::StorageService;
use app_core::{features::auth::models::SessionUser, responses::api::ApiResponse};
use axum::{extract::Multipart, response::IntoResponse, Extension, Json};

pub async fn upload(Extension(user): Extension<SessionUser>, mut files: Multipart) -> ApiResponse {
    let service = StorageService::new();
    let path = format!("storage/{}", user.id);

    while let Ok(field) = files.next_field().await {
        if let Some(file) = field {
            if file.name() != Some("file") {
                continue;
            }

            let data = file.bytes().await?;
            let path = service.save(&path, data).await?;

            let response_json = serde_json::json!({
                "url": format!("/{path}")
            });
            return Ok(Json(response_json).into_response());
        }
    }
    Ok(().into_response())
}
