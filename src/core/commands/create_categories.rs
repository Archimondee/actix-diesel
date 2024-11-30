use actix_web::http::StatusCode;
use diesel::{
    query_dsl::methods::FilterDsl, Connection, ExpressionMethods, OptionalExtension, RunQueryDsl,
};

use crate::{
    common::{dtos::create_category_dto::CreateCategoryDto, vms::category_vm::CategoryVms},
    core::{aggregator::Aggregator, entities::category_entities::Category},
    utils::{log_query, response::ApiError},
};

impl Aggregator<CategoryVms> for CreateCategoryDto {
    fn handle(
        &self,
        conn: &mut r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::SqliteConnection>,
        >,
    ) -> Result<Option<CategoryVms>, ApiError> {
        use crate::infrastructure::schema::schema::categories::dsl::*;

        conn.transaction(|txn_conn| {
            let query = categories.filter(name.eq(&self.name));
            let category_exists =
                log_query(query, || query.first::<Category>(txn_conn).optional())?;

            if category_exists.is_some() {
                return Err(ApiError {
                    message: "Category name already exists".to_string(),
                    status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    error: None,
                });
            }

            let category = Category::new(&self.name);
            let query = diesel::insert_into(categories).values(&category);
            log_query(query, || query.execute(txn_conn))?;

            let category = CategoryVms {
                id: category.id,
                name: category.name,
                created_at: category.created_at,
            };

            Ok(Some(category))
        })
    }
}
