use axum::http::StatusCode;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RepoError {
    NotFound,
    InternalError,
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
