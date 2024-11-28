use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::common::vms::token_vm::Token;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub username: String,
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub user_id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
}

impl Claims {
    pub fn new(login: &Token) -> Self {
        let issued_at = Utc::now().timestamp() as u64;
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Failed to set expiration")
            .timestamp() as u64;

        Claims {
            sub: login.id.to_owned(),
            exp: expiration,
            iat: issued_at,
            id: login.id.to_owned(),
            user_id: login.user_id.to_owned(),
            lastname: login.lastname.to_owned(),
            firstname: login.firstname.to_owned(),
            email: login.email.to_owned(),
            username: login.username.to_owned(),
        }
    }
}

#[allow(dead_code)]
pub fn create_jwt(login: &Token, secret: &str) -> String {
    let claims = Claims::new(login);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("JWT encoding failed");

    token
}

#[allow(dead_code)]
pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
