use axum::Router;
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod controllers;

use crate::controllers::hello::hello_routes;

use self::controllers::hello::Hello;

#[derive(OpenApi)]
#[openapi(paths(controllers::hello::handler), components(schemas(Hello)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Logging - Filter to INFO and above
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(hello_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);
    // Bind a TCP listener to the address.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
