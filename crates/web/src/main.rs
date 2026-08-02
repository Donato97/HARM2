use crate::features::{auth, games, storage};
use app_core::{state::AppState, CustomPool};
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use time::Duration;
use tower_http::services::ServeDir;
use tower_sessions::{cookie::SameSite, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;

#[cfg(test)]
mod tests;

pub mod request;
pub mod features {
    pub mod auth {
        pub mod handlers;
        pub mod service;
    }
    pub mod games {
        pub mod handlers;
        pub mod services;
        pub mod views;
    }
    pub mod storage {
        pub mod handlers;
        pub mod services;
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // In dev usa SQLite; in prod sostituisci con MySqlPool::connect(...).
    let opts = SqliteConnectOptions::from_str("sqlite://crates/web/db.sqlite")
        .expect("Errore nella configurazione del DB!")
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(opts)
        .await
        .expect("Connessione al DB fallita!");

    let session_store = SqliteStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("Migrazione della sessione DB fallita!");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_same_site(SameSite::Lax)
        .with_secure(!cfg!(debug_assertions))
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Creazione del client reqwest fallita!");

    let state = AppState {
        pool: CustomPool::Sqlite(pool),
        http_client: client,
    };

    let routes = router(state, session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server in ascolto su http://localhost:3000");
    axum::serve(listener, routes).await.unwrap();
}

fn router(state: AppState, session_layer: SessionManagerLayer<SqliteStore>) -> Router {
    app_core::router()
        .route("/sign-up", post(auth::handlers::sign_up))
        .route("/sign-in", post(auth::handlers::sign_in))
        .route("/sign-out", post(auth::handlers::sign_out))
        .route("/games", get(games::handlers::index))
        .route("/steam-login", get(games::handlers::steam_login))
        .route("/storage/upload", post(storage::handlers::upload))
        .route(
            "/storage/{user_id}/{file_name}",
            get(storage::handlers::read),
        )
        .layer(middleware::from_fn(auth::handlers::middleware))
        .layer(session_layer)
        .nest_service("/assets", ServeDir::new("dist/assets"))
        .layer(Extension(state))
}
