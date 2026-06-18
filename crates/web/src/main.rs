use time::Duration;

use app_core::{AppState, CustomPool};
use axum::{middleware, routing::post};
use tower_http::services::ServeDir;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;

use crate::features::auth;

pub mod routes;
pub mod features {
    pub mod auth {
        pub mod handlers;
    }
}

#[tokio::main]
async fn main() {
    // In dev usa SQLite; in prod sostituisci con MySqlPool::connect(...).
    let pool = sqlx::SqlitePool::connect("sqlite://crates/web/db.sqlite")
        .await
        .expect("Connessione al DB fallita!");

    let session_store = SqliteStore::new(pool.clone());
    session_store.migrate().await;

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Creazione del client reqwest fallita!");

    let routes = app_core::router()
        .route("/sign-up", post(auth::handlers::sign_up))
        .route("/sign-in", post(auth::handlers::sign_in))
        .route("/sign-out", post(auth::handlers::sign_out))
        .layer(middleware::from_fn(auth::handlers::middleware))
        .layer(session_layer)
        .nest_service("/assets", ServeDir::new("dist/assets"))
        .with_state(AppState {
            pool: CustomPool::Sqlite(pool),
            http_client: client,
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server in ascolto su http://localhost:3000");
    axum::serve(listener, routes).await.unwrap();
}
