use actix_web::HttpRequest;
use frank_jwt::{decode, Algorithm};
use serde::{Deserialize, Serialize};

use crate::model::ApplicationState;

const BEARER_PREFIX: &str = "Bearer ";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitchJWTPayload {
    channel_id: String,
    exp: u64,
    opaque_user_id: String,
    role: String,
    user_id: String,
    pubsub_perms: TwitchJWTPermissions,
}

impl TwitchJWTPayload {
    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }

    pub fn role(&self) -> &str {
        &self.role
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitchJWTPermissions {
    listen: Vec<String>,
    send: Vec<String>,
}

pub fn verify_auth_header(
    state: &ApplicationState,
    request: &HttpRequest,
) -> Option<TwitchJWTPayload> {
    if let Some(header) = request.headers().get("authorization") {
        if let Ok(header) = header.to_str() {
            if header.starts_with(BEARER_PREFIX) {
                let token = &header[BEARER_PREFIX.len()..];

                // attempt to decode the authorization token
                if let Ok(decoded) = decode(
                    token,
                    &base64::decode(state.twitch_secret_key()).unwrap(),
                    Algorithm::HS256,
                ) {
                    if let Ok(payload) = serde_json::from_value(decoded.1) {
                        return Some(payload);
                    }
                }
            }
        }
    }

    None
}
