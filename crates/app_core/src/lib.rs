pub mod helper;
pub mod request;
pub mod responses;
pub mod state;
pub mod features {
    pub mod auth {
        pub mod handlers;
        pub mod models;
        pub mod repositories;
        pub mod views;
    }
    pub mod editor {
        pub mod handlers;
        pub mod models;
        pub mod repositories;
        pub mod services;
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

pub use state::CustomPool;

use crate::features::{auth, editor, movies, notes};

pub fn router() -> Router {
    let notes_router = Router::new()
        .route("/api/file/{id}", put(notes::handlers::update))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)); // 10 MB

    let other_router = Router::new()
        .route("/", get(editor::handlers::index))
        .route("/movies", get(movies::handlers::index))
        .route("/sign-up", get(auth::handlers::sign_up))
        .route("/sign-in", get(auth::handlers::sign_in))
        .route("/filesystem", post(editor::handlers::create))
        .route("/filesystem/{id}", put(editor::handlers::update))
        .route("/filesystem/{id}", delete(editor::handlers::delete))
        .route("/file/{id}", get(editor::handlers::index))
        .route("/api/file/{id}", get(notes::handlers::find))
        .route("/api/file/{id}/save", put(notes::handlers::update));

    Router::new().merge(notes_router).merge(other_router)
}
