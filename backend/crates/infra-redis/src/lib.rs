use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("refresh token not found")]
    NotFound,

    #[error("session store error")]
    StoreFailed,
}

/// Opaque refresh token store — Redis in prod, in-memory for tests.
#[async_trait]
pub trait RefreshTokenStore: Send + Sync {
    async fn store(&self, user_id: Uuid, token: &str, ttl: Duration) -> Result<(), SessionError>;
    async fn resolve_user(&self, token: &str) -> Result<Uuid, SessionError>;
    async fn revoke(&self, user_id: Uuid) -> Result<(), SessionError>;
}

#[derive(Default)]
struct SessionData {
    by_user: HashMap<Uuid, String>,
    by_token: HashMap<String, Uuid>,
}

#[derive(Default)]
pub struct InMemoryRefreshTokenStore {
    sessions: Mutex<SessionData>,
}

impl InMemoryRefreshTokenStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl RefreshTokenStore for InMemoryRefreshTokenStore {
    async fn store(&self, user_id: Uuid, token: &str, _ttl: Duration) -> Result<(), SessionError> {
        let mut guard = self
            .sessions
            .lock()
            .map_err(|_| SessionError::StoreFailed)?;
        if let Some(old) = guard.by_user.insert(user_id, token.to_owned()) {
            guard.by_token.remove(&old);
        }
        guard.by_token.insert(token.to_owned(), user_id);
        Ok(())
    }

    async fn resolve_user(&self, token: &str) -> Result<Uuid, SessionError> {
        self.sessions
            .lock()
            .map_err(|_| SessionError::StoreFailed)?
            .by_token
            .get(token)
            .copied()
            .ok_or(SessionError::NotFound)
    }

    async fn revoke(&self, user_id: Uuid) -> Result<(), SessionError> {
        let mut guard = self
            .sessions
            .lock()
            .map_err(|_| SessionError::StoreFailed)?;
        if let Some(token) = guard.by_user.remove(&user_id) {
            guard.by_token.remove(&token);
        }
        Ok(())
    }
}

pub mod redis_store;

pub use redis_store::RedisRefreshTokenStore;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn given_token_when_store_resolve_revoke_then_lifecycle() {
        let store = InMemoryRefreshTokenStore::new();
        let user_id = Uuid::now_v7();
        store
            .store(user_id, "refresh-abc", Duration::from_secs(3600))
            .await
            .expect("store");
        assert_eq!(
            store.resolve_user("refresh-abc").await.expect("resolve"),
            user_id
        );
        store.revoke(user_id).await.expect("revoke");
        assert!(store.resolve_user("refresh-abc").await.is_err());
    }
}
