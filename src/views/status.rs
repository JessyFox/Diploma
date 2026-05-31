use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Status {
    #[schema(example = "Ok")]
    pub message: String,
}

impl Status {
    #[must_use]
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self {
            message: "Ok".to_string(),
        }
    }
}
