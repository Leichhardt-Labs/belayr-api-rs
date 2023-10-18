use axum::Router;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use self::controllers::hello::Hello;
use crate::controllers::hello::hello_routes;

mod controllers;

#[derive(OpenApi)]
#[openapi(paths(controllers::hello::handler), components(schemas(Hello)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Logging - Filter to INFO and above
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    // Add DB connection pool
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let app = Router::new()
        .with_state(pool)
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
