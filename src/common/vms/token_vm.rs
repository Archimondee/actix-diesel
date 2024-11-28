use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct Token {
    pub id: String,
    pub username: String,
    pub user_id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}
