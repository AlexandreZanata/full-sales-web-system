use std::time::Duration;

use async_trait::async_trait;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use uuid::Uuid;

use crate::{RefreshTokenStore, SessionError};

const SESSION_USER_PREFIX: &str = "session:";
const SESSION_TOKEN_PREFIX: &str = "refresh:";

pub struct RedisRefreshTokenStore {
    client: ConnectionManager,
}

impl RedisRefreshTokenStore {
    pub async fn connect(redis_url: &str) -> Result<Self, SessionError> {
        let client = redis::Client::open(redis_url).map_err(|_| SessionError::StoreFailed)?;
        let manager = ConnectionManager::new(client)
            .await
            .map_err(|_| SessionError::StoreFailed)?;
        Ok(Self { client: manager })
    }

    fn user_key(user_id: Uuid) -> String {
        format!("{SESSION_USER_PREFIX}{user_id}")
    }

    fn token_key(token: &str) -> String {
        format!("{SESSION_TOKEN_PREFIX}{token}")
    }
}

#[async_trait]
impl RefreshTokenStore for RedisRefreshTokenStore {
    async fn store(&self, user_id: Uuid, token: &str, ttl: Duration) -> Result<(), SessionError> {
        let mut conn = self.client.clone();
        let user_key = Self::user_key(user_id);
        let token_key = Self::token_key(token);
        let secs = ttl.as_secs();

        if let Some(old_token) = conn
            .get::<_, Option<String>>(&user_key)
            .await
            .map_err(|_| SessionError::StoreFailed)?
        {
            let _: () = conn
                .del(Self::token_key(&old_token))
                .await
                .map_err(|_| SessionError::StoreFailed)?;
        }

        conn.set_ex::<_, _, ()>(&user_key, token, secs)
            .await
            .map_err(|_| SessionError::StoreFailed)?;
        conn.set_ex::<_, _, ()>(&token_key, user_id.to_string(), secs)
            .await
            .map_err(|_| SessionError::StoreFailed)
    }

    async fn resolve_user(&self, token: &str) -> Result<Uuid, SessionError> {
        let mut conn = self.client.clone();
        let raw: Option<String> = conn
            .get(Self::token_key(token))
            .await
            .map_err(|_| SessionError::StoreFailed)?;
        let raw = raw.ok_or(SessionError::NotFound)?;
        Uuid::parse_str(&raw).map_err(|_| SessionError::NotFound)
    }

    async fn revoke(&self, user_id: Uuid) -> Result<(), SessionError> {
        let mut conn = self.client.clone();
        let user_key = Self::user_key(user_id);
        if let Some(token) = conn
            .get::<_, Option<String>>(&user_key)
            .await
            .map_err(|_| SessionError::StoreFailed)?
        {
            let _: () = conn
                .del(Self::token_key(&token))
                .await
                .map_err(|_| SessionError::StoreFailed)?;
        }
        conn.del::<_, ()>(user_key)
            .await
            .map_err(|_| SessionError::StoreFailed)
    }
}
