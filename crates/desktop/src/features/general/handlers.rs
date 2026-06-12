use app_core::helper::markup_errors::{server_error, AppResponse};
use axum::response::{IntoResponse, Redirect};

pub async fn init() -> AppResponse {
    let token = keyring_core::Entry::new("com.harm2.desktop", "session_token")
        .map_err(server_error)?
        .get_password();

    match token {
        Ok(_) => Ok(Redirect::to("/").into_response()),
        Err(_) => Ok(Redirect::to("/sign-up").into_response()),
    }
}
