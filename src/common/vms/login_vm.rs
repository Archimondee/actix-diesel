use super::user_vm::UserVms;
use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LoginVms {
    pub id: String,
    pub token: String,
    pub username: String,
    pub user_info: UserVms,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
}
