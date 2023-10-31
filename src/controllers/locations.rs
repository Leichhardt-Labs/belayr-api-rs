use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{
        generic_models::PagedResponse,
        location_models::{GetLocationsRequest, LocationDetailsResponse, LocationSummary},
    },
    repositories::location_repo::LocationRepo,
    util::{common::RepoError, logging::LoggingRouterExt},
};

pub fn location_routes(location_repo: LocationRepo) -> Router {
    Router::new()
        .route("/location/:id/details", get(get_location_details))
        .route("/locations", get(get_locations))
        .with_state(location_repo)
        .add_logging()
}

pub async fn get_location_details(
    Path(id): Path<Uuid>,
    State(location_repo): State<LocationRepo>,
) -> Result<Json<LocationDetailsResponse>, (StatusCode, String)> {
    let location = location_repo
        .get_location(id)
        .await
        .map_err(|err| match err {
            RepoError::NotFound => (StatusCode::NOT_FOUND, "Location not found".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        })?;

    Ok(Json(location.into()))
}

pub async fn get_locations(
    State(location_repo): State<LocationRepo>,
    Query(request): Query<GetLocationsRequest>,
) -> Result<Json<PagedResponse<LocationSummary>>, (StatusCode, String)> {
    // validate request
    request
        .validate()
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("Invalid request: {}", err)))?;

    let locations = location_repo
        .get_paged_locations(request.page, request.page_size)
        .await
        .map_err(|err| match err {
            RepoError::NotFound => (StatusCode::NOT_FOUND, "Locations not found".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        })?;

    let location_summaries = locations.into_iter().map(|l| l.into()).collect();

    let total_pages = location_repo
        .get_total_pages(request.page_size)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            )
        })?;

    let paged_response = PagedResponse {
        current_page: request.page,
        total_pages,
        data: location_summaries,
    };

    Ok(Json(paged_response))
}
