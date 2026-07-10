use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VelocityError {
    #[error("velocity counter store error")]
    StoreFailed,
}

#[async_trait]
pub trait VelocityCounter: Send + Sync {
    async fn increment(&self, key: &str, ttl: Duration) -> Result<u32, VelocityError>;
    async fn get(&self, key: &str) -> Result<u32, VelocityError>;
}

#[derive(Default)]
struct CounterStore {
    entries: HashMap<String, (Instant, u32)>,
}

#[derive(Default)]
pub struct InMemoryVelocityCounter {
    store: Mutex<CounterStore>,
}

impl InMemoryVelocityCounter {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl VelocityCounter for InMemoryVelocityCounter {
    async fn increment(&self, key: &str, ttl: Duration) -> Result<u32, VelocityError> {
        let mut guard = self
            .store
            .lock()
            .map_err(|_| VelocityError::StoreFailed)?;
        let now = Instant::now();
        let count = match guard.entries.get(key) {
            Some((expires, count)) if *expires > now => count + 1,
            _ => 1,
        };
        guard.entries.insert(key.to_owned(), (now + ttl, count));
        Ok(count)
    }

    async fn get(&self, key: &str) -> Result<u32, VelocityError> {
        let guard = self
            .store
            .lock()
            .map_err(|_| VelocityError::StoreFailed)?;
        let now = Instant::now();
        Ok(guard
            .entries
            .get(key)
            .filter(|(expires, _)| *expires > now)
            .map(|(_, count)| *count)
            .unwrap_or(0))
    }
}

pub struct RedisVelocityCounter {
    client: ConnectionManager,
}

impl RedisVelocityCounter {
    pub async fn connect(redis_url: &str) -> Result<Self, VelocityError> {
        let client = redis::Client::open(redis_url).map_err(|_| VelocityError::StoreFailed)?;
        let manager = ConnectionManager::new(client)
            .await
            .map_err(|_| VelocityError::StoreFailed)?;
        Ok(Self { client: manager })
    }
}

#[async_trait]
impl VelocityCounter for RedisVelocityCounter {
    async fn increment(&self, key: &str, ttl: Duration) -> Result<u32, VelocityError> {
        let mut conn = self.client.clone();
        let count: u32 = redis::cmd("INCR")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|_| VelocityError::StoreFailed)?;
        if count == 1 {
            conn.expire::<_, ()>(key, ttl.as_secs() as i64)
                .await
                .map_err(|_| VelocityError::StoreFailed)?;
        }
        Ok(count)
    }

    async fn get(&self, key: &str) -> Result<u32, VelocityError> {
        let mut conn = self.client.clone();
        let value: Option<u32> = conn.get(key).await.map_err(|_| VelocityError::StoreFailed)?;
        Ok(value.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn given_increments_when_within_ttl_then_count_grows() {
        let counter = InMemoryVelocityCounter::new();
        let ttl = Duration::from_secs(60);
        assert_eq!(counter.increment("fraud:test", ttl).await.expect("inc"), 1);
        assert_eq!(counter.increment("fraud:test", ttl).await.expect("inc"), 2);
        assert_eq!(counter.get("fraud:test").await.expect("get"), 2);
    }
}
