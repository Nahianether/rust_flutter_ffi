use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ApiGetModel {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    #[serde(rename = "stackTrace")]
    pub stack_trace: Option<String>,
}
