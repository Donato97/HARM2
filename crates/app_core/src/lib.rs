pub mod helper;
pub mod state;
pub mod features {
    pub mod auth {
        pub mod handlers;
        pub mod models;
        pub mod views;
    }
    pub mod generic {
        pub mod views;
    }
}
pub mod layouts {
    pub mod auth;
    pub mod default;

    pub use auth::auth;
    pub use default::default;
}

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub use state::{AppState, CustomPool};

use crate::features::{auth::handlers, generic::views::index};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/sign-up", get(handlers::sign_up))
        .route("/sign-in", get(handlers::sign_in))
        .nest_service("/assets", ServeDir::new("dist/assets"))
}
