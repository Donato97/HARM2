use super::*;
use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use tower::ServiceExt;

pub async fn test_app() -> Router {
    dotenvy::dotenv().ok();
    let opts = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(opts)
        .await
        .unwrap();

    sqlx::migrate!("../app_core/migrations")
        .run(&pool)
        .await
        .unwrap();

    let store = SqliteStore::new(pool.clone());
    store.migrate().await.unwrap();

    router(
        AppState {
            pool: CustomPool::Sqlite(pool),
            http_client: reqwest::Client::new(),
        },
        SessionManagerLayer::new(store),
    )
}

fn post_form(uri: &str, body: &'static str) -> Request<Body> {
    Request::post(uri)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(body))
        .unwrap()
}

async fn login(app: &Router) -> String {
    let res = app
        .clone()
        .oneshot(post_form(
            "/sign-up",
            "email=tizio@test.it&password=segretissima",
        ))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::SEE_OTHER, "sign-up fallito");

    res.headers()
        .get(header::SET_COOKIE)
        .expect("nessun cookie di sessione")
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_owned()
}

#[tokio::test]
async fn il_login_accetta_una_form_urlencoded() {
    let app = test_app().await;

    let res = app
        .clone()
        .oneshot(post_form(
            "/sign-up",
            "email=tizio@test.it&password=segretissima",
        ))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::SEE_OTHER);

    let res = app
        .oneshot(post_form(
            "/sign-in",
            "email=tizio@test.it&password=segretissima",
        ))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::SEE_OTHER);
    assert_eq!(res.headers()[header::LOCATION], "/");
    assert!(res.headers().contains_key(header::SET_COOKIE));
}

#[tokio::test]
async fn il_login_rifiuta_un_body_json() {
    let res = test_app()
        .await
        .oneshot(
            Request::post("/sign-in")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"email":"tizio@test.it","password":"segretissima"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(res.status(), StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn lettura_storage_altrui() {
    let app = test_app().await;
    let cookie = login(&app).await;

    let res = app
        .oneshot(
            Request::get("/storage/2/0c5e275f-32ed-4af1-a2e6-332b8ce620ce.png")
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::NOT_FOUND, res.status())
}

#[tokio::test]
async fn lettura_storage() {
    let app = test_app().await;
    let cookie = login(&app).await;

    let res = app
        .oneshot(
            Request::get("/storage/1/0c5e275f-32ed-4af1-a2e6-332b8ce620ce.png")
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, res.status());
    assert_eq!(res.headers()[header::CONTENT_TYPE], "image/png");
    assert_eq!(
        res.headers()[header::CACHE_CONTROL],
        "public, max-age=31536000, immutable"
    );
}
