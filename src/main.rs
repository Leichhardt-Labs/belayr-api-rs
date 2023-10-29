use crate::controllers::profiles::profile_routes;
use crate::repositories::profile_repo::{ProfileRepo, ProfileRepository};
use crate::repositories::session_repo::{SessionRepo, SessionRepository};
use crate::util::logging::LoggingRouterExt;
use axum::Router;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenv::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod models;
mod repositories;
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

    let profile_repo = Arc::new(ProfileRepository { pool: pool.clone() }) as ProfileRepo;
    let session_repo = Arc::new(SessionRepository { pool: pool.clone() }) as SessionRepo;

    let app = Router::new()
        .merge(profile_routes(profile_repo.clone(), session_repo.clone()))
        .add_logging();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    // Bind a TCP listener to the address.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
