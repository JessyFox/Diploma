use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub refresh: Option<Refresh>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Refresh {
    pub secret: String,
    #[serde(default = "refresh_expiration_default")]
    pub expiration: u64,
    #[serde(default = "refresh_name_default")]
    pub name: String,
}

const fn refresh_expiration_default() -> u64 {
    2_592_000
}

fn refresh_name_default() -> String {
    "refresh_token".to_string()
}

impl Settings {
    /// Parses settings part of config file
    ///
    /// # Errors
    /// Config structure is not valid
    pub fn from_json(value: &serde_json::Value) -> loco_rs::Result<Self> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
