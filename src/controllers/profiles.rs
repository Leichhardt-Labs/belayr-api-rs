use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use uuid::Uuid;

use crate::{
    models::{database_models::Session, profile_models::ProfileDetailsResponse},
    repositories::{profile_repo::ProfileRepo, session_repo::SessionRepo},
    util::{common::RepoError, logging::LoggingRouterExt},
};

#[derive(Clone)]
struct ProfileRouteState {
    profile_repo: ProfileRepo,
    session_repo: SessionRepo,
}

impl FromRef<ProfileRouteState> for ProfileRepo {
    fn from_ref(state: &ProfileRouteState) -> ProfileRepo {
        state.profile_repo.clone()
    }
}

impl FromRef<ProfileRouteState> for SessionRepo {
    fn from_ref(state: &ProfileRouteState) -> SessionRepo {
        state.session_repo.clone()
    }
}

pub fn profile_routes(profile_repo: ProfileRepo, session_repo: SessionRepo) -> Router {
    Router::new()
        .route("/profile/:id/details", get(get_profile))
        .route(
            "/profile/:id/sessions/subscribed",
            get(get_profile_sessions),
        )
        .with_state(ProfileRouteState {
            profile_repo,
            session_repo,
        })
        .add_logging()
}

pub async fn get_profile(
    Path(id): Path<Uuid>,
    State(profile_repo): State<ProfileRepo>,
) -> Result<Json<ProfileDetailsResponse>, (StatusCode, String)> {
    let profile = profile_repo
        .get_profile(id)
        .await
        .map_err(|err| match err {
            RepoError::NotFound => (StatusCode::NOT_FOUND, "Profile not found".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        })?;

    let user_disciplines =
        profile_repo
            .get_user_disciplines(id)
            .await
            .map_err(|err| match err {
                RepoError::NotFound => (StatusCode::NOT_FOUND, "Profile not found".into()),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                ),
            })?;

    Ok(Json((profile, user_disciplines).into()))
}

pub async fn get_profile_sessions(
    Path(id): Path<Uuid>,
    State(session_repo): State<SessionRepo>,
) -> Result<Json<Vec<Session>>, (StatusCode, String)> {
    let sessions = session_repo
        .get_sessions_by_profile_id(id)
        .await
        .map_err(|err| match err {
            RepoError::NotFound => (StatusCode::NOT_FOUND, "Profile not found".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        })?;

    Ok(Json(sessions))
}
