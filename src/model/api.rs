use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAPIKeyResponse {
    pub key: String,
}

impl GetAPIKeyResponse {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
