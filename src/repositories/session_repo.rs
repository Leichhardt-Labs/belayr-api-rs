use std::sync::Arc;

use axum::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::models::database_models::Session;
use crate::schema::{session_participants, sessions};
use crate::util::common::{Pool, RepoError};

pub type SessionRepo = Arc<dyn FetchesSession + Send + Sync>;

pub struct SessionRepository {
    pub pool: Pool,
}

#[async_trait]
pub trait FetchesSession {
    /// Loop up a user by their id.
    async fn get_sessions_by_profile_id(&self, user_id: Uuid) -> Result<Vec<Session>, RepoError>;
}

#[async_trait]
impl FetchesSession for SessionRepository {
    async fn get_sessions_by_profile_id(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<Session>, RepoError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|_| RepoError::InternalError)?;

        // Find out which sessions the user is subscribed to
        // Query the Session Participants table
        let session_ids: Vec<Uuid> = session_participants::table
            .filter(session_participants::user_id.eq(profile_id))
            .select(session_participants::session_id)
            .load::<Uuid>(&mut conn)
            .await
            .map_err(|err| match err {
                diesel::result::Error::NotFound => RepoError::NotFound,
                _ => RepoError::InternalError,
            })?;

        // Query the Sessions table
        let sessions: Vec<Session> = sessions::table
            .filter(sessions::id.eq_any(session_ids))
            .load::<Session>(&mut conn)
            .await
            .map_err(|err| match err {
                diesel::result::Error::NotFound => RepoError::NotFound,
                _ => RepoError::InternalError,
            })?;

        Ok(sessions)
    }
}
