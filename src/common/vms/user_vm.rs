use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserVms {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
}
