use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone)]
pub struct CreateCategoryDto {
    #[validate(length(min = 3, message = "Category name must be at least 3 character long"))]
    pub name: String,
}
