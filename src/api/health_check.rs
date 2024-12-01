use actix_web::{HttpResponse, Responder};
use serde_json::json;

use crate::{
    common::{dtos::create_user_dto::CreateUserDto, vms::auth_vm::AuthVms},
    utils::response::{ApiError, ApiResponse},
};

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserDto,
    responses(
        (status = 201, description = "User registered successfully", body = ApiResponse<AuthVms>),
        (status = 422, description = "Validation error", body = ApiResponse<ApiError>),
        (status = 500, description = "Internal server error")
    ),
    tag = "User Management"
)]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
      "status": "Up",
      "code": 200,
      "message": "Service is runnning"
    }))
}
