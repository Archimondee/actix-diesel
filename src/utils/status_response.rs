use super::response::ApiResponse;
use actix_web::{http::Error, HttpResponse};
use serde::Serialize;

pub fn status_response<T: Serialize>(
    status: u16,
    response: ApiResponse<T>,
) -> Result<HttpResponse, Error> {
    match status {
        200 => Ok(HttpResponse::Ok().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        400 => Ok(HttpResponse::BadRequest().json(response)),
        404 => Ok(HttpResponse::NotFound().json(response)),
        422 => Ok(HttpResponse::UnprocessableEntity().json(response)),
        _ => Ok(HttpResponse::Ok().json(response)),
    }
}
