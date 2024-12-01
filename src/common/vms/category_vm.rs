use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Queryable, ToSchema)]
pub struct CategoryVms {
    pub id: String,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
}
