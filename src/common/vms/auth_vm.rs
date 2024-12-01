use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;
use utoipa::ToSchema;

use super::user_vm::UserVms;

#[derive(Serialize, Queryable, ToSchema)]
pub struct AuthVms {
    pub id: String,
    pub username: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    pub user_info: UserVms,
}
