use std::sync::Arc;

use axum::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::models::database_models::{Discipline, Profile};
use crate::schema::{profiles, user_disciplines};
use crate::util::common::{Pool, RepoError};

pub type ProfileRepo = Arc<dyn FetchesProfile + Send + Sync>;

pub struct ProfileRepository {
    pub pool: Pool,
}

#[async_trait]
pub trait FetchesProfile {
    /// Loop up a user by their id.
    async fn get_profile(&self, user_id: Uuid) -> Result<Profile, RepoError>;

    async fn get_user_disciplines(&self, user_id: Uuid) -> Result<Vec<Discipline>, RepoError>;
}

#[async_trait]
impl FetchesProfile for ProfileRepository {
    async fn get_profile(&self, profile_id: Uuid) -> Result<Profile, RepoError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|_| RepoError::InternalError)?;

        let profile = profiles::table
            .filter(profiles::id.eq(profile_id))
            .first::<Profile>(&mut conn)
            .await
            .map_err(|err| match err {
                diesel::result::Error::NotFound => RepoError::NotFound,
                _ => RepoError::InternalError,
            })?;

        Ok(profile)
    }

    async fn get_user_disciplines(&self, user_id: Uuid) -> Result<Vec<Discipline>, RepoError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|_| RepoError::InternalError)?;

        let disciplines = user_disciplines::table
            .filter(user_disciplines::user_id.eq(user_id))
            .select(user_disciplines::discipline)
            .load::<Discipline>(&mut conn)
            .await
            .map_err(|err| match err {
                diesel::result::Error::NotFound => RepoError::NotFound,
                _ => RepoError::InternalError,
            })?;

        Ok(disciplines)
    }
}
