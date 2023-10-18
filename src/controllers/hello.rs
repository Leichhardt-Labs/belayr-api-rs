use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use diesel_async::RunQueryDsl;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    models::ClimbLocation,
    util::{
        common::{internal_error, Pool},
        logging::LoggingRouterExt,
    },
};

#[derive(Serialize, ToSchema)]
pub struct Hello {
    message: String,
}

pub fn hello_routes(db_pool: Pool) -> Router {
    Router::new()
        .route("/hello/:name", get(handler))
        .route("/locations", get(first_location))
        .with_state(db_pool)
        .add_logging()
}

#[utoipa::path(
    get,
    request_body = HelloBody,
    path = "/hello/{name}",
    responses(
        (status = 200, body = Hello)
    )
)]
pub async fn handler(Path(name): Path<String>) -> impl IntoResponse {
    tracing::info!("Hello, {}!", name);
    (
        StatusCode::OK,
        Json(Hello {
            message: format!("Hello, {}!", name),
        }),
    )
}

pub async fn first_location(
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    use crate::schema::climb_locations::dsl::*;

    let mut conn = pool.get().await.map_err(internal_error)?;

    let first_location = climb_locations.first::<ClimbLocation>(&mut conn).await;

    match first_location {
        Ok(first_location) => Ok(Json(first_location)),
        Err(diesel::result::Error::NotFound) => {
            Err((StatusCode::NOT_FOUND, "No locations found".to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
