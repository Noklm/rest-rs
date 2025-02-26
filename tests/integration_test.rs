use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use http_body_util::BodyExt; // for `collect`
use tower::ServiceExt;

/// Example axum testing: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
#[tokio::test]
async fn hello_world() {
    let app = Router::new().route("/", get(rest_rs::hello_world));

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
