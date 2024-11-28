use std::fmt;

use super::meta::Meta;
use diesel::result::Error as DieselError;
use serde::{self, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub message: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

#[allow(dead_code)]
pub fn create_response<T: Serialize>(
    message: &str,
    status: u16,
    data: Option<T>,
    meta: Option<Meta>,
    error: Option<Value>,
) -> ApiResponse<T> {
    ApiResponse {
        message: message.to_string(),
        status,
        data,
        meta,
        error,
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub error: Option<Value>,
    pub status: u16,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<DieselError> for ApiError {
    fn from(err: DieselError) -> Self {
        ApiError {
            message: "Database error".to_string(),
            error: Some(serde_json::json!({ "details": err.to_string() })),
            status: 500,
        }
    }
}

impl From<bcrypt::BcryptError> for ApiError {
    fn from(err: bcrypt::BcryptError) -> Self {
        ApiError {
            message: "Password hashing error".to_string(),
            error: Some(serde_json::json!({ "details": err.to_string() })),
            status: 500,
        }
    }
}

impl From<String> for ApiError {
    fn from(err: String) -> Self {
        ApiError {
            message: err,
            error: None,
            status: 500,
        }
    }
}
