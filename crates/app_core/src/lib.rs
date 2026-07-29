pub mod helper;
pub mod state;
pub mod features {
    pub mod auth {
        pub mod handlers;
        pub mod models;
        pub mod repositories;
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
    pub mod file_system {
        pub mod handlers;
        pub mod models;
    }
    pub mod notes {
        pub mod handlers;
        pub mod models;
    }
}
pub mod layouts {
    pub mod auth;
    pub mod default;

    pub use auth::auth;
    pub use default::default;
}

use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post, put},
    Router,
};

pub use state::{AppState, CustomPool};

use crate::features::{auth, file_system, generic, movies, notes};

pub fn router() -> Router<AppState> {
    let notes_router = Router::new()
        .route("/api/files/{id}", put(notes::handlers::update))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)); // 10 MB

    let other_router = Router::new()
        .route("/", get(generic::views::index))
        .route("/movies", get(movies::handlers::index))
        .route("/sign-up", get(auth::handlers::sign_up))
        .route("/sign-in", get(auth::handlers::sign_in))
        .route("/api/filesystem", get(file_system::handlers::index))
        .route(
            "/api/filesystem",
            post(file_system::handlers::create_or_update),
        )
        .route("/api/filesystem/{id}", put(file_system::handlers::update))
        .route(
            "/api/filesystem/{id}",
            delete(file_system::handlers::delete),
        )
        .route("/api/files/{id}", get(notes::handlers::find));

    Router::new().merge(notes_router).merge(other_router)
}
