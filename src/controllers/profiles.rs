use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    models::{
        database_models::{Discipline, Profile},
        profile_models::ProfileDetailsResponse,
    },
    schema::{profiles, user_disciplines},
    util::{
        common::{internal_error, Pool},
        logging::LoggingRouterExt,
    },
};

pub fn profile_routes(db_pool: Pool) -> Router {
    Router::new()
        .route("/profile/:id/details", get(get_profile))
        .with_state(db_pool)
        .add_logging()
}

pub async fn get_profile(
    Path(id): Path<Uuid>,
    State(pool): State<Pool>,
) -> Result<Json<ProfileDetailsResponse>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let profile = profiles::table
        .filter(profiles::id.eq(id))
        .first::<Profile>(&mut conn)
        .await
        .map_err(|err| match err {
            diesel::result::Error::NotFound => (StatusCode::NOT_FOUND, "Profile not found".into()),
            _ => internal_error(err),
        })?;

    let user_disciplines = profiles::table
        .inner_join(user_disciplines::table)
        .filter(profiles::id.eq(id))
        .select(user_disciplines::discipline)
        .load::<Discipline>(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(ProfileDetailsResponse {
        id: profile.id,
        username: profile.username,
        first_name: profile.first_name,
        last_name: profile.last_name,
        disciplines: user_disciplines,
    }))
}
