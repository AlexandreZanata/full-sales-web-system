use std::collections::HashMap;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

type CacheMap = HashMap<String, (Instant, serde_json::Value)>;

pub struct SettlementCache {
    inner: Mutex<CacheMap>,
    ttl: Duration,
}

impl SettlementCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            ttl,
        }
    }

    pub async fn get(&self, key: &str) -> Option<serde_json::Value> {
        let guard = self.inner.lock().await;
        guard.get(key).and_then(|(at, value)| {
            if at.elapsed() < self.ttl {
                Some(value.clone())
            } else {
                None
            }
        })
    }

    pub async fn set(&self, key: String, value: serde_json::Value) {
        let mut guard = self.inner.lock().await;
        guard.insert(key, (Instant::now(), value));
    }
}
