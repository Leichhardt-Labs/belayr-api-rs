use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Serialize;
use tracing::info;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Hello {
    message: String,
}

pub fn hello_routes() -> Router {
    Router::new().route("/hello/:name", get(handler))
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
