use redis::{Commands, Connection, RedisResult};

pub struct ApplicationState {
    _client_id: String,
    twitch_secret_key: String,
    backend_secret_key: &'static str,
    store: Connection,
}

impl ApplicationState {
    pub fn new(
        client_id: &str,
        twitch_secret_key: String,
        backend_secret_key: &'static str,
        store: Connection,
    ) -> Self {
        Self {
            _client_id: client_id.to_string(),
            twitch_secret_key,
            backend_secret_key,
            store,
        }
    }
}

impl ApplicationState {
    /// Stores a value in the application database.
    #[allow(clippy::let_unit_value)]
    pub fn set(&mut self, key: &str, value: &str) -> RedisResult<()> {
        let _ = self.store.set(key, value)?;
        Ok(())
    }

    /// Get a value from the application database.
    pub fn get(&mut self, key: &str) -> Result<String, redis::RedisError> {
        self.store.get(key)
    }

    /// The twitch secret key associated with the current state.
    pub fn twitch_secret_key(&self) -> &str {
        &self.twitch_secret_key
    }

    /// The backend secret key associated with the current state.
    pub fn backend_secret_key(&self) -> &'static str {
        &self.backend_secret_key
    }
}
