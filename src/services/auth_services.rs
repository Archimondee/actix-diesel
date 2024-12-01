use crate::{
    common::{
        dtos::{create_user_dto::CreateUserDto, login_user_dto::LoginUserDto},
        vms::{auth_vm::AuthVms, login_vm::LoginVms},
    },
    core::aggregator::Aggregator,
    infrastructure::db::connection::DbPool,
    utils::{
        check_connection::check_connection,
        response::{create_response, ApiError, ApiResponse},
        status_response::status_response,
        validation_errors::format_validation_errors,
    },
};
use actix_web::{http::StatusCode, web, Error, HttpResponse};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = CreateUserDto,
    responses(
        (status = 201, description = "Success", body = ApiResponse<AuthVms>),
        (status = 422, description = "Validation error", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "Auth Management"
)]

pub async fn create_user(
    payload: web::Json<CreateUserDto>,
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

    let user_dto = CreateUserDto {
        username: payload.username.clone(),
        password: payload.password.clone(),
        firstname: payload.firstname.clone(),
        lastname: payload.lastname.clone(),
        email: payload.email.clone(),
    };

    match user_dto.handle(&mut conn) {
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

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginUserDto,
    responses(
        (status = 201, description = "Success", body = ApiResponse<LoginVms>),
        (status = 422, description = "Validation error", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "Auth Management"
)]
pub async fn login_user(
    payload: web::Json<LoginUserDto>,
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

    let user_dto = LoginUserDto {
        username: payload.username.clone(),
        password: payload.password.clone(),
    };

    match user_dto.handle(&mut conn) {
        Ok(user) => {
            let response =
                create_response("Success", StatusCode::OK.as_u16(), Some(user), None, None);
            Ok(status_response(StatusCode::OK.as_u16(), response).unwrap())
        }
        Err(e) => {
            let response = create_response(&e.message, e.status, Some({}), None, e.error);
            Ok(status_response(e.status, response).unwrap())
        }
    }
}
