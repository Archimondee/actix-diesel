use super::user_vm::UserVms;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginVms {
    pub id: String,
    pub token: String,
    pub username: String,
    pub user_info: UserVms,
    pub created_at: NaiveDateTime,
}
