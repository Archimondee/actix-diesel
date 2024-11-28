use chrono::NaiveDateTime;
use diesel::{Queryable, SqliteConnection};
use r2d2::PooledConnection;
use serde::Serialize;

use crate::utils::response::ApiError;

use super::user_vm::UserVms;

#[derive(Serialize, Queryable)]
pub struct AuthVms {
    pub id: String,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub user_info: UserVms,
}

pub trait AuthTrait {
    fn handle(
        &self,
        conn: &mut PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>,
    ) -> Result<Option<AuthVms>, ApiError>;
}
