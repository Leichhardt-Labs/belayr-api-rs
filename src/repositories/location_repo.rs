use std::sync::Arc;

use axum::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::models::database_models::ClimbLocation;
use crate::schema::climb_locations;
use crate::util::common::{Pool, RepoError};

pub type LocationRepo = Arc<dyn FetchesLocation + Send + Sync>;

pub struct LocationRepository {
    pub pool: Pool,
}

#[async_trait]
pub trait FetchesLocation {
    async fn get_location(&self, location_id: Uuid) -> Result<ClimbLocation, RepoError>;

    async fn get_paged_locations(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<ClimbLocation>, RepoError>;

    async fn get_total_pages(&self, page_size: i64) -> Result<i64, RepoError>;
}

#[async_trait]
impl FetchesLocation for LocationRepository {
    async fn get_location(&self, location_id: Uuid) -> Result<ClimbLocation, RepoError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|_| RepoError::InternalError)?;

        let location = climb_locations::table
            .filter(climb_locations::id.eq(location_id))
            .first::<ClimbLocation>(&mut conn)
            .await
            .map_err(|err| match err {
                diesel::result::Error::NotFound => RepoError::NotFound,
                _ => RepoError::InternalError,
            })?;

        Ok(location)
    }

    async fn get_paged_locations(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<ClimbLocation>, RepoError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|_| RepoError::InternalError)?;

        let locations = climb_locations::table
            .limit(page_size)
            .offset(page * page_size)
            .load::<ClimbLocation>(&mut conn)
            .await
            .map_err(|err| match err {
                diesel::result::Error::NotFound => RepoError::NotFound,
                _ => RepoError::InternalError,
            })?;

        Ok(locations)
    }

    async fn get_total_pages(&self, page_size: i64) -> Result<i64, RepoError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|_| RepoError::InternalError)?;

        let total = climb_locations::table
            .count()
            .first::<i64>(&mut conn)
            .await
            .map_err(|_| RepoError::InternalError)?;

        // Round up to the nearest page
        let total_pages = (total as f64 / page_size as f64).ceil() as i64;

        Ok(total_pages)
    }
}
