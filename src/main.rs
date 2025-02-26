mod settings;
use std::time::Duration;

use axum::{
    extract::{MatchedPath, Request},
    response::Response,
    routing::get,
    Router,
};
use settings::Settings;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let settings = Settings::new("settings", "APP").expect("Bad configuration");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&settings.database.url)
        .await
        .expect("can't connect to database");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}={},tower_http=info,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME"),
                    settings.log_level
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(rest_rs::hello_world))
        .with_state(pool)
        .route("/error", get(rest_rs::hello_error))
        .nest("/api", rest_rs::handlers::routes())
        // .merge(app::handlers::routes())
        // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
        // It provides good defaults but is also very customizable.
        //
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        //
        // If you want to customize the behavior using closures here is how.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        latency = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                    tracing::debug!("{} {}", _request.method(), _request.uri().path())
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    _span.record("latency", format!("{:?}", _latency));
                    tracing::debug!("{}", _response.status())
                }),
        )
        .layer(tower_http::catch_panic::CatchPanicLayer::new());

    // run it
    let listener = TcpListener::bind(settings.url)
        .await
        .expect("Failed to bind TCP listener");

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("axum::serve failed");
}
