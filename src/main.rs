use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Serialize;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    // Logging - Filter to INFO and above
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        .route("/", get(handler))
        .route("/hello/:name", get(handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);
    // Bind a TCP listener to the address.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct Hello {
    message: String,
}

async fn handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(Hello {
            message: "Hello, World!".to_owned(),
        }),
    )
}
