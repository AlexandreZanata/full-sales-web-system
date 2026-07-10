use redis::aio::ConnectionManager;

use crate::SessionError;

/// Pings Redis and returns round-trip latency in milliseconds.
pub async fn ping_redis(redis_url: &str) -> Result<u64, SessionError> {
    let client = redis::Client::open(redis_url).map_err(|_| SessionError::StoreFailed)?;
    let mut conn = ConnectionManager::new(client)
        .await
        .map_err(|_| SessionError::StoreFailed)?;
    let started = std::time::Instant::now();
    let _: String = redis::cmd("PING")
        .query_async(&mut conn)
        .await
        .map_err(|_| SessionError::StoreFailed)?;
    Ok(started.elapsed().as_millis() as u64)
}
