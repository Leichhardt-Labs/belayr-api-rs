use bb8::PooledConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type DbConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RepoError {
    NotFound,
    InternalError,
}
