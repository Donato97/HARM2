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
    pub mod games {
        pub mod handlers;
        pub mod views;
    }
    pub mod movies {
        pub mod handlers;
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

pub use state::{AppState, CustomPool};

use crate::features::{auth, games, generic::views::index, movies};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/games", get(games::handlers::index))
        .route("/movies", get(movies::handlers::index))
        .route("/sign-up", get(auth::handlers::sign_up))
        .route("/sign-in", get(auth::handlers::sign_in))
}
