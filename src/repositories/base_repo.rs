use axum::async_trait;
use bb8::PooledConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use crate::util::common::{Pool, RepoError};

#[async_trait]
pub trait DatabasePoolManager {
    async fn get_db_connection(
        &self,
    ) -> Result<PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>, RepoError>
    {
        self.get_pool()
            .get()
            .await
            .map_err(|_| RepoError::InternalError)
    }

    fn get_pool(&self) -> &Pool;
}
