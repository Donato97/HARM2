use crate::features::storage::services::storage_root;

use super::services::save;
use app_core::{
    features::auth::models::SessionUser,
    request::RequestContext,
    responses::{api::ApiResponse, Error},
};
use axum::{
    extract::{Multipart, Request},
    http::header,
    response::IntoResponse,
    Extension, Json,
};

pub async fn read(mut req: Request) -> ApiResponse {
    let (user_path, file_name): (u64, String) = req.path().await?;
    let user = req.user()?;
    if user_path != user.id {
        return Err(Error::NotFound.into());
    }

    let storage_root = storage_root();
    let base = tokio::fs::canonicalize(format!("{storage_root}/{user_path}"))
        .await
        .map_err(|_| Error::NotFound)?;
    let target = tokio::fs::canonicalize(format!("{storage_root}/{user_path}/{file_name}"))
        .await
        .map_err(|_| Error::NotFound)?;

    if !target.starts_with(&base) {
        return Err(Error::NotFound.into());
    }
    let data = tokio::fs::read(target).await.map_err(|_| Error::NotFound)?;
    let mime = infer::get(&data)
        .map(|kind| kind.mime_type())
        .ok_or(Error::NotFound)?;

    let headers = [
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        (header::CONTENT_TYPE, mime),
        (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
    ];

    Ok((headers, data).into_response())
}

pub async fn upload(Extension(user): Extension<SessionUser>, mut files: Multipart) -> ApiResponse {
    let storage_root = storage_root();
    let path = format!("{}/{}", storage_root, user.id);

    while let Some(file) = files.next_field().await? {
        if file.name() != Some("file") {
            continue;
        }

        let data = file.bytes().await?;
        let filename = save(&path, data).await?;

        let response_json = serde_json::json!({
            "url": format!("/storage/{}/{}", user.id, filename)
        });
        return Ok(Json(response_json).into_response());
    }
    Ok(().into_response())
}
