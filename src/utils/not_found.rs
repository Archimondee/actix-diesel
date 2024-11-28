use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(json!({
        "status": "Not Found",
        "code": "404",
        "message": "Nothing to see here"
    }))
}
