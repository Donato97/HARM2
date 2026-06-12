use time::Duration;

use app_core::{AppState, CustomPool};
use axum::routing::post;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;

use crate::features::auth::handlers::{sign_in, sign_out, sign_up};

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

    let routes = app_core::router()
        .route("/sign-up", post(sign_up))
        .route("/sign-in", post(sign_in))
        .route("/sign-out", post(sign_out))
        .layer(session_layer)
        .with_state(AppState {
            pool: CustomPool::Sqlite(pool),
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server in ascolto su http://localhost:3000");
    axum::serve(listener, routes).await.unwrap();
}
