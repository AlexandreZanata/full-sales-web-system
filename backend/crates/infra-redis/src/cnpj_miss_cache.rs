use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use thiserror::Error;

pub const CNPJ_MISS_TTL: Duration = Duration::from_secs(24 * 60 * 60);
const MISS_VALUE: &str = "1";

#[derive(Debug, Error)]
pub enum MissCacheError {
    #[error("miss cache store error")]
    StoreFailed,
}

#[async_trait]
pub trait CnpjMissCache: Send + Sync {
    async fn is_negative(&self, cnpj: &str) -> bool;
    async fn record_negative(&self, cnpj: &str) -> Result<(), MissCacheError>;
}

pub fn miss_key(cnpj: &str) -> String {
    format!("cnpj-lookup:miss:{cnpj}")
}

struct MissEntry {
    expires_at: Instant,
}

#[derive(Default)]
pub struct InMemoryCnpjMissCache {
    entries: Mutex<HashMap<String, MissEntry>>,
}

impl InMemoryCnpjMissCache {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl CnpjMissCache for InMemoryCnpjMissCache {
    async fn is_negative(&self, cnpj: &str) -> bool {
        let Ok(mut guard) = self.entries.lock() else {
            return false;
        };
        let key = miss_key(cnpj);
        let Some(entry) = guard.get(&key) else {
            return false;
        };
        if entry.expires_at <= Instant::now() {
            guard.remove(&key);
            return false;
        }
        true
    }

    async fn record_negative(&self, cnpj: &str) -> Result<(), MissCacheError> {
        let mut guard = self
            .entries
            .lock()
            .map_err(|_| MissCacheError::StoreFailed)?;
        guard.insert(
            miss_key(cnpj),
            MissEntry {
                expires_at: Instant::now() + CNPJ_MISS_TTL,
            },
        );
        Ok(())
    }
}

pub struct RedisCnpjMissCache {
    client: ConnectionManager,
}

impl RedisCnpjMissCache {
    pub async fn connect(redis_url: &str) -> Result<Self, MissCacheError> {
        let client = redis::Client::open(redis_url).map_err(|_| MissCacheError::StoreFailed)?;
        let manager = ConnectionManager::new(client)
            .await
            .map_err(|_| MissCacheError::StoreFailed)?;
        Ok(Self { client: manager })
    }
}

#[async_trait]
impl CnpjMissCache for RedisCnpjMissCache {
    async fn is_negative(&self, cnpj: &str) -> bool {
        let mut conn = self.client.clone();
        conn.exists(miss_key(cnpj)).await.unwrap_or(false)
    }

    async fn record_negative(&self, cnpj: &str) -> Result<(), MissCacheError> {
        let mut conn = self.client.clone();
        conn.set_ex::<_, _, ()>(miss_key(cnpj), MISS_VALUE, CNPJ_MISS_TTL.as_secs())
            .await
            .map_err(|_| MissCacheError::StoreFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn given_miss_when_recorded_then_is_negative() {
        let cache = InMemoryCnpjMissCache::new();
        cache
            .record_negative("99999999000191")
            .await
            .expect("record");
        assert!(cache.is_negative("99999999000191").await);
        assert!(!cache.is_negative("00000000000191").await);
    }
}
