use crate::{
    common::dtos::create_user_dto::CreateUserDto,
    infrastructure::db::connection::DbPool,
    utils::{
        check_connection::check_connection,
        response::{create_response, ApiResponse},
        status_response::status_response,
        validation_errors::format_validation_errors,
    },
};
use actix_web::{http::StatusCode, web, Error, HttpResponse};
use validator::Validate;

pub struct AuthService {}

impl AuthService {
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
}
