use axum::async_trait;

use crate::util::common::{DbConnection, Pool, RepoError};

#[async_trait]
pub trait DatabasePoolManager {
    async fn get_db_connection(&self) -> Result<DbConnection<'_>, RepoError> {
        self.get_pool()
            .get()
            .await
            .map_err(|_| RepoError::InternalError)
    }

    fn get_pool(&self) -> &Pool;
}
