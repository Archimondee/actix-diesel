use actix_web::{error::InternalError, http::StatusCode, web};
use r2d2::PooledConnection;

use crate::infrastructure::db::connection::DbPool;
use diesel::SqliteConnection;

#[allow(dead_code)]
pub fn check_connection(
    pool: web::Data<DbPool>,
) -> PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> {
    let conn = pool
        .get()
        .map_err(|_| {
            InternalError::new(
                "Failed to get DB connection",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
        .unwrap();

    conn
}
