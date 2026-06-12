use app_core::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().route("/sign-in", axum::routing::get(|| async { "" }))
}
