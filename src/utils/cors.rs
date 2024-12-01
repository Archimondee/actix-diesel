use actix_cors::Cors;

pub fn cors() -> Cors {
    Cors::default()
        .send_wildcard()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec!["Content-Type", "Authorization"])
        .max_age(3600)
}
