use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::QueryableByName;
use diesel::Selectable;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::schema::schema::auths;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Selectable, QueryableByName)]
#[diesel(table_name = auths)]
pub struct Auth {
    pub id: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl Auth {
    pub fn new(username: &str, hashed_password: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            password: hashed_password.to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}
