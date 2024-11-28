use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

use super::user_vm::UserVms;

#[derive(Serialize, Queryable)]
pub struct AuthVms {
    pub id: String,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub user_info: UserVms,
}
