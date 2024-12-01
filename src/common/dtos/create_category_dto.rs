use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, Clone, ToSchema)]
pub struct CreateCategoryDto {
    #[validate(length(min = 3, message = "Category name must be at least 3 character long"))]
    pub name: String,
}
