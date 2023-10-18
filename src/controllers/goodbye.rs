use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use diesel_async::RunQueryDsl;
use serde::Serialize;
use tracing::info;
use utoipa::ToSchema;

use crate::{
    models::ClimbLocation,
    util::common::{internal_error, Pool},
};

#[derive(Serialize, ToSchema)]
pub struct Hello {
    message: String,
}

pub fn goodbye_routes(db_pool: Pool) -> Router {
    Router::new()
        .route("/goodbye/:name", get(handler))
        .route("/lecations", get(first_location))
        .with_state(db_pool)
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
    info!("Hello, {}!", name);
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
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))),
    }
}
