use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserVms {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
}
