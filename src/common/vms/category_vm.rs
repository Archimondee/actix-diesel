use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct CategoryVms {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}
