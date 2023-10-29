use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
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

async fn fetch_profile_by_id(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<Profile> {
    profiles::table
        .filter(profiles::id.eq(id))
        .first::<Profile>(conn)
        .await
}

async fn fetch_user_disciplines_by_profile_id(
    conn: &mut AsyncPgConnection,
    profile_id: Uuid,
) -> QueryResult<Vec<Discipline>> {
    profiles::table
        .inner_join(user_disciplines::table)
        .filter(profiles::id.eq(profile_id))
        .select(user_disciplines::discipline)
        .load::<Discipline>(conn)
        .await
}

pub async fn get_profile(
    Path(id): Path<Uuid>,
    State(pool): State<Pool>,
) -> Result<Json<ProfileDetailsResponse>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let profile = fetch_profile_by_id(&mut conn, id)
        .await
        .map_err(|err| match err {
            diesel::result::Error::NotFound => (StatusCode::NOT_FOUND, "Profile not found".into()),
            _ => internal_error(err),
        })?;

    let user_disciplines = fetch_user_disciplines_by_profile_id(&mut conn, id)
        .await
        .map_err(internal_error)?;

    Ok(Json((profile, user_disciplines).into()))
}
