use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use uuid::Uuid;

use crate::{
    models::session_models::SessionDetailsResponse,
    repositories::{
        location_repo::LocationRepo, profile_repo::ProfileRepo, session_repo::SessionRepo,
    },
    util::{common::RepoError, logging::LoggingRouterExt},
};

#[derive(Clone)]
struct SessionRouteState {
    profile_repo: ProfileRepo,
    session_repo: SessionRepo,
    location_repo: LocationRepo,
}

impl FromRef<SessionRouteState> for ProfileRepo {
    fn from_ref(state: &SessionRouteState) -> ProfileRepo {
        state.profile_repo.clone()
    }
}

impl FromRef<SessionRouteState> for SessionRepo {
    fn from_ref(state: &SessionRouteState) -> SessionRepo {
        state.session_repo.clone()
    }
}

impl FromRef<SessionRouteState> for LocationRepo {
    fn from_ref(state: &SessionRouteState) -> LocationRepo {
        state.location_repo.clone()
    }
}

pub fn session_routes(
    profile_repo: ProfileRepo,
    session_repo: SessionRepo,
    location_repo: LocationRepo,
) -> Router {
    Router::new()
        .route("/session/:id/details", get(get_session))
        .with_state(SessionRouteState {
            profile_repo,
            session_repo,
            location_repo,
        })
        .add_logging()
}

async fn get_session(
    Path(session_id): Path<Uuid>,
    State(repos): State<SessionRouteState>,
) -> Result<Json<SessionDetailsResponse>, (StatusCode, String)> {
    let SessionRouteState {
        profile_repo,
        session_repo,
        location_repo,
    } = repos;

    let session = session_repo
        .get_session(session_id)
        .await
        .map_err(|err| match err {
            RepoError::NotFound => (StatusCode::NOT_FOUND, "Session not found".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        })?;

    let profile_ids = session_repo
        .get_session_participants(session_id)
        .await
        .map_err(|err| match err {
            RepoError::NotFound => (StatusCode::NOT_FOUND, "Session not found".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        })?;

    let profiles = profile_repo
        .get_profiles(profile_ids)
        .await
        .map_err(|err| match err {
            RepoError::NotFound => (StatusCode::NOT_FOUND, "Session not found".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        })?;

    let location = location_repo
        .get_location(session.location_id)
        .await
        .map_err(|err| match err {
            RepoError::NotFound => (StatusCode::NOT_FOUND, "Session not found".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        })?;

    Ok(Json((session, profiles, location).into()))
}
