use actix_web::{
    http::{Error, StatusCode},
    web, HttpResponse,
};
use validator::Validate;

use crate::{
    common::{dtos::create_category_dto::CreateCategoryDto, vms::category_vm::CategoryVms},
    core::aggregator::Aggregator,
    infrastructure::db::connection::DbPool,
    utils::{
        check_connection::check_connection,
        response::{create_response, ApiError, ApiResponse},
        status_response::status_response,
        validation_errors::format_validation_errors,
    },
};

#[utoipa::path(
    post,
    path = "/category/create",
    request_body = CreateCategoryDto,
    responses(
        (status = 200, description = "Success", body = ApiResponse<CategoryVms>),
        (status = 401, description = "Unauthorized error", body = ApiResponse<ApiError>),
        (status = 422, description = "Validation error", body = ApiResponse<ApiError>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Category",
    security(
        ("Token" = []),
    )
)]
pub async fn create_category(
    payload: web::Json<CreateCategoryDto>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    match payload.validate() {
        Ok(()) => (),
        Err(e) => {
            let response: ApiResponse<()> = create_response(
                "Validation Error",
                StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                None,
                None,
                Some(format_validation_errors(&e)),
            );

            return Ok(HttpResponse::UnprocessableEntity().json(response));
        }
    }

    let mut conn = check_connection(pool);

    let category_dto = CreateCategoryDto {
        name: payload.name.clone(),
    };

    match category_dto.handle(&mut conn) {
        Ok(user) => {
            let response = create_response(
                "User registered successfully",
                StatusCode::CREATED.as_u16(),
                Some(user),
                None,
                None,
            );
            Ok(status_response(StatusCode::CREATED.as_u16(), response).unwrap())
        }
        Err(e) => {
            let response = create_response(&e.message, e.status, Some({}), None, e.error);
            Ok(status_response(e.status, response).unwrap())
        }
    }
}
