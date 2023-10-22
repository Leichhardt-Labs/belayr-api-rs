use crate::controllers::goodbye::goodbye_routes;
use crate::controllers::hello::hello_routes;
use crate::controllers::profiles::profile_routes;
use crate::util::logging::LoggingRouterExt;
use axum::Router;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenv::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod models;
mod schema;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Logging - Filter to INFO and above
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "belayr_api_rs=debug,tower_http=info,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    // Add DB connection pool
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let app = Router::new()
        .merge(goodbye_routes(pool.clone()))
        .merge(hello_routes(pool.clone()))
        .merge(profile_routes(pool.clone()))
        .add_logging();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    // Bind a TCP listener to the address.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
