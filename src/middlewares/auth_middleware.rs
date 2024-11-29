use std::env;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::Error,
    middleware::Next,
    HttpMessage, HttpResponse,
};
use serde_json::json;

use crate::utils::jwt::decode_jwt;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                match decode_jwt(token, &env::var("SECRET_KEY").unwrap()) {
                    Ok(token_data) => {
                        req.extensions_mut().insert(token_data);
                        return next.call(req).await;
                    }
                    Err(_) => {
                        return Err(actix_web::error::InternalError::from_response(
                            "Invalid token",
                            HttpResponse::Unauthorized().json(json!({
                                "error": "Invalid token"
                            })),
                        )
                        .into());
                    }
                }
            }
        }
    }

    Err(actix_web::error::InternalError::from_response(
        "Missing token",
        HttpResponse::Unauthorized().json(json!({
            "message": "Error",
            "status": 401,
            "error": "Missing or invalid Authorization header"
        })),
    )
    .into())
}
