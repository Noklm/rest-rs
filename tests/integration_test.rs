use std::time::Duration;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use http_body_util::BodyExt;
use sqlx::postgres::PgPoolOptions;
// for `collect`
use tower::ServiceExt;

/// Example axum testing: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
#[tokio::test]
async fn hello_world() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgresql://axum:password@127.0.0.1:5432/dbdev")
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/", get(rest_rs::hello_world))
        .with_state(pool);

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World!");
}
