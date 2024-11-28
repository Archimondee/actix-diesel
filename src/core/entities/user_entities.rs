use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::QueryableByName, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::schema::schema::users;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Selectable, QueryableByName)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub auth_id: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn new(auth_id: &str, firstname: &str, lastname: &str, email: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            auth_id: auth_id.to_string(),
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
            email: email.to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}
