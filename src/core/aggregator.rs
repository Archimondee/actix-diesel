use diesel::{r2d2::ConnectionManager, SqliteConnection};
use r2d2::PooledConnection;
use serde::Serialize;

use crate::utils::response::ApiError;

pub trait Aggregator<T>
where
    T: Serialize,
{
    fn handle(
        &self,
        conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    ) -> Result<Option<T>, ApiError>;
}
