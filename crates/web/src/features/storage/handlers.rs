use super::services::save;
use app_core::{features::auth::models::SessionUser, responses::api::ApiResponse};
use axum::{extract::Multipart, response::IntoResponse, Extension, Json};

pub async fn upload(Extension(user): Extension<SessionUser>, mut files: Multipart) -> ApiResponse {
    let path = format!("storage/{}", user.id);

    while let Some(file) = files.next_field().await? {
        if file.name() != Some("file") {
            continue;
        }

        let data = file.bytes().await?;
        let path = save(&path, data).await?;

        let response_json = serde_json::json!({
            "url": format!("/{path}")
        });
        return Ok(Json(response_json).into_response());
    }
    Ok(().into_response())
}
