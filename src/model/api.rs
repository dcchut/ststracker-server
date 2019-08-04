use libsts::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub cards: Vec<Card>,
    pub relics: Vec<String>,
}

impl GetRequest {
    pub fn new(cards: Vec<Card>, relics: Vec<String>) -> Self {
        Self { cards, relics }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAPIKeyResponse {
    pub key: String,
}

impl GetAPIKeyResponse {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub cards: Vec<Card>,
    pub relics: Vec<String>,
    pub key: String,
}
