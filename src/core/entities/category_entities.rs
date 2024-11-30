use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::QueryableByName;
use diesel::Selectable;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::schema::schema::categories;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Selectable, QueryableByName)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl Category {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}
