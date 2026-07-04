use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Sliding-window rate limit policy.
#[derive(Debug, Clone, Copy)]
pub struct RateLimitPolicy {
    pub max: u32,
    pub window: Duration,
}

pub trait RateLimiter: Send + Sync {
    fn is_blocked(&self, key: &str, policy: RateLimitPolicy) -> bool;

    fn record_failure(&self, key: &str, policy: RateLimitPolicy);

    fn try_consume(&self, key: &str, policy: RateLimitPolicy) -> bool;
}

#[derive(Default)]
struct WindowStore {
    entries: HashMap<String, Vec<Instant>>,
}

#[derive(Default)]
pub struct InMemoryRateLimiter {
    store: Mutex<WindowStore>,
}

impl InMemoryRateLimiter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RateLimiter for InMemoryRateLimiter {
    fn is_blocked(&self, key: &str, policy: RateLimitPolicy) -> bool {
        count_in_window(&self.store, key, policy) >= policy.max
    }

    fn record_failure(&self, key: &str, policy: RateLimitPolicy) {
        push_event(&self.store, key, policy);
    }

    fn try_consume(&self, key: &str, policy: RateLimitPolicy) -> bool {
        if count_in_window(&self.store, key, policy) >= policy.max {
            return false;
        }
        push_event(&self.store, key, policy);
        true
    }
}

fn count_in_window(store: &Mutex<WindowStore>, key: &str, policy: RateLimitPolicy) -> u32 {
    let Ok(mut guard) = store.lock() else {
        return policy.max;
    };
    prune(&mut guard.entries, key, policy.window);
    guard.entries.get(key).map(|v| v.len() as u32).unwrap_or(0)
}

fn push_event(store: &Mutex<WindowStore>, key: &str, policy: RateLimitPolicy) {
    let Ok(mut guard) = store.lock() else {
        return;
    };
    prune(&mut guard.entries, key, policy.window);
    guard
        .entries
        .entry(key.to_owned())
        .or_default()
        .push(Instant::now());
}

fn prune(entries: &mut HashMap<String, Vec<Instant>>, key: &str, window: Duration) {
    let cutoff = Instant::now() - window;
    if let Some(events) = entries.get_mut(key) {
        events.retain(|instant| *instant > cutoff);
        if events.is_empty() {
            entries.remove(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_failures_when_max_reached_then_blocked() {
        let limiter = InMemoryRateLimiter::new();
        let policy = RateLimitPolicy {
            max: 3,
            window: Duration::from_secs(60),
        };
        let key = "ratelimit:login:127.0.0.1";

        for _ in 0..3 {
            limiter.record_failure(key, policy);
        }
        assert!(limiter.is_blocked(key, policy));
    }

    #[test]
    fn given_requests_when_under_max_then_consume_succeeds() {
        let limiter = InMemoryRateLimiter::new();
        let policy = RateLimitPolicy {
            max: 2,
            window: Duration::from_secs(60),
        };
        let key = "ratelimit:verify:127.0.0.1";

        assert!(limiter.try_consume(key, policy));
        assert!(limiter.try_consume(key, policy));
        assert!(!limiter.try_consume(key, policy));
    }
}
