#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_core::{AppState, CustomPool};

mod features {
    pub mod auth {
        pub mod handlers;
    }
    pub mod general {
        pub mod handlers;
    }
}

use crate::features::auth;
use axum::middleware;
use axum::routing::{get, post};
use features::general::handlers::init;
use keyring_core::set_default_store;
use tower_http::services::ServeDir;
use windows_native_keyring_store::Store;

#[tokio::main]
async fn main() {
    set_default_store(Store::new().expect("keyring core error"));

    let pool = sqlx::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Connessione al DB fallita!");

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Creazione del client reqwest fallita!");

    let router = app_core::router()
        .route("/init", get(init))
        .route("/sign-up", post(auth::handlers::sign_up))
        .route("/sign-in", post(auth::handlers::sign_in))
        .route("/sign-out", post(auth::handlers::sign_out))
        .nest_service("/assets", ServeDir::new("dist/assets"))
        .layer(middleware::from_fn(auth::handlers::middleware))
        .with_state(AppState {
            pool: CustomPool::Sqlite(pool),
            http_client: client,
        });

    tauri::Builder::default()
        .plugin(tauri_plugin_axum::init(router))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
