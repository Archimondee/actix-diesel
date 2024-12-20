use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, Clone, ToSchema)]
pub struct LoginUserDto {
    #[validate(length(min = 3, message = "Username must be at least 3 character"))]
    pub username: String,

    #[validate(length(min = 3, message = "Password must be at least 3 character"))]
    pub password: String,
}
