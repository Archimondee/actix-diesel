use actix_cors::Cors;

pub fn cors() -> Cors {
    Cors::default()
        .send_wildcard() // Allow requests from specific domain
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Allow specific HTTP methods
        .allowed_headers(vec!["Content-Type", "Authorization"]) // Allow specific headers
        .max_age(3600) // Cache the preflight request for 1 hour
        .supports_credentials() // Allow credentials like cookies or HTTP authentication
}
